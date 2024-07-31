use gtest::{Program, System};
use pebble_game_io::*;

fn load_init() {
    let program = Program::current(&System::new());
    unsafe {
        PEBBLE_GAME = Some(program.state());
    }
}
#[test]
fn test() {
    
}