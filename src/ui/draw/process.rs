use crate::types::{
    global::State,
    process::{Batch, Process},
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

fn batch_list(
    f: &mut Frame<CrosstermBackend<io::Stdout>>,
    size: Rect,
    batches: &[Batch],
    active_index: usize,
) {
    let batches_len = if batches.len() > 0 {
        batches.len() - 1
    } else {
        0
    };
    let block = Block::default()
        .title(Spans::from(vec![
            Span::from(" Lotes restantes: "),
            Span::from(batches_len.to_string()),
            Span::from(" "),
        ]))
        .borders(Borders::ALL);

    let mut text = vec![Spans::from("")];
    for (index, batch) in batches.iter().enumerate() {
        let is_active = index == 0;
        let queued = batch.get_queued(is_active);
        if queued.len() == 0 {
            continue;
        }
        if is_active {
            continue;
        }

        text.push(Spans::from(vec![Span::styled(
            String::from("Lote #")
                + &(index + active_index).to_string()
                + if is_active { " (Activo)" } else { "" },
            Style::default()
                .fg(if is_active {
                    Color::LightBlue
                } else {
                    Color::DarkGray
                })
                .add_modifier(Modifier::UNDERLINED),
        )]));
        for p in queued {
            text.push(Spans::from(vec![
                Span::from("  • "),
                Span::from(&p.pid[..]),
            ]));
        }
        text.push(Spans::from(""));
    }
    let text = Paragraph::new(text).block(block);
    f.render_widget(text, size);
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
            Span::from(index.to_string()),
            Span::from(" "),
        ]))
        .borders(Borders::ALL);
    let mut process_list: Vec<Spans> = batch
        .get_processes()
        .iter()
        .map(|p| Spans::from(String::from("#") + &p.pid[..] + ": " + &seconds_to_str(p.et)))
        .collect();
    let mut text = vec![Spans::from(vec![
        Span::from("Tiempo estimado: "),
        Span::from(seconds_to_str(batch.estimated())),
    ])];
    text.append(&mut process_list);
    f.render_widget(Paragraph::new(text).block(block), rect);
}

fn active_process(
    f: &mut Frame<CrosstermBackend<io::Stdout>>,
    rect: Rect,
    process: &Process,
    elapsed: u32,
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
            Span::styled(&process.pid[..], Style::default().fg(Color::Green)),
            Span::from(" "),
        ]))
        .borders(Borders::ALL);
    let text = vec![
        Spans::from(""),
        Spans::from(vec![
            Span::from("Número de programa: "),
            Span::from(&process.pid[..]),
        ]),
        Spans::from(vec![
            Span::from("Programador: "),
            Span::from(&process.owner[..]),
        ]),
        Spans::from(vec![
            Span::from("Operacion: "),
            Span::from(process.operation.to_string()),
        ]),
        Spans::from(vec![
            Span::from("Tiempo máximo: "),
            Span::from(seconds_to_str(process.et)),
        ]),
    ];
    let safe_elapsed = if elapsed > process.et {
        process.et
    } else {
        elapsed
    };
    let percent: u16 = ((safe_elapsed as f64 / process.et as f64) * 100.0) as u16;
    let gauge = Gauge::default()
        .gauge_style(
            Style::default()
                .fg(Color::Green)
                .bg(Color::Gray)
                .add_modifier(Modifier::ITALIC),
        )
        .percent(if percent > 100 { 100 } else { percent });
    f.render_widget(block, rect);
    f.render_widget(Paragraph::new(text), info[0]);
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
            Span::from(seconds_to_str(process.et - safe_elapsed)),
        ]))
        .alignment(Alignment::Right),
        times[1],
    );
    f.render_widget(gauge, info[2]);
}

fn active(f: &mut Frame<CrosstermBackend<io::Stdout>>, size: Rect, batch: &Batch, index: usize) {
    let layout = Layout::default()
        .margin(0)
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(size);
    active_batch(f, layout[0], batch, index);
    active_process(f, layout[1], batch.get_active(), batch.delta());
}

fn finished_proc(text: &mut Vec<Spans>, proc: &Process) {
    text.push(Spans::from(vec![
        Span::styled("#", Style::default().add_modifier(Modifier::UNDERLINED)),
        Span::styled(
            proc.pid.clone(),
            Style::default().add_modifier(Modifier::UNDERLINED),
        ),
    ]));
    text.push(Spans::from(vec![
        Span::from(" Operacion: "),
        Span::from(proc.operation.to_string()),
        Span::from(" = "),
        Span::from(proc.operation.calc().to_string()),
    ]));
}

fn finished(
    f: &mut Frame<CrosstermBackend<io::Stdout>>,
    size: Rect,
    batch: &Batch,
    batches: &[Batch],
) {
    let block = Block::default()
        .title("Procesos terminados")
        .borders(Borders::ALL);
    let mut text = Vec::new();
    for b in batches {
        for p in b.get_processes() {
            finished_proc(&mut text, p)
        }
        text.push(Spans::from(""));
    }
    for p in batch.get_finished() {
        finished_proc(&mut text, p);
    }
    f.render_widget(Paragraph::new(text).block(block), size);
}

pub fn render(f: &mut Frame<CrosstermBackend<io::Stdout>>, rect: Rect, state: &State) {
    let layout = get_layout(rect);
    batch_list(f, layout[0], state.get_queued(), state.active_index());
    active(f, layout[1], state.get_active(), state.active_index());
    finished(f, layout[2], state.get_active(), state.get_finished());
}
