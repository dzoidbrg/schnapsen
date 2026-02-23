//! Dreierschnapsen - Terminal UI
//!
//! A playable implementation of the three-player Schnapsen card game.

mod app;
mod ui;

use std::io;
use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app_result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(e) = app_result {
        eprintln!("Error: {e:?}");
    }
    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut app = App::new();
    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('n') => app.new_round(),
                        KeyCode::Left => app.prev_card(),
                        KeyCode::Right => app.next_card(),
                        KeyCode::Enter => app.play_selected(),
                        KeyCode::Char(c) if ('1'..='6').contains(&c) => {
                            let idx = (c as u8 - b'1') as usize;
                            app.select_card(idx);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}
