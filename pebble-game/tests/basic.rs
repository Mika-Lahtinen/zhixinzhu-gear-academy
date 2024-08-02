use gtest::{Program, System};
use pebble_game::*;
use pebble_game_io::*;

const USERS: &[u64] = &[3, 4, 5];

#[cfg(test)]
#[test]
fn test_init_should_work() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let res = program.send(
        USERS[0],
        PebblesInit {
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
        PebblesInit {
            pebbles_count: 10,
            max_pebbles_per_turn: 11,
            difficulty: DifficultyLevel::Easy,
        },
    );

    assert!(res.main_failed());
}

#[test]
fn test_with_positive_input() {
    let system = System::new();
    let program = Program::current(&system);
    let pid = program.id();
    let sender_id = 50;

    // Setting of the pebbles game
    let init_message = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 10,
        max_pebbles_per_turn: 4,
    };

    program.send(sender_id, init_message);

    // Check the initial state of the GameState
    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the initial state of the game");

    assert!(state.pebbles_remaining <= 10);
    assert_eq!(state.pebbles_count, 10);
    assert_eq!(state.max_pebbles_per_turn, 4);
    assert_eq!(state.difficulty, DifficultyLevel::Easy);
    assert_eq!(state.winner, None::<Player>);

    // Player's (User) turn
    program.send(sender_id, PebblesAction::Turn(3));

    // Check the state after user's turn
    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the state of the game after user's turn");

    assert!(state.pebbles_remaining < 7);

    // Player gives up
    program.send(sender_id, PebblesAction::GiveUp);

    // Check the state after player gives up
    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the state of the game after giving up");

    assert_eq!(state.winner, Some(Player::Program));

    // Restart the game
    let restart_message = PebblesAction::Restart {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 15,
        max_pebbles_per_turn: 10,
    };

    program.send(sender_id, restart_message);

    // Check the state after restart
    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the state of the game after restart");

    assert!(state.pebbles_remaining <= 15);
    assert_eq!(state.pebbles_count, 15);
    assert_eq!(state.max_pebbles_per_turn, 10);
    assert_eq!(state.difficulty, DifficultyLevel::Hard);
    assert_eq!(state.winner, None);
}

#[test]
fn test_with_negative_input() {
    let system = System::new();
    let program = Program::current(&system);
    let pid = program.id();
    let sender_id = 50;

    // Setting of the pebbles game
    let init_message = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count: 3,
        max_pebbles_per_turn: 4,
    };

    program.send(sender_id, init_message);

    // Check the initial state of the GameState
    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the initial state of the game");

    assert!(state.pebbles_remaining <= 10);
    assert_eq!(state.pebbles_count, 3);
    assert_eq!(state.max_pebbles_per_turn, 4);
    assert_eq!(state.difficulty, DifficultyLevel::Easy);
    assert_eq!(state.winner, None::<Player>);

    // Player's (User) turn
    program.send(sender_id, PebblesAction::Turn(1));
    //assert!(state.max_pebbles_per_turn <= -5);

    // Check the state after user's turn
    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the state of the game after user's turn");

    //assert!(state.pebbles_remaining < 0);
    if state.pebbles_remaining <= state.max_pebbles_per_turn {
        assert_eq!(state.winner, Some(Player::User));

        // Restart the game
        let restart_message = PebblesAction::Restart {
            difficulty: DifficultyLevel::Hard,
            pebbles_count: 15,
            max_pebbles_per_turn: 10,
        };

        program.send(sender_id, restart_message);

        // Check the state after restart
        let state: GameState = program
            .read_state(pid)
            .expect("Failed to get the state of the game after restart");

        assert!(state.pebbles_remaining <= 15);
        assert_eq!(state.pebbles_count, 15);
        assert_eq!(state.max_pebbles_per_turn, 10);
        assert_eq!(state.difficulty, DifficultyLevel::Hard);
        assert_eq!(state.winner, None);
    } else {
        // Player gives up
        program.send(sender_id, PebblesAction::GiveUp);

        // Check the state after player gives up
        let state: GameState = program
            .read_state(pid)
            .expect("Failed to get the state of the game after giving up");
        assert_eq!(state.winner, Some(Player::Program));
    }
}

#[test]
fn test_with_illegal_input() {
    let system = System::new();
    let program = Program::current(&system);
    let pid = program.id();
    let sender_id = 50;
    let pebbles_count = 10;

    // Setting of the pebbles game
    let init_message = PebblesInit {
        difficulty: DifficultyLevel::Easy,
        pebbles_count,
        max_pebbles_per_turn: 4,
    };

    program.send(sender_id, init_message);

    // Check the initial state of the GameState
    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the initial state of the game");

    assert!(state.pebbles_remaining <= 10);
    assert_eq!(state.pebbles_count, 10);
    assert_eq!(state.max_pebbles_per_turn, 4);
    assert_eq!(state.difficulty, DifficultyLevel::Easy);
    assert_eq!(state.winner, None::<Player>);

    // Player's (User) turn
    program.send(sender_id, PebblesAction::Turn(12));
    assert!(state.max_pebbles_per_turn <= 12);

    // Check the state after user's turn
    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the state of the game after user's turn");

    //assert!(state.pebbles_remaining < 0);
    if state.pebbles_remaining >= 0 {
        // Player gives up
        program.send(sender_id, PebblesAction::GiveUp);

        // Check the state after player gives up
        let state: GameState = program
            .read_state(pid)
            .expect("Failed to get the state of the game after giving up");

        assert_eq!(state.winner, Some(Player::Program));
    };

    // Restart the game
    let restart_message = PebblesAction::Restart {
        difficulty: DifficultyLevel::Hard,
        pebbles_count: 15,
        max_pebbles_per_turn: 10,
    };

    program.send(sender_id, restart_message);

    // Check the state after restart
    let state: GameState = program
        .read_state(pid)
        .expect("Failed to get the state of the game after restart");

    assert!(state.pebbles_remaining <= 15);
    assert_eq!(state.pebbles_count, 15);
    assert_eq!(state.max_pebbles_per_turn, 10);
    assert_eq!(state.difficulty, DifficultyLevel::Hard);
    assert_eq!(state.winner, None);
}
