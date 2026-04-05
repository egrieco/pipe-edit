use std::io::{self, IsTerminal, Read, Write};

use arboard::Clipboard;
use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, Paragraph},
};
use tui_textarea::TextArea;

// Include build-time information
mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

fn get_version_string() -> String {
    let version = built_info::PKG_VERSION;

    let git_info = match (built_info::GIT_COMMIT_HASH_SHORT, built_info::GIT_DIRTY) {
        (Some(commit), Some(dirty)) => {
            let dirty_str = if dirty { "-dirty" } else { "" };
            format!(" (git: {}{})", commit, dirty_str)
        }
        (Some(commit), None) => format!(" (git: {})", commit),
        _ => String::new(),
    };

    format!("{}{}", version, git_info)
}

#[derive(Parser, Debug)]
#[command(name = "pipe-edit")]
#[command(about = "A TUI editor for manually modifying data piped from stdin or the clipboard.")]
#[command(version = None)] // Disable default version, we'll handle it ourselves
struct Args {
    /// Output to clipboard instead of stdout
    #[arg(short, long)]
    clipboard: bool,

    /// Join all lines into a single line, squeezing whitespace
    #[arg(short, long)]
    single_line: bool,

    /// Print version information
    #[arg(short = 'V', long = "version")]
    version: bool,

    /// Ignore stdin/clipboard and start with empty editor for user input.
    /// Optionally provide a custom header to display.
    #[arg(short, long, value_name = "HEADER")]
    question: Option<Option<String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DialogOption {
    Cancel,
    Abort,
    PipeOut,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Match {
    row: usize,
    col_start: usize,
    col_end: usize,
}

#[derive(Debug, Clone)]
struct SearchState {
    active: bool,
    query: String,
    matches: Vec<Match>,
    current_match_index: Option<usize>,
}

impl SearchState {
    fn new() -> Self {
        Self {
            active: false,
            query: String::new(),
            matches: Vec::new(),
            current_match_index: None,
        }
    }

    fn find_matches(&mut self, lines: &[String]) {
        self.matches.clear();
        self.current_match_index = None;

        if self.query.is_empty() {
            return;
        }

        for (row, line) in lines.iter().enumerate() {
            let mut start = 0;
            while let Some(pos) = line[start..].find(&self.query) {
                let col_start = start + pos;
                let col_end = col_start + self.query.len();
                self.matches.push(Match {
                    row,
                    col_start,
                    col_end,
                });
                start = col_start + 1;
            }
        }

        if !self.matches.is_empty() {
            self.current_match_index = Some(0);
        }
    }

    fn next_match(&mut self) {
        if self.matches.is_empty() {
            return;
        }
        self.current_match_index = Some(match self.current_match_index {
            Some(idx) => (idx + 1) % self.matches.len(),
            None => 0,
        });
    }

    fn prev_match(&mut self) {
        if self.matches.is_empty() {
            return;
        }
        self.current_match_index = Some(match self.current_match_index {
            Some(idx) => {
                if idx == 0 {
                    self.matches.len() - 1
                } else {
                    idx - 1
                }
            }
            None => self.matches.len() - 1,
        });
    }

    fn current_match(&self) -> Option<&Match> {
        self.current_match_index
            .and_then(|idx| self.matches.get(idx))
    }
}

struct App<'a> {
    textarea: TextArea<'a>,
    show_dialog: bool,
    show_help: bool,
    selected_option: DialogOption,
    should_exit: bool,
    exit_with_output: bool,
    single_line_mode: bool,
    search: SearchState,
}

impl<'a> App<'a> {
    fn new(initial_content: String, single_line_mode: bool, header: Option<String>) -> Self {
        let lines: Vec<String> = initial_content.lines().map(String::from).collect();
        let mut textarea = TextArea::new(lines);
        let title = header.unwrap_or_else(|| "Editor".to_string());
        textarea.set_block(Block::default().borders(Borders::ALL).title(title));

        Self {
            textarea,
            show_dialog: false,
            show_help: false,
            selected_option: DialogOption::Cancel,
            should_exit: false,
            exit_with_output: false,
            single_line_mode,
            search: SearchState::new(),
        }
    }

