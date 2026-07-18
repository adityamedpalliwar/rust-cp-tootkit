use std::time::Instant;

/// A simple timer to measure execution duration.
pub struct Timer {
    start: Instant,
}

impl Timer {
    /// Starts a new timer.
    pub fn start() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    /// Returns the elapsed time in seconds as a floating point number.
    pub fn elapsed_secs(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_timer() {
        let timer = Timer::start();
        sleep(Duration::from_millis(50));
        let elapsed = timer.elapsed_secs();
        assert!(elapsed >= 0.045 && elapsed < 0.2); // relaxed upper bound for CI environments
    }
}
