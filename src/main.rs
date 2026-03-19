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
    style::{Color, Style},
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DialogOption {
    Cancel,
    Abort,
    PipeOut,
}

struct App<'a> {
    textarea: TextArea<'a>,
    show_dialog: bool,
    selected_option: DialogOption,
    should_exit: bool,
    exit_with_output: bool,
    single_line_mode: bool,
}

impl<'a> App<'a> {
    fn new(initial_content: String, single_line_mode: bool) -> Self {
        let lines: Vec<String> = initial_content.lines().map(String::from).collect();
        let mut textarea = TextArea::new(lines);
        textarea.set_block(
            Block::default()
                .borders(Borders::ALL)
                .title("Editor (Alt+Enter: Exit and Pipe Output, Esc: Menu)"),
        );

        Self {
            textarea,
            show_dialog: false,
            selected_option: DialogOption::Cancel,
            should_exit: false,
            exit_with_output: false,
            single_line_mode,
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

    fn handle_key_event(&mut self, key: KeyEvent) {
        if self.show_dialog {
            self.handle_dialog_key_event(key);
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

    fn handle_editor_key_event(&mut self, key: KeyEvent) {
        match key {
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::ALT,
                ..
            } => {
                self.should_exit = true;
                self.exit_with_output = true;
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
            // Some terminals send Ctrl-Backspace as Ctrl-H
            KeyEvent {
                code: KeyCode::Char('h'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
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
    let initial_content = if !io::stdin().is_terminal() {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else {
        get_clipboard_content()
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
    let mut app = App::new(initial_content, args.single_line);

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

    // Render the textarea
    f.render_widget(&app.textarea, size);

    // Render dialog if shown
    if app.show_dialog {
        render_dialog(f, app);
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