    fn get_content(&self) -> String {
        let content = self.textarea.lines().join("\n");

        if self.single_line_mode {
            // Join all lines and squeeze whitespace runs into single spaces
            let single_line: String = content.split_whitespace().collect::<Vec<&str>>().join(" ");
            single_line
        } else {
            content
        }
    }

    fn update_search_matches(&mut self) {
        let lines: Vec<String> = self
            .textarea
            .lines()
            .iter()
            .map(|s| s.to_string())
            .collect();
        self.search.find_matches(&lines);
        self.update_search_highlighting();
    }

    fn update_search_highlighting(&mut self) {
        // Clear any existing search style
        let _ = self.textarea.set_search_pattern("");

        if !self.search.query.is_empty() {
            // Use regex escaping for the search pattern
            let escaped = regex::escape(&self.search.query);
            let _ = self.textarea.set_search_pattern(&escaped);
            self.textarea.set_search_style(
                Style::default()
                    .bg(Color::Yellow)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            );
        }
    }

    fn jump_to_current_match(&mut self) {
        if let Some(m) = self.search.current_match().cloned() {
            // Move cursor to the match position
            let row = m.row;
            let col = m.col_start;

            // Move to the beginning first
            self.textarea.move_cursor(tui_textarea::CursorMove::Top);
            self.textarea.move_cursor(tui_textarea::CursorMove::Head);

            // Move down to the correct row
            for _ in 0..row {
                self.textarea.move_cursor(tui_textarea::CursorMove::Down);
            }

            // Move to the correct column
            for _ in 0..col {
                self.textarea.move_cursor(tui_textarea::CursorMove::Forward);
            }
        }
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        if self.show_dialog {
            self.handle_dialog_key_event(key);
        } else if self.show_help {
            self.handle_help_key_event(key);
        } else if self.search.active {
            self.handle_search_key_event(key);
        } else {
            self.handle_editor_key_event(key);
        }
    }

    fn join_lines(&mut self) {
        let (current_row, _) = self.textarea.cursor();
        let lines = self.textarea.lines();

        // Check if there's a line below to join with
        if current_row + 1 >= lines.len() {
            return;
        }

        let current_line = lines[current_row].to_string();
        let next_line = lines[current_row + 1].to_string();

        // Trim trailing whitespace from current line and leading whitespace from next line
        let current_trimmed = current_line.trim_end();
        let next_trimmed = next_line.trim_start();

        // Join with a single space (or no space if either part is empty)
        let joined = if current_trimmed.is_empty() {
            next_trimmed.to_string()
        } else if next_trimmed.is_empty() {
            current_trimmed.to_string()
        } else {
            format!("{} {}", current_trimmed, next_trimmed)
        };

        // Calculate the new cursor position (at the join point)
        let new_cursor_col = current_trimmed.len();

        // Select from start of current line to end of next line, then replace with joined content
        // First, move to start of current line
        self.textarea.move_cursor(tui_textarea::CursorMove::Head);

        // Start selection
        self.textarea.start_selection();

        // Move to next line
        self.textarea.move_cursor(tui_textarea::CursorMove::Down);

        // Move to end of that line
        self.textarea.move_cursor(tui_textarea::CursorMove::End);

        // Cut the selection (both lines)
        self.textarea.cut();

        // Insert the joined content
        self.textarea.insert_str(&joined);

        // Position cursor at the join point
        self.textarea.move_cursor(tui_textarea::CursorMove::Head);
        for _ in 0..new_cursor_col {
            self.textarea.move_cursor(tui_textarea::CursorMove::Forward);
        }
    }

    fn delete_to_start_of_buffer(&mut self) {
        // Select from current position to beginning of buffer, then delete
        self.textarea.start_selection();
        self.textarea.move_cursor(tui_textarea::CursorMove::Top);
        self.textarea.move_cursor(tui_textarea::CursorMove::Head);
        self.textarea.cut();
    }

