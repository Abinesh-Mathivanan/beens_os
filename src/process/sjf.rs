#[derive(Debug, Clone)]
struct Process {
    id: usize,
    burst_time: u32,
    arrival_time: u32,
}

fn shortest_job_first(mut processes: Vec<Process>) {
    processes.sort_by_key(|p| (p.arrival_time, p.burst_time));

    let mut time_elapsed = 0;
    let mut completed = Vec::new();

    while !processes.is_empty() {
        let idx = processes.iter()
            .enumerate()
            .filter(|(_, p)| p.arrival_time <= time_elapsed)
            .min_by_key(|(_, p)| p.burst_time)
            .map(|(i, _)| i);

        if let Some(i) = idx {
            let process = processes.remove(i);
            println!("Time {}-{}: Process {} executes for {} units", 
                time_elapsed, time_elapsed + process.burst_time, process.id, process.burst_time);
            time_elapsed += process.burst_time;
            completed.push(process);
        } else {
            time_elapsed += 1;
        }
    }
}

fn main() {
    let processes = vec![
        Process { id: 1, burst_time: 6, arrival_time: 2 },
        Process { id: 2, burst_time: 8, arrival_time: 1 },
        Process { id: 3, burst_time: 7, arrival_time: 3 },
        Process { id: 4, burst_time: 3, arrival_time: 5 },
    ];

    println!("Shortest Job First Scheduling:");
    shortest_job_first(processes);
}
