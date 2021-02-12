use embedded_time::duration::*;

pub enum State {
    Track,
    FastForward(bool),
    Hold,
}

pub struct EQTracker {
    waiting_time: Microseconds,
    state: State,
}

impl EQTracker {
    pub fn new(waiting_time: Microseconds) -> Self {
        EQTracker {
            waiting_time,
            state: State::Track,
        }
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    pub fn get_waiting_time(&self) -> Microseconds {
        self.waiting_time
    }

    pub fn set_waiting_time(&mut self, duration: Microseconds) {
        self.waiting_time = duration;
    }
}
