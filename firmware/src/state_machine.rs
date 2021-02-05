use embedded_time::duration::*;

enum State {
    Track,
    FastForward{ forward_direction: bool },
    Halt,
}

pub struct EQTracker {
    waiting_time: Microseconds,
    state: State,
}

impl EQTracker {
    pub fn new(waiting_time: Microseconds) -> Self {
        Self {
            waiting_time,
            state: State::Track,
        }
    }

    pub fn run(&self) {
        match self.state {
            State::Track => track(self.waiting_time),
            State::Halt => halt(),
            State::FastForward{ forward_direction } => fast_forward(forward_direction),
        }
    }
}

fn track(waiting_time: Microseconds) {
    
}

fn halt() {

}

fn fast_forward(forward_direction: bool) {

}
