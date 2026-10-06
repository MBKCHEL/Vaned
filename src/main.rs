use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};
use std::io::{self, SeekFrom::Current};

struct App {
    lines: Vec<String>,
    cursor_x: usize,
    cursor_y: usize,
    filename: String,
}

impl App {
    fn new() -> Self {
        Self {
            lines: vec![String::new()],
            cursor_x: 0,
            cursor_y: 0,
            filename: "unknow".to_string(),
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new();

    let result = run(&mut terminal, &mut app);

    ratatui::restore();

    result
}

fn run(terminal: &mut DefaultTerminal, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Fill(1), Constraint::Length(1)])
                .split(frame.area());

            let editor_area = Paragraph::new(format!("{}", app.lines.join("\n")))
                .block(Block::default().title(" vaned ").borders(Borders::ALL));

            let status_bar = Paragraph::new(" ^X Exit ");

            frame.render_widget(editor_area, chunks[0]);
            frame.render_widget(status_bar, chunks[1]);

            frame.set_cursor_position(((app.cursor_x + 1) as u16, (app.cursor_y + 1) as u16));
        })?;

        if let Event::Key(key) = event::read()? {
            if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('x') {
                break Ok(());
            }
            match key.code {
                KeyCode::Char(c) => {
                    let current_line = &mut app.lines[app.cursor_y];
                    let byte_idx = current_line
                        .char_indices()
                        .nth(app.cursor_x)
                        .map(|(idx, _)| idx)
                        .unwrap_or(current_line.len());

                    current_line.insert(byte_idx, c);
                    app.cursor_x += 1;
                }

                KeyCode::Backspace => {
                    if app.cursor_x > 0 {
                        let current_line = &mut app.lines[app.cursor_y];

                        if let Some((byte_idx, _)) = current_line.char_indices().nth(app.cursor_x - 1) {
                            current_line.remove(byte_idx);
                            app.cursor_x -= 1;
                        }
                    } else if app.cursor_y > 0 {

                        let current_line = app.lines.remove(app.cursor_y);

                        app.cursor_y -= 1;

                        let prev_line_len = app.lines[app.cursor_y].chars().count();

                        app.lines[app.cursor_y].push_str(&current_line);

                        app.cursor_x = prev_line_len;
                    }
                }

                KeyCode::Enter => {
                    let current_line = &mut app.lines[app.cursor_y];

                    let byte_idx = current_line
                        .char_indices()
                        .nth(app.cursor_x)
                        .map(|(idx, _)| idx)
                        .unwrap_or(current_line.len());

                    let tail = current_line.split_off(byte_idx);

                    app.cursor_y += 1;
                    app.lines.insert(app.cursor_y, tail);
                    app.cursor_x = 0;
                }

                _ => {}
            }
        }
    }
}
