use std::fmt;

use crate::errors::{Result, TaError};
use crate::{Low, Next, Period, Reset};

/// Returns the lowest value in a given time frame.
///
/// # Parameters
///
/// * _period_ - size of the time frame (integer greater than 0). Default value is 14.
///
/// # Example
///
/// ```
/// use base::indicators::Minimum;
/// use base::Next;
///
/// let mut min = Minimum::new(3).unwrap();
/// assert_eq!(min.next(10.0), 10.0);
/// assert_eq!(min.next(11.0), 10.0);
/// assert_eq!(min.next(12.0), 10.0);
/// assert_eq!(min.next(13.0), 11.0);
/// ```
#[derive(Debug, Clone)]
pub struct Minimum {
    period: usize,
    min_index: usize,
    cur_index: usize,
    deque: Box<[f64]>,
}

impl Minimum {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self { period, min_index: 0, cur_index: 0, deque: vec![f64::INFINITY; period].into_boxed_slice() }),
        }
    }

    fn find_min_index(&self) -> usize {
        let mut min = f64::INFINITY;
        let mut index: usize = 0;

        for (i, &val) in self.deque.iter().enumerate() {
            if val < min {
                min = val;
                index = i;
            }
        }

        index
    }
}

impl Period for Minimum {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<f64> for Minimum {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.deque[self.cur_index] = input;

        if input < self.deque[self.min_index] {
            self.min_index = self.cur_index;
        } else if self.min_index == self.cur_index {
            self.min_index = self.find_min_index();
        }

        self.cur_index = if self.cur_index + 1 < self.period { self.cur_index + 1 } else { 0 };

        self.deque[self.min_index]
    }
}

impl<T: Low> Next<&T> for Minimum {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.low())
    }
}

impl Reset for Minimum {
    fn reset(&mut self) {
        for i in 0..self.period {
            self.deque[i] = f64::INFINITY;
        }
    }
}

impl Default for Minimum {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for Minimum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MIN({})", self.period)
    }
}
