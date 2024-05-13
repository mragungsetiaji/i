use crate::response::Response;
use std::{
    cell::{Cell},
    sync::mpsc::Sender,
};

pub struct Request {
    pub prompt: String,
    pub response: Sender<Response>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SequenceState {
    Done,
    Running,
    Waiting,
}

#[derive(Clone)]
pub struct Sequence {
    tokens: Vec<u32>,
    id: usize,
    state: Cell<SequenceState>,
}

impl Sequence {
    pub fn new_waiting(tokens: Vec<u32>, id: usize) -> Self {
        Self {
            tokens,
            id,
            state: Cell::new(SequenceState::Waiting),
        }
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn id(&self) -> &usize {
        &self.id
    }

    pub fn is_running(&self) -> bool {
        self.state.get() == SequenceState::Running
    }

    pub fn set_state(&self, state: SequenceState) {
        self.state.set(state)
    }

    pub fn get_tokens(&self) -> &[u32] {
        &self.tokens
    }
}
