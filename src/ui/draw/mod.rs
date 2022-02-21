mod footer;
mod header;
mod process;
use crate::types::global;
use crossterm::terminal::enable_raw_mode;
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    Frame, Terminal,
};

pub fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn std::error::Error>>
{
    enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    Ok(terminal)
}

fn get_layout(rect: Rect) -> Vec<Rect> {
    Layout::default()
        .margin(1)
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(80),
            Constraint::Percentage(10),
        ])
        .split(rect)
}

pub fn render(f: &mut Frame<CrosstermBackend<io::Stdout>>, state: &global::State) {
    let layout = get_layout(f.size());
    header::render(f, layout[0]);
    process::render(f, layout[1], state);
    footer::render(
        f,
        layout[2],
        state.estimated(),
        state.elapsed(),
        state.get_processes_len(),
        state.status(),
    );
}
