#![no_std]

use core::panic;

use auxilar::get_init_pebbles_start;
use gstd::*;
use pebble_game_io::*;

pub mod auxilar;

static mut PEBBLE_GAME: Option<GameState> = None;

#[no_mangle]
extern "C" fn init() {
    let config: PebblesInit = msg::load().expect("Init Error.");

    if config.max_pebbles_per_turn > config.pebbles_count {
        panic!("Max pebbles per turn cannot be greater than pebbles count.");
    }

    let first_player = auxilar::check_first_player();

    let init_game = get_init_pebbles_start(
        config.difficulty,
        config.pebbles_count,
        config.max_pebbles_per_turn,
        first_player.clone(),
    );

    let game = GameState {
        pebbles_count: config.pebbles_count,
        max_pebbles_per_turn: config.max_pebbles_per_turn,
        pebbles_remaining: init_game,
        difficulty: config.difficulty,
        first_player,
        winner: None,
    };
    unsafe { PEBBLE_GAME = Some(game) };
}

#[no_mangle]
extern "C" fn handle() {
    unimplemented!();
}

#[no_mangle]
extern "C" fn state() {
    let game_state = unsafe { PEBBLE_GAME.clone().expect("Get state failed.") };

    msg::reply(game_state, 0).expect("State reply failed");
}
