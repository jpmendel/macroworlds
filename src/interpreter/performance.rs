use std::collections::{HashMap, HashSet};
use std::time::Duration;

pub struct PerformanceTracker {
    pub execution_times: HashMap<Box<str>, Vec<PerformanceResult>>,
}

impl PerformanceTracker {
    pub fn new() -> Self {
        PerformanceTracker {
            execution_times: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn record(&mut self, command: &str, time: Duration, tag: String) {
        let perf = PerformanceResult::new(time, tag);
        if let Some(array) = self.execution_times.get_mut(command) {
            array.push(perf);
        } else {
            self.execution_times.insert(Box::from(command), vec![perf]);
        }
    }

    #[allow(dead_code)]
    pub fn average(&self, command: &str) -> Duration {
        let Some(results) = self.execution_times.get(command) else {
            panic!("no times recorded for: {}", command);
        };
        let mut sum: u128 = 0;
        for result in results {
            sum += result.time.as_nanos();
        }
        Duration::from_nanos((sum / results.len() as u128) as u64)
    }

    #[allow(dead_code)]
    pub fn print_summary(&self, include: HashSet<&str>) {
        for (command, results) in &self.execution_times {
            if include.get(command.as_ref()).is_none() {
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
    pub fn print_full_report(&self, include: HashSet<&str>) {
        for (command, results) in &self.execution_times {
            if include.get(command.as_ref()).is_none() {
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
        self.execution_times.clear();
    }
}

pub struct PerformanceResult {
    pub time: Duration,
    pub tag: String,
}

impl PerformanceResult {
    pub fn new(time: Duration, tag: String) -> Self {
        PerformanceResult { time, tag }
    }
}
