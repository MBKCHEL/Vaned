use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal,
};
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let result = run(&mut terminal);

    ratatui::restore();

    result
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1), 
            Constraint::Length(1), 
        ])
        .split(frame.area());

    let editor_area = Block::default().title(" vaned ").borders(Borders::ALL);

    let status_bar = Paragraph::new(" ^X Exit ");

    frame.render_widget(editor_area, chunks[0]);
    frame.render_widget(status_bar, chunks[1]);
})?;

        if let Event::Key(key) = event::read()? {
            if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('x') {
                break;
            }
        }


}

    Ok(())
}
