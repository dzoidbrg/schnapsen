use crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::app::{App, Screen};

pub fn handle_input(app: &mut App) -> std::io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(false);
            }

            if key.code == KeyCode::Char('q') {
                app.should_quit = true;
                return Ok(true);
            }

            match app.screen {
                Screen::TrumpSelection => handle_trump_input(app, key.code),
                Screen::Bidding => handle_bidding_input(app, key.code),
                Screen::TalonExchange => handle_talon_input(app, key.code),
                Screen::Playing => handle_playing_input(app, key.code),
                Screen::TrickResult => handle_continue_input(app, key.code),
                Screen::DealResult => handle_deal_result_input(app, key.code),
                Screen::MatchResult => handle_match_result_input(app, key.code),
            }
        }
    }
    Ok(false)
}

fn handle_trump_input(app: &mut App, code: KeyCode) {
    match code {
        KeyCode::Left | KeyCode::Char('h') => {
            if app.selected_trump_index > 0 {
                app.selected_trump_index -= 1;
            } else {
                app.selected_trump_index = 3;
            }
        }
        KeyCode::Right | KeyCode::Char('l') => {
            app.selected_trump_index = (app.selected_trump_index + 1) % 4;
        }
        KeyCode::Enter => {
            app.select_trump();
        }
        _ => {}
    }
}

fn handle_bidding_input(app: &mut App, code: KeyCode) {
    let bid_count = app.available_bids().len().max(1);

    match code {
        KeyCode::Up | KeyCode::Char('k') => {
            if app.selected_bid_index > 0 {
                app.selected_bid_index -= 1;
            } else {
                app.selected_bid_index = bid_count - 1;
            }
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.selected_bid_index = (app.selected_bid_index + 1) % bid_count;
        }
        KeyCode::Enter => {
            let bids = app.available_bids();
            if !bids.is_empty() {
                let gt = bids[app.selected_bid_index % bids.len()];
                app.bid_game(gt);
            }
        }
        KeyCode::Char('p') => {
            app.bid_pass();
        }
        _ => {}
    }
}

fn handle_talon_input(app: &mut App, code: KeyCode) {
    let hand_len = app.human_hand().len().max(1);

    match code {
        KeyCode::Char('t') => {
            app.take_talon();
        }
        KeyCode::Left | KeyCode::Char('h') => {
            if app.selected_card_index > 0 {
                app.selected_card_index -= 1;
            } else {
                app.selected_card_index = hand_len - 1;
            }
        }
        KeyCode::Right | KeyCode::Char('l') => {
            app.selected_card_index = (app.selected_card_index + 1) % hand_len;
        }
        KeyCode::Char(' ') => {
            if app.talon_taken {
                let idx = app.selected_card_index % hand_len;
                app.toggle_discard(idx);
            }
        }
        KeyCode::Enter => {
            if app.talon_taken {
                app.confirm_discard();
            }
        }
        _ => {}
    }
}

fn handle_playing_input(app: &mut App, code: KeyCode) {
    let valid_count = app.valid_plays().len().max(1);

    match code {
        KeyCode::Left | KeyCode::Char('h') => {
            if app.selected_card_index > 0 {
                app.selected_card_index -= 1;
            } else {
                app.selected_card_index = valid_count - 1;
            }
        }
        KeyCode::Right | KeyCode::Char('l') => {
            app.selected_card_index = (app.selected_card_index + 1) % valid_count;
        }
        KeyCode::Enter => {
            app.play_selected_card();
        }
        _ => {}
    }
}

fn handle_continue_input(app: &mut App, code: KeyCode) {
    if code == KeyCode::Enter {
        app.screen = Screen::Playing;
        app.selected_card_index = 0;
    }
}

fn handle_deal_result_input(app: &mut App, code: KeyCode) {
    if code == KeyCode::Enter {
        app.next_deal();
    }
}

fn handle_match_result_input(app: &mut App, code: KeyCode) {
    if code == KeyCode::Enter {
        app.new_match();
    }
}
