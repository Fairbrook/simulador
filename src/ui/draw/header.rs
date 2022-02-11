use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

fn get_layout(rect: Rect) -> Vec<Rect> {
    Layout::default()
        .margin(0)
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rect)
}

fn title(f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect) {
    let block = Block::default()
        .title(Span::styled(
            "Simulaodr de procesos",
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::NONE);
    let text = Spans::from(vec![Span::styled(
        "Autor: Kevin Martinez 219294382",
        Style::default().fg(Color::DarkGray),
    )]);
    f.render_widget(Paragraph::new(text).block(block), rect);
}

fn quit_label(f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect) {
    let block = Block::default().borders(Borders::NONE);
    let text = Spans::from(vec![
        Span::styled("Presione ", Style::default().fg(Color::DarkGray)),
        Span::styled("q", Style::default().fg(Color::LightBlue)),
        Span::styled(" para ", Style::default().fg(Color::DarkGray)),
        Span::styled("salir", Style::default().fg(Color::LightBlue)),
    ]);
    f.render_widget(
        Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Right),
        rect,
    );
}

pub fn render(f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect) {
    let layout = get_layout(rect);
    title(f, layout[0]);
    quit_label(f, layout[1]);
}