    fn delete_to_end_of_buffer(&mut self) {
        // Select from current position to end of buffer, then delete
        self.textarea.start_selection();
        self.textarea.move_cursor(tui_textarea::CursorMove::Bottom);
        self.textarea.move_cursor(tui_textarea::CursorMove::End);
        self.textarea.cut();
    }

    fn handle_help_key_event(&mut self, key: KeyEvent) {
        // Any key closes the help screen
        match key.code {
            KeyCode::Esc
            | KeyCode::Enter
            | KeyCode::Char('q')
            | KeyCode::Char('h')
            | KeyCode::Char(' ') => {
                self.show_help = false;
            }
            _ => {
                self.show_help = false;
            }
        }
    }

    fn handle_search_key_event(&mut self, key: KeyEvent) {
        match key {
            KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                // Exit search mode
                self.search.active = false;
                self.search.query.clear();
                self.search.matches.clear();
                self.search.current_match_index = None;
                // Clear search highlighting
                let _ = self.textarea.set_search_pattern("");
            }
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => {
                // Jump to current match and exit search mode
                self.jump_to_current_match();
                self.search.active = false;
            }
            // Next match: Ctrl+F
            KeyEvent {
                code: KeyCode::Char('f'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                self.search.next_match();
                self.jump_to_current_match();
            }
            // Previous match: Ctrl+G
            KeyEvent {
                code: KeyCode::Char('g'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                self.search.prev_match();
                self.jump_to_current_match();
            }
            KeyEvent {
                code: KeyCode::Backspace,
                ..
            } => {
                self.search.query.pop();
                self.update_search_matches();
            }
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers,
                ..
            } if !modifiers.contains(KeyModifiers::CONTROL)
                && !modifiers.contains(KeyModifiers::ALT) =>
            {
                self.search.query.push(c);
                self.update_search_matches();
            }
            _ => {}
        }
    }

