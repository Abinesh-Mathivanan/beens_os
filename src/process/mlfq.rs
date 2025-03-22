extern crate alloc;
use alloc::collections::VecDeque;
use alloc::vec::Vec;

pub struct MLFQ {
    queues: Vec<VecDeque<usize>>,
    time_slices: Vec<u32>,
    current_queue: usize,
    max_priority: usize,
}

impl MLFQ {
    pub fn new(levels: usize, base_time_slice: u32) -> Self {
        let mut queues = Vec::with_capacity(levels);
        let mut time_slices = Vec::with_capacity(levels);
        
        for i in 0..levels {
            queues.push(VecDeque::new());
            time_slices.push(base_time_slice * (1 << i));
        }

        MLFQ {
            queues,
            time_slices,
            current_queue: 0,
            max_priority: levels - 1,
        }
    }

    pub fn enqueue(&mut self, process_id: usize, priority: usize) {
        let priority = priority.min(self.max_priority);
        self.queues[priority].push_back(process_id);
    }

    pub fn dequeue(&mut self) -> Option<(usize, u32)> {
        for i in 0..=self.max_priority {
            if !self.queues[i].is_empty() {
                self.current_queue = i;
                return Some((
                    self.queues[i].pop_front().unwrap(),
                    self.time_slices[i],
                ));
            }
        }
        None
    }

    pub fn demote(&mut self, process_id: usize) {
        let next_queue = (self.current_queue + 1).min(self.max_priority);
        self.queues[next_queue].push_back(process_id);
    }

    pub fn boost(&mut self) {
        for i in 1..=self.max_priority {
            while let Some(process_id) = self.queues[i].pop_front() {
                self.queues[0].push_back(process_id);
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.queues.iter().all(|queue| queue.is_empty())
    }
}