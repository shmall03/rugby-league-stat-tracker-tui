mod app;
mod export;
mod input;
mod models;
mod ui;

use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyEventKind};

use app::App;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App::new();
    let tick_rate = Duration::from_millis(100);

    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        if event::poll(tick_rate)? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    app.handle_key(key);
                    if app.should_quit {
                        break;
                    }
                }
                _ => {}
            }
        } else {
            app.tick(0.0);
        }
    }

    if !app.state.team_a.is_empty() && !app.state.team_b.is_empty() {
        let _ = export::save_to_json(&app.state);
    }

    ratatui::restore();
    Ok(())
}