    fn handle_editor_key_event(&mut self, key: KeyEvent) {
        // TODO switch key code handling to the `keybinds` crate
        match key {
            // Search: Ctrl+F
            KeyEvent {
                code: KeyCode::Char('f'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                self.search.active = true;
                self.search.query.clear();
                self.search.matches.clear();
                self.search.current_match_index = None;
            }
            // Previous match: Ctrl+G (when not in search mode)
            KeyEvent {
                code: KeyCode::Char('g'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                if !self.search.matches.is_empty() {
                    self.search.prev_match();
                    self.jump_to_current_match();
                }
            }
            // Help: Ctrl+H
            KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                self.show_help = true;
            }
            // Exit with output: Alt+Enter
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::ALT,
                ..
            } => {
                self.should_exit = true;
                self.exit_with_output = true;
            }
            // Exit with output: Ctrl+Enter
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                self.should_exit = true;
                self.exit_with_output = true;
            }
            // Exit with output: Shift+Enter
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::SHIFT,
                ..
            } => {
                self.should_exit = true;
                self.exit_with_output = true;
            }
            // Exit with output: Ctrl+D
            KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                self.should_exit = true;
                self.exit_with_output = true;
            }
            // Exit without output: Ctrl+C
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                self.should_exit = true;
                self.exit_with_output = false;
            }
            // Exit without output: Ctrl+Q
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                self.should_exit = true;
                self.exit_with_output = false;
            }
            // Exit without output: Ctrl+W
            KeyEvent {
                code: KeyCode::Char('w'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                self.should_exit = true;
                self.exit_with_output = false;
            }
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            } if self.single_line_mode => {
                // In single-line mode, Enter exits and outputs
                self.should_exit = true;
                self.exit_with_output = true;
            }
            KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.show_dialog = true;
                self.selected_option = DialogOption::Cancel;
            }
            KeyEvent {
                code: KeyCode::Char('j'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                self.join_lines();
            }
            // Delete to beginning of buffer: Ctrl+Alt+U
            KeyEvent {
                code: KeyCode::Char('u'),
                modifiers,
                ..
            } if modifiers.contains(KeyModifiers::CONTROL)
                && modifiers.contains(KeyModifiers::ALT) =>
            {
                self.delete_to_start_of_buffer();
            }
            // Delete to beginning of buffer: Alt+<
            KeyEvent {
                code: KeyCode::Char('<'),
                modifiers: KeyModifiers::ALT,
                ..
            } => {
                self.delete_to_start_of_buffer();
            }
            // Delete to end of buffer: Ctrl+Alt+K
            KeyEvent {
                code: KeyCode::Char('k'),
                modifiers,
                ..
            } if modifiers.contains(KeyModifiers::CONTROL)
                && modifiers.contains(KeyModifiers::ALT) =>
            {
                self.delete_to_end_of_buffer();
            }
            // Delete to end of buffer: Alt+>
            KeyEvent {
                code: KeyCode::Char('>'),
                modifiers: KeyModifiers::ALT,
                ..
            } => {
                self.delete_to_end_of_buffer();
            }
            // Ctrl+Alt+Shift+Delete: delete from cursor to end of buffer (legacy)
            KeyEvent {
                code: KeyCode::Delete,
                modifiers,
                ..
            } if modifiers.contains(KeyModifiers::CONTROL)
                && modifiers.contains(KeyModifiers::ALT)
                && modifiers.contains(KeyModifiers::SHIFT) =>
            {
                self.delete_to_end_of_buffer();
            }
            // Ctrl-Delete or Shift-Delete: delete word to the right
            KeyEvent {
                code: KeyCode::Delete,
                modifiers,
                ..
            } if modifiers.contains(KeyModifiers::CONTROL)
                || modifiers.contains(KeyModifiers::SHIFT) =>
            {
                self.textarea.delete_next_word();
            }
            // Ctrl-Backspace or Shift-Backspace: delete word to the left
            // Some terminals send Ctrl-Backspace as Backspace with CONTROL modifier
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers,
                ..
            } if modifiers.contains(KeyModifiers::CONTROL)
                || modifiers.contains(KeyModifiers::SHIFT) =>
            {
                self.textarea.delete_word();
            }
            // Some terminals send Ctrl-Backspace as Char('\x7f') with CONTROL
            KeyEvent {
                code: KeyCode::Char('\x7f'),
                modifiers,
                ..
            } if modifiers.contains(KeyModifiers::CONTROL) => {
                self.textarea.delete_word();
            }
            // Some terminals send Ctrl-Backspace as Char('\x08') (backspace char)
            KeyEvent {
                code: KeyCode::Char('\x08'),
                modifiers,
                ..
            } if modifiers.contains(KeyModifiers::CONTROL) => {
                self.textarea.delete_word();
            }
            _ => {
                self.textarea.input(key);
            }
        }
    }

    fn handle_dialog_key_event(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.show_dialog = false;
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.selected_option = match self.selected_option {
                    DialogOption::Cancel => DialogOption::PipeOut,
                    DialogOption::Abort => DialogOption::Cancel,
                    DialogOption::PipeOut => DialogOption::Abort,
                };
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.selected_option = match self.selected_option {
                    DialogOption::Cancel => DialogOption::Abort,
                    DialogOption::Abort => DialogOption::PipeOut,
                    DialogOption::PipeOut => DialogOption::Cancel,
                };
            }
            KeyCode::Enter => match self.selected_option {
                DialogOption::Cancel => {
                    self.show_dialog = false;
                }
                DialogOption::Abort => {
                    self.should_exit = true;
                    self.exit_with_output = false;
                }
                DialogOption::PipeOut => {
                    self.should_exit = true;
                    self.exit_with_output = true;
                }
            },
            KeyCode::Char('c') | KeyCode::Char('C') => {
                self.show_dialog = false;
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                self.should_exit = true;
                self.exit_with_output = false;
            }
            KeyCode::Char('p') | KeyCode::Char('P') => {
                self.should_exit = true;
                self.exit_with_output = true;
            }
            _ => {}
        }
    }
}

fn get_clipboard_content() -> String {
    Clipboard::new()
        .and_then(|mut clipboard| clipboard.get_text())
        // TODO handle these errors more gracefully
        .unwrap()
}

fn set_clipboard_content(content: &str) -> Result<(), arboard::Error> {
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(content)?;
    Ok(())
}

