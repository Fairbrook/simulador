use crate::types::{
    global::State,
    process::{Batch, StatefulProcess},
    seconds_to_str,
};
use std::io;
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

fn get_layout(size: Rect) -> Vec<Rect> {
    Layout::default()
        .margin(0)
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ])
        .split(size)
}

fn queued_batch_number(f: &mut Frame<CrosstermBackend<io::Stdout>>, size: Rect, len: u32) {
    let block = Block::default().title("").borders(Borders::ALL);
    let paragraph = Paragraph::new(Spans::from(vec![
        Span::from(" Lotes restantes: "),
        Span::from(len.to_string()),
        Span::from(" "),
    ]))
    .block(block);
    f.render_widget(paragraph, size);
}

fn active_batch(
    f: &mut Frame<CrosstermBackend<io::Stdout>>,
    rect: Rect,
    batch: &Batch,
    index: usize,
) {
    let block = Block::default()
        .title(Spans::from(vec![
            Span::from(" Lote en ejecución #"),
            Span::from((index + 1).to_string()),
            Span::from(" "),
        ]))
        .borders(Borders::ALL);
    let mut text = vec![Spans::from(vec![Span::styled(
        "Procesos en cola: ",
        Style::default().fg(Color::DarkGray),
    )])];
    for stateful in batch.get_queued(true) {
        text.push(Spans::from(Span::styled(
            String::from("PID: ") + &stateful.process.pid,
            Style::default().add_modifier(Modifier::BOLD),
        )));
        text.push(Spans::from(
            String::from(" Estimado: ") + &stateful.process.et.to_string(),
        ));
        text.push(Spans::from(
            String::from(" Restante: ") + &(stateful.process.et - stateful.elapsed).to_string(),
        ));
    }
    f.render_widget(Paragraph::new(text).block(block), rect);
}

fn active_process(
    f: &mut Frame<CrosstermBackend<io::Stdout>>,
    rect: Rect,
    stateful_process: &StatefulProcess,
) {
    let info = Layout::default()
        .margin(0)
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(rect.inner(&Margin {
            horizontal: 2,
            vertical: 1,
        }));
    let times = Layout::default()
        .margin(0)
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(info[1]);
    let block = Block::default()
        .title(Spans::from(vec![
            Span::styled(" Proceso en ejecución: ", Style::default().fg(Color::Green)),
            Span::styled(
                &stateful_process.process.pid[..],
                Style::default().fg(Color::Green),
            ),
            Span::from(" "),
        ]))
        .borders(Borders::ALL);
    let text = vec![
        Spans::from(""),
        Spans::from(vec![
            Span::from("Número de programa: "),
            Span::from(&stateful_process.process.pid[..]),
        ]),
        Spans::from(vec![
            Span::from("Operacion: "),
            Span::from(stateful_process.process.operation.to_string()),
        ]),
        Spans::from(vec![
            Span::from("Tiempo máximo: "),
            Span::from(seconds_to_str(stateful_process.process.et)),
        ]),
    ];
    let safe_elapsed = if stateful_process.elapsed > stateful_process.process.et {
        stateful_process.process.et
    } else {
        stateful_process.elapsed
    };
    f.render_widget(block, rect);
    f.render_widget(Paragraph::new(text), info[0]);
    if stateful_process.error {
        f.render_widget(
            Paragraph::new(Spans::from(Span::styled(
                "ERROR",
                Style::default().fg(Color::LightRed),
            ))),
            info[2],
        );
        return;
    }
    if stateful_process.finished {
        f.render_widget(
            Paragraph::new(vec![
                Spans::from("Operación: "),
                Spans::from(Span::styled(
                    stateful_process.process.operation.to_string()
                        + " = "
                        + &stateful_process.result.to_string(),
                    Style::default().fg(Color::LightBlue),
                )),
            ]),
            info[1],
        );
        return;
    }
    f.render_widget(
        Paragraph::new(Spans::from(vec![
            Span::from("T: "),
            Span::from(seconds_to_str(safe_elapsed)),
        ])),
        times[0],
    );
    f.render_widget(
        Paragraph::new(Spans::from(vec![
            Span::from("Restante: "),
            Span::from(seconds_to_str(stateful_process.process.et - safe_elapsed)),
        ]))
        .alignment(Alignment::Right),
        times[1],
    );
    let percent: u16 = ((safe_elapsed as f64 / stateful_process.process.et as f64) * 100.0) as u16;
    let gauge = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(Color::Green)
                .bg(Color::Gray)
                .add_modifier(Modifier::ITALIC),
        )
        .percent(if percent > 100 { 100 } else { percent });
    f.render_widget(gauge, info[2]);
}

fn active(f: &mut Frame<CrosstermBackend<io::Stdout>>, size: Rect, state: &State) {
    let layout = Layout::default()
        .margin(0)
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Ratio(1, 1)])
        .split(size);
    let batch = state.get_active();
    let queued_batch = state.len() - state.active_index() as u32 - 1;
    queued_batch_number(f, layout[0], queued_batch);
    active_process(f, layout[1], batch.get_active());
}

fn finished_proc(text: &mut Vec<Spans>, stateful_process: &StatefulProcess) {
    text.push(Spans::from(vec![
        Span::styled("PID: ", Style::default().add_modifier(Modifier::BOLD)),
        Span::styled(
            stateful_process.process.pid.clone(),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ]));
    if stateful_process.error {
        text.push(Spans::from(vec![Span::styled(
            " ERROR",
            Style::default().fg(Color::LightRed),
        )]));
        return;
    }
    text.push(Spans::from(vec![
        Span::from(" Operacion: "),
        Span::from(stateful_process.process.operation.to_string()),
        Span::from(" = "),
        Span::from(stateful_process.result.to_string()),
    ]));
}

fn finished(f: &mut Frame<CrosstermBackend<io::Stdout>>, size: Rect, batches: &[Batch]) {
    let block = Block::default()
        .title("Procesos terminados")
        .borders(Borders::ALL);
    let mut text = Vec::new();
    for (i, b) in batches.iter().enumerate() {
        let list = b.get_finished();
        if list.len() == 0 {
            continue;
        }
        text.push(Spans::from(vec![Span::styled(
            String::from("Lote: ") + &(i + 1).to_string(),
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::UNDERLINED),
        )]));
        for p in list {
            finished_proc(&mut text, &p)
        }
        text.push(Spans::from(""));
    }
    f.render_widget(Paragraph::new(text).block(block), size);
}

pub fn render(f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect, state: &State) {
    let layout = get_layout(rect);
    active_batch(f, layout[0], state.get_active(), state.active_index());
    // batch_list(f, layout[0], state.get_queued(), state.active_index());
    active(f, layout[1], state);
    finished(f, layout[2], state.get_batches());
}
