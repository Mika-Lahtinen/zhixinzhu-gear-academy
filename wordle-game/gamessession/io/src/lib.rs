#![no_std]

use gmeta::{In, Out, InOut, Metadata};
use gstd::{collections::HashMap, prelude::*, ActorId, MessageId};

pub struct GamesSessionMetadata;

impl Metadata for GamesSessionMetadata {
    type Init = In<GamesSessionInit>;
    type Handle = InOut<GamesSessionAction, GamesSessionResponse>;
    type State = Out<GamesSessionState>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct GamesSessionState {
    pub wordle_program_id: ActorId,
    pub game_sessions: Vec<(ActorId, SessionInfo)>,
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct GamesSessionInit {
    pub wordle_program_id: ActorId,
}

impl GamesSessionInit {
    pub fn assert_valid(&self) {
        assert!(
            !self.wordle_program_id.is_zero(),
            "Invalid wordle_program_id"
        );
    }
}

impl From<GamesSessionInit> for GamesSession {
    fn from(game_session_init: GamesSessionInit) -> Self {
        Self {
            wordle_program_id: game_session_init.wordle_program_id,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum GamesSessionAction {
    StartGame,
    CheckWord {
        word: String,
    },
    CheckGameStatus {
        user: ActorId,
        session_id: MessageId,
    },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum WordleAction {
    StartGame { user: ActorId },
    CheckWord { user: ActorId, word: String },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum GamesSessionResponse {
    StartSuccess,
    CheckWordResult {
        correct_positions: Vec<u8>,
        contained_in_word: Vec<u8>,
    },
    GameOver(GameStatus),
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum GameStatus {
    Win,
    Lose,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum WordleEvent {
    GameStarted {
        user: ActorId,
    },
    WordChecked {
        user: ActorId,
        correct_positions: Vec<u8>,
        contained_in_word: Vec<u8>,
    },
}

impl WordleEvent {
    pub fn get_user(&self) -> &ActorId {
        match self {
            WordleEvent::GameStarted { user } => user,
            WordleEvent::WordChecked { user, .. } => user,
        }
    }

    pub fn has_guessed(&self) -> bool {
        match self {
            WordleEvent::GameStarted { .. } => unimplemented!(),
            WordleEvent::WordChecked {
                correct_positions, ..
            } => correct_positions == &vec![0, 1, 2, 3, 4],
        }
    }
}

impl From<&WordleEvent> for GamesSessionResponse {
    fn from(wordle_event: &WordleEvent) -> Self {
        match wordle_event {
            WordleEvent::GameStarted { .. } => GamesSessionResponse::StartSuccess,
            WordleEvent::WordChecked {
                correct_positions,
                contained_in_word,
                ..
            } => GamesSessionResponse::CheckWordResult {
                correct_positions: correct_positions.clone(),
                contained_in_word: contained_in_word.clone(),
            },
        }
    }
}

#[derive(Default, Debug, Clone, Encode, Decode, TypeInfo)]
pub enum SessionStatus {
    #[default]
    Init,
    WaitUserInput,
    WaitWordleStartReply,
    WaitWordleCheckWordReply,
    ReplyReceived(WordleEvent),
    GameOver(GameStatus),
}

#[derive(Default, Debug, Clone, Encode, Decode, TypeInfo)]
pub struct SessionInfo {
    pub session_id: MessageId,
    pub original_msg_id: MessageId,
    pub send_to_wordle_msg_id: MessageId,
    pub tries: u8,
    pub session_status: SessionStatus,
}

impl SessionInfo {
    pub fn is_wait_reply_status(&self) -> bool {
        matches!(
            self.session_status,
            SessionStatus::WaitWordleCheckWordReply | SessionStatus::WaitWordleStartReply
        )
    }
}

#[derive(Default, Debug, Clone)]
pub struct GamesSession {
    pub wordle_program_id: ActorId,
    pub sessions: HashMap<ActorId, SessionInfo>,
}

impl From<&GamesSession> for GamesSessionState {
    fn from(game_session: &GamesSession) -> Self {
        Self {
            wordle_program_id: game_session.wordle_program_id,
            game_sessions: game_session
                .sessions
                .iter()
                .map(|(k, v)| (*k, v.clone()))
                .collect(),
        }
    }
}