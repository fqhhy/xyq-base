use std::fmt;

use crate::errors::{Result, TaError};
use crate::{High, Next, Period, Reset};

/// Returns the highest value in a given time frame.
///
/// # Parameters
///
/// * _period_ - size of the time frame (integer greater than 0). Default value is 14.
///
/// # Example
///
/// ```
/// use base::indicators::Maximum;
/// use base::Next;
///
/// let mut max = Maximum::new(3).unwrap();
/// assert_eq!(max.next(7.0), 7.0);
/// assert_eq!(max.next(5.0), 7.0);
/// assert_eq!(max.next(4.0), 7.0);
/// assert_eq!(max.next(4.0), 5.0);
/// assert_eq!(max.next(8.0), 8.0);
/// ```
#[derive(Debug, Clone)]
pub struct Maximum {
    period: usize,
    max_index: usize,
    cur_index: usize,
    deque: Box<[f64]>,
}

impl Maximum {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self { period, max_index: 0, cur_index: 0, deque: vec![f64::NEG_INFINITY; period].into_boxed_slice() }),
        }
    }

    fn find_max_index(&self) -> usize {
        let mut max = f64::NEG_INFINITY;
        let mut index: usize = 0;

        for (i, &val) in self.deque.iter().enumerate() {
            if val > max {
                max = val;
                index = i;
            }
        }

        index
    }
}

impl Period for Maximum {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<f64> for Maximum {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.deque[self.cur_index] = input;

        if input > self.deque[self.max_index] {
            self.max_index = self.cur_index;
        } else if self.max_index == self.cur_index {
            self.max_index = self.find_max_index();
        }

        self.cur_index = if self.cur_index + 1 < self.period { self.cur_index + 1 } else { 0 };

        self.deque[self.max_index]
    }
}

impl<T: High> Next<&T> for Maximum {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.high())
    }
}

impl Reset for Maximum {
    fn reset(&mut self) {
        for i in 0..self.period {
            self.deque[i] = f64::NEG_INFINITY;
        }
    }
}

impl Default for Maximum {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for Maximum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MAX({})", self.period)
    }
}
