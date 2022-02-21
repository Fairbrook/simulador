use crate::types::{global::States, seconds_to_str};
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

fn get_layout(size: Rect) -> Vec<Rect> {
    Layout::default()
        .margin(0)
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(size)
}

fn status(f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect, state: &States) {
    let block = Block::default().title("").borders(Borders::NONE);
    let text = match state {
        States::Processing => Spans::from(Span::styled(
            "PROCESANDO",
            Style::default().fg(Color::Green),
        )),
        States::Finished => {
            Spans::from(Span::styled("TERMINADO", Style::default().fg(Color::Gray)))
        }
        States::Paused => Spans::from(Span::styled(
            "PAUSADO",
            Style::default().fg(Color::LightMagenta),
        )),
    };
    f.render_widget(
        Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center),
        rect,
    )
}

fn number_of_process(f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect, number: u32) {
    let block = Block::default().title("").borders(Borders::NONE);
    let text = Spans::from(vec![
        Span::from("Procesos: "),
        Span::from(number.to_string()),
    ]);
    f.render_widget(
        Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center),
        rect,
    )
}

fn time(f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect, msg: &str, time: u32) {
    let block = Block::default().title("").borders(Borders::NONE);
    let text = Spans::from(vec![Span::from(msg), Span::from(seconds_to_str(time))]);
    f.render_widget(
        Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center),
        rect,
    )
}

fn elapsed_time(f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect, time: u32) {
    let block = Block::default().title("").borders(Borders::NONE);
    let text = Spans::from(vec![
        Span::styled("Transcurrido: ", Style::default().fg(Color::Green)),
        Span::styled(seconds_to_str(time), Style::default().fg(Color::Green)),
    ]);
    f.render_widget(
        Paragraph::new(text)
            .block(block)
            .alignment(Alignment::Center),
        rect,
    )
}

pub fn render(
    f: &mut Frame<CrosstermBackend<io::Stdout>>,
    rect: Rect,
    estimated: u32,
    elapsed: u32,
    processes_len: u32,
    state: &States,
) {
    let layout = get_layout(rect);
    let block = Block::default().borders(Borders::ALL);
    f.render_widget(block, rect);
    let safe_elapsed = if elapsed > estimated {
        estimated
    } else {
        elapsed
    };
    let dif = estimated - safe_elapsed;
    elapsed_time(f, layout[0], elapsed);
    time(f, layout[1], "Estimado: ", estimated);
    time(f, layout[2], "Restante: ", dif);
    number_of_process(f, layout[3], processes_len);
    status(f, layout[4], &state);
}
