use std::collections::{HashMap, HashSet};
use std::time::Duration;

pub struct PerformanceTracker {
    pub command_execution_times: HashMap<String, Vec<PerformanceResult>>,
}

impl PerformanceTracker {
    pub fn new() -> Self {
        PerformanceTracker {
            command_execution_times: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn record_command_execution(&mut self, command: &String, time: Duration, tag: String) {
        if let Some(array) = self.command_execution_times.get_mut(command) {
            array.push(PerformanceResult::from(time, tag));
        } else {
            self.command_execution_times
                .insert(command.clone(), vec![PerformanceResult::from(time, tag)]);
        }
    }

    #[allow(dead_code)]
    pub fn print_summary(&self, include: HashSet<String>) {
        for (command, results) in &self.command_execution_times {
            if include.get(command).is_none() {
                continue;
            }
            let mut sum: u128 = 0;
            for result in results {
                sum += result.time.as_nanos();
            }
            let average = Duration::from_nanos((sum / results.len() as u128) as u64);
            println!("command: {} ({})", command, results.len());
            println!(" average => {} ns", average.as_nanos());
        }
    }

    #[allow(dead_code)]
    pub fn print_full_report(&self, include: HashSet<String>) {
        for (command, results) in &self.command_execution_times {
            if include.get(command).is_none() {
                continue;
            }
            println!("command: {} ({})", command, results.len());
            for result in results {
                println!(" ({}) => {} ns", result.tag, result.time.as_nanos());
            }
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.command_execution_times.clear();
    }
}

pub struct PerformanceResult {
    pub time: Duration,
    pub tag: String,
}

impl PerformanceResult {
    pub fn from(time: Duration, tag: String) -> Self {
        PerformanceResult { time, tag }
    }
}
