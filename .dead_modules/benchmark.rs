//! Benchmark Framework
//!
//! Performance benchmarking for compiler operations.

use std::time::{Duration, Instant};

/// Benchmark result
#[derive(Debug, Clone)]
pub struct BenchResult {
    pub name: String,
    pub iterations: u32,
    pub total_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
}

impl BenchResult {
    pub fn avg_time(&self) -> Duration {
        self.total_time / self.iterations
    }

    pub fn ops_per_sec(&self) -> f64 {
        let avg_ns = self.avg_time().as_nanos() as f64;
        if avg_ns == 0.0 {
            0.0
        } else {
            1_000_000_000.0 / avg_ns
        }
    }
}

/// Simple benchmark runner
pub struct Bencher {
    iterations: u32,
}

impl Bencher {
    pub fn new(iterations: u32) -> Self {
        Self { iterations }
    }

    pub fn run<F>(&self, name: &str, mut f: F) -> BenchResult
    where
        F: FnMut(),
    {
        let mut total = Duration::ZERO;
        let mut min = Duration::MAX;
        let mut max = Duration::ZERO;

        for _ in 0..self.iterations {
            let start = Instant::now();
            f();
            let elapsed = start.elapsed();
            total += elapsed;
            min = min.min(elapsed);
            max = max.max(elapsed);
        }

        BenchResult {
            name: name.to_string(),
            iterations: self.iterations,
            total_time: total,
            min_time: min,
            max_time: max,
        }
    }
}
