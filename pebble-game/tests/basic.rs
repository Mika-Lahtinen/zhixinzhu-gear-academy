use gtest::{Program, System};
use pebble_game_io::*;
use pebble_game::*;

const USERS:&[u64] = &[3,4,5];

#[test]
fn test_init_should_work() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let res = program.send(
        USERS[0],
        PebblesInit{
            pebbles_count: 10,
            max_pebbles_per_turn: 3,
            difficulty: DifficultyLevel::Easy,
        },
    );

    assert!(!res.main_failed());
}


#[test]
fn test_init_should_failed() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let res = program.send(
        USERS[0],
        PebblesInit{
            pebbles_count: 10,
            max_pebbles_per_turn: 11,
            difficulty: DifficultyLevel::Easy,
        },
    );

    assert!(res.main_failed());
}