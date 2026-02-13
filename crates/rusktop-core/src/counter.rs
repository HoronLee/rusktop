#[derive(Debug, Clone)]
pub struct Counter {
    value: i32,
}

impl Counter {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn with_value(value: i32) -> Self {
        Self { value }
    }

    pub fn increment(&mut self) {
        self.value += 1;
    }

    pub fn decrement(&mut self) {
        self.value -= 1;
    }

    pub fn reset(&mut self) {
        self.value = 0;
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        let mut counter = Counter::new();
        counter.increment();
        assert_eq!(counter.value(), 1);
    }

    #[test]
    fn test_decrement() {
        let mut counter = Counter::new();
        counter.decrement();
        assert_eq!(counter.value(), -1);
    }

    #[test]
    fn test_reset() {
        let mut counter = Counter::with_value(10);
        counter.reset();
        assert_eq!(counter.value(), 0);
    }
}