/// Process input for single-line mode: join all lines and squeeze whitespace
fn process_single_line_input(input: &str) -> String {
    input.split_whitespace().collect::<Vec<&str>>().join(" ")
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Handle --version flag
    if args.version {
        println!("pipe-edit {}", get_version_string());
        return Ok(());
    }

    // Read input from STDIN if it's not a terminal (i.e., piped input)
    // Otherwise, read from clipboard
    let (initial_content, header) = if args.question.is_some() {
        // --question flag: ignore stdin/clipboard, start empty
        let header = args
            .question
            .clone()
            .unwrap()
            .unwrap_or_else(|| "Editor".to_string());
        (String::new(), Some(header))
    } else if !io::stdin().is_terminal() {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        (buffer, None)
    } else {
        (get_clipboard_content(), None)
    };

    // Process input for single-line mode if enabled
    let initial_content = if args.single_line {
        process_single_line_input(&initial_content)
    } else {
        initial_content
    };

    // Set up terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new(initial_content, args.single_line, header);

    // Main loop
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            app.handle_key_event(key);
        }

        if app.should_exit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    // Output if requested
    if app.exit_with_output {
        let content = app.get_content();
        if args.clipboard {
            // For clipboard, strip trailing CR/LF in single-line mode
            let output = if args.single_line {
                content.trim_end_matches(['\r', '\n'])
            } else {
                &content
            };
            set_clipboard_content(output).map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("Clipboard error: {}", e))
            })?;
        } else {
            if args.single_line {
                // In single-line mode, don't add trailing newline
                let output = content.trim_end_matches(['\r', '\n']);
                io::stdout().write_all(output.as_bytes())?;
            } else {
                io::stdout().write_all(content.as_bytes())?;
                io::stdout().write_all(b"\n")?;
            }
        }
        Ok(())
    } else {
        std::process::exit(1);
    }
}

fn ui(f: &mut Frame, app: &App) {
    let size = f.area();

    // Main layout: editor area and status bar at bottom
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(size);

    let editor_area = main_chunks[0];
    let status_area = main_chunks[1];

    if app.search.active {
        // When search is active, split the editor area to show search bar
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(editor_area);

        // Render the textarea in the main area
        f.render_widget(&app.textarea, chunks[0]);

        // Render the search bar
        render_search_bar(f, app, chunks[1]);
    } else {
        // Render the textarea in the editor area
        f.render_widget(&app.textarea, editor_area);
    }

    // Render status bar with instructions
    render_status_bar(f, status_area);

    // Render dialog if shown
    if app.show_dialog {
        render_dialog(f, app);
    }

    // Render help if shown
    if app.show_help {
        render_help(f);
    }
}

fn render_status_bar(f: &mut Frame, area: Rect) {
    let instructions = "Alt+Enter: Exit and Pipe Output | Esc: Menu | Ctrl+F: Search | Ctrl+H: Help";
    let status =
        Paragraph::new(instructions).style(Style::default().fg(Color::White).bg(Color::DarkGray));
    f.render_widget(status, area);
}

fn render_search_bar(f: &mut Frame, app: &App, area: Rect) {
    let match_info = if app.search.matches.is_empty() {
        if app.search.query.is_empty() {
            String::new()
        } else {
            " (no matches)".to_string()
        }
    } else {
        let current = app.search.current_match_index.map(|i| i + 1).unwrap_or(0);
        let total = app.search.matches.len();
        format!(" ({}/{})", current, total)
    };

    let title = format!(
        "Search{} (Enter: jump, Esc: cancel, Ctrl+F/Ctrl+G: next/prev)",
        match_info
    );

    let search_block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .style(Style::default().bg(Color::DarkGray));

    let inner_area = search_block.inner(area);
    f.render_widget(search_block, area);

    let search_text = Paragraph::new(app.search.query.as_str())
        .style(Style::default().fg(Color::White).bg(Color::DarkGray));

    f.render_widget(search_text, inner_area);

    // Show cursor in search bar
    let cursor_x = inner_area.x + app.search.query.len() as u16;
    let cursor_y = inner_area.y;
    if cursor_x < inner_area.x + inner_area.width {
        f.set_cursor_position((cursor_x, cursor_y));
    }
}

