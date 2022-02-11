mod draw;
mod events;
use crossterm::{event::KeyCode, terminal::disable_raw_mode};
use std::time::{Duration, Instant};
use crate::types;

pub fn start(batches: Vec<types::Batch>) -> Result<(), Box<dyn std::error::Error>> {
    let mut state = types::GobalState::new();
    for batch in batches{
        state.add_batch(batch);
    }
    let (_, rx) = events::spawn_event_thread();
    let mut terminal = draw::init_terminal()?;
    let tick_rate = Duration::from_millis(1000);
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| {
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
                state.add_seg();
            }
            draw::render(f, &state);
        })?;
        match rx.recv()? {
            events::Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                }
                _ => {}
            },
            events::Event::Tick => {}
        }
    }

    Ok(())
}
