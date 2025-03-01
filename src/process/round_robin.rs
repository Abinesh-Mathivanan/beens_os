use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Process {
    id: usize,
    burst_time: u32,
    remaining_time: u32,
}

pub fn round_robin_scheduling(mut processes: Vec<Process>, time_quantum: u32) {
    let mut queue: VecDeque<Process> = VecDeque::from(processes);
    let mut time_elapsed = 0;

    while !queue.is_empty() {
        let mut process = queue.pop_front().unwrap();

        let exec_time = time_quantum.min(process.remaining_time);
        process.remaining_time -= exec_time;
        time_elapsed += exec_time;

        println!(
            "Time {}-{}: Process {} executes for {} units",
            time_elapsed - exec_time,
            time_elapsed,
            process.id,
            exec_time
        );

        if process.remaining_time > 0 {
            queue.push_back(process);
        } else {
            println!("Process {} completed at time {}", process.id, time_elapsed);
        }
    }
}

pub fn main() {
    let processes = vec![
        Process { id: 1, burst_time: 10, remaining_time: 10 },
        Process { id: 2, burst_time: 5, remaining_time: 5 },
        Process { id: 3, burst_time: 8, remaining_time: 8 },
    ];

    let time_quantum = 3;

    println!("Round Robin Scheduling:");
    round_robin_scheduling(processes, time_quantum);
}