fn render_dialog(f: &mut Frame, app: &App) {
    let area = f.area();

    // Calculate dialog size and position
    let dialog_width = 50.min(area.width.saturating_sub(4));
    let dialog_height = 7;
    let dialog_x = (area.width.saturating_sub(dialog_width)) / 2;
    let dialog_y = (area.height.saturating_sub(dialog_height)) / 2;

    let dialog_area = Rect::new(dialog_x, dialog_y, dialog_width, dialog_height);

    // Clear the area behind the dialog
    f.render_widget(Clear, dialog_area);

    // Create dialog block
    let dialog_block = Block::default()
        .title("Exit Options")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));

    f.render_widget(dialog_block, dialog_area);

    // Create inner area for content
    let inner_area = Rect::new(
        dialog_area.x + 2,
        dialog_area.y + 2,
        dialog_area.width.saturating_sub(4),
        dialog_area.height.saturating_sub(4),
    );

    // Layout for buttons
    let button_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
            Constraint::Ratio(1, 3),
        ])
        .split(inner_area);

    // Render buttons
    let cancel_style = if app.selected_option == DialogOption::Cancel {
        Style::default().fg(Color::Black).bg(Color::White)
    } else {
        Style::default().fg(Color::White)
    };

    let abort_style = if app.selected_option == DialogOption::Abort {
        Style::default().fg(Color::Black).bg(Color::White)
    } else {
        Style::default().fg(Color::White)
    };

    let pipe_out_style = if app.selected_option == DialogOption::PipeOut {
        Style::default().fg(Color::Black).bg(Color::White)
    } else {
        Style::default().fg(Color::White)
    };

    let cancel_btn = Paragraph::new("[C]ancel").style(cancel_style);
    let abort_btn = Paragraph::new("[A]bort").style(abort_style);
    let pipe_out_btn = Paragraph::new("[P]ipe Out").style(pipe_out_style);

    f.render_widget(cancel_btn, button_layout[0]);
    f.render_widget(abort_btn, button_layout[1]);
    f.render_widget(pipe_out_btn, button_layout[2]);
}

fn render_help(f: &mut Frame) {
    let area = f.area();

    // Calculate help screen size and position
    let help_width = 60.min(area.width.saturating_sub(4));
    let help_height = 22.min(area.height.saturating_sub(4));
    let help_x = (area.width.saturating_sub(help_width)) / 2;
    let help_y = (area.height.saturating_sub(help_height)) / 2;

    let help_area = Rect::new(help_x, help_y, help_width, help_height);

    // Clear the area behind the help screen
    f.render_widget(Clear, help_area);

    // Create help content
    let help_text = vec![
        "",
        "  KEYBOARD SHORTCUTS",
        "  ──────────────────────────────────────────",
        "",
        "  Exit & Output:",
        "    Alt+Enter, Ctrl+Enter, Shift+Enter, Ctrl+D",
        "",
        "  Exit without Output:",
        "    Ctrl+C, Ctrl+Q, Ctrl+W",
        "",
        "  Navigation & Editing:",
        "    Ctrl+J          Join current line with next",
        "    Ctrl+Backspace  Delete word left",
        "    Ctrl+Delete     Delete word right",
        "    Ctrl+Alt+U, Alt+<        Delete to start of buffer",
        "    Ctrl+Alt+K, Alt+>        Delete to end of buffer",
        "",
        "  Search:",
        "    Ctrl+F          Open search / Next match",
        "    Ctrl+G          Previous match",
        "",
        "  Other:",
        "    Esc             Open exit menu",
        "    Ctrl+H          Show this help",
        "",
        "  Press any key to close this help screen",
    ];

    let help_paragraph = Paragraph::new(help_text.join("\n"))
        .block(
            Block::default()
                .title(" Help ")
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::DarkGray)),
        )
        .style(Style::default().fg(Color::White).bg(Color::DarkGray));

    f.render_widget(help_paragraph, help_area);
}
