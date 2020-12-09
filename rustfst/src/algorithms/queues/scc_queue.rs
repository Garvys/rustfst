use crate::algorithms::{Queue, QueueType};
use crate::StateId;

#[derive(Debug)]
pub struct SccQueue {
    front: i32,
    back: i32,
    queues: Vec<Box<dyn Queue>>,
    sccs: Vec<StateId>,
}

static NO_STATE_ID: i32 = -1;

impl SccQueue {
    pub fn new(queues: Vec<Box<dyn Queue>>, sccs: Vec<StateId>) -> Self {
        Self {
            front: 0,
            back: NO_STATE_ID,
            queues,
            sccs,
        }
    }
}

impl Queue for SccQueue {
    fn head(&mut self) -> Option<StateId> {
        while self.front <= self.back && self.queues[self.front as usize].is_empty() {
            self.front += 1;
        }
        self.queues[self.front as usize].head()
    }

    fn enqueue(&mut self, state: StateId) {
        let u_state = state as usize;
        if self.front > self.back {
            self.front = self.sccs[u_state] as i32;
            self.back = self.sccs[u_state] as i32;
        } else if (self.sccs[u_state] as i32) > self.back {
            self.back = self.sccs[u_state] as i32;
        } else if (self.sccs[u_state] as i32) < self.front {
            self.front = self.sccs[u_state] as i32;
        }
        self.queues[self.sccs[u_state] as usize].enqueue(state);
    }

    fn dequeue(&mut self) {
        self.queues[self.front as usize].dequeue()
    }

    fn update(&mut self, state: StateId) {
        self.queues[self.sccs[state as usize] as usize].update(state)
    }

    fn is_empty(&self) -> bool {
        if self.front < self.back {
            false
        } else if self.front > self.back {
            true
        } else {
            self.queues[self.front as usize].is_empty()
        }
    }

    fn clear(&mut self) {
        for i in self.front..=self.back {
            let i = i as usize;
            self.queues[i].clear();
        }
        self.front = 0;
        self.back = NO_STATE_ID;
    }

    fn queue_type(&self) -> QueueType {
        QueueType::SccQueue
    }
}
