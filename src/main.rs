use std::io::{self, IsTerminal, Read, Write};

use arboard::Clipboard;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame, Terminal,
};
use tui_textarea::TextArea;

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
}

impl<'a> App<'a> {
    fn new(initial_content: String) -> Self {
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
        }
    }

    fn get_content(&self) -> String {
        self.textarea.lines().join("\n")
    }

    fn handle_key_event(&mut self, key: KeyEvent) {
        if self.show_dialog {
            self.handle_dialog_key_event(key);
        } else {
            self.handle_editor_key_event(key);
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
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.show_dialog = true;
                self.selected_option = DialogOption::Cancel;
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
        .unwrap_or_default()
}

fn main() -> io::Result<()> {
    // Read input from STDIN if it's not a terminal (i.e., piped input)
    // Otherwise, read from clipboard
    let initial_content = if !io::stdin().is_terminal() {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else {
        get_clipboard_content()
    };

    // Set up terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new(initial_content);

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

    // Output to STDOUT if requested
    if app.exit_with_output {
        let content = app.get_content();
        io::stdout().write_all(content.as_bytes())?;
        io::stdout().write_all(b"\n")?;
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
