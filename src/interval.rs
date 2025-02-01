use crate::rtweekend::INFINITY;

#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

pub enum IntervalParameter {
    Default, // Default interval is empty
    Range { min: f64, max: f64 },
    EncloseInterval { a: Interval, b: Interval }, // Create the interval tightly enclosing the two input intervals.
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: INFINITY,
            max: -INFINITY,
        }
    }
}

impl Interval {
    pub fn new(param: IntervalParameter) -> Self {
        match param {
            IntervalParameter::Default => Default::default(),
            IntervalParameter::Range { min, max } => {
                Self { min, max }
            }
            IntervalParameter::EncloseInterval { a, b } => {
                let min = a.min.min(b.min);
                let max = a.max.max(b.max);
                Self { min, max }
            }
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}

pub const EMPTY: Interval = Interval { min: INFINITY, max: -INFINITY };
pub const UNIVERSE: Interval = Interval { min: -INFINITY, max: INFINITY };
