use std::fmt;

use crate::errors::{Result, TaError};
use crate::{Close, Next, Period, Reset};

/// Standard deviation (SD).
///
/// Returns the standard deviation of the last n values.
///
/// # Formula
///
/// ![SD formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/2845de27edc898d2a2a4320eda5f57e0dac6f650)
///
/// Where:
///
/// * _σ_ - value of standard deviation for N given probes.
/// * _N_ - number of probes in observation.
/// * _x<sub>i</sub>_ - i-th observed value from N elements observation.
///
/// # Parameters
///
/// * _period_ - number of periods (integer greater than 0)
///
/// # Example
///
/// ```
/// use base::indicators::StandardDeviation;
/// use base::Next;
///
/// let mut sd = StandardDeviation::new(3).unwrap();
/// assert_eq!(sd.next(10.0), 0.0);
/// assert_eq!(sd.next(20.0), 5.0);
/// ```
///
/// # Links
///
/// * [Standard Deviation, Wikipedia](https://en.wikipedia.org/wiki/Standard_deviation)
///
#[doc(alias = "SD")]
#[derive(Debug, Clone)]
pub struct StandardDeviation {
    period: usize,
    index: usize,
    count: usize,
    m: f64,
    m2: f64,
    deque: Box<[f64]>,
}

impl StandardDeviation {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self { period, index: 0, count: 0, m: 0.0, m2: 0.0, deque: vec![0.0; period].into_boxed_slice() }),
        }
    }

    pub(super) fn mean(&self) -> f64 {
        self.m
    }
}

impl Period for StandardDeviation {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<f64> for StandardDeviation {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let old_val = self.deque[self.index];
        self.deque[self.index] = input;

        self.index = if self.index + 1 < self.period { self.index + 1 } else { 0 };

        if self.count < self.period {
            self.count += 1;
            let delta = input - self.m;
            self.m += delta / self.count as f64;
            let delta2 = input - self.m;
            self.m2 += delta * delta2;
        } else {
            let delta = input - old_val;
            let old_m = self.m;
            self.m += delta / self.period as f64;
            let delta2 = input - self.m + old_val - old_m;
            self.m2 += delta * delta2;
        }
        if self.m2 < 0.0 {
            self.m2 = 0.0;
        }

        (self.m2 / self.count as f64).sqrt()
    }
}

impl<T: Close> Next<&T> for StandardDeviation {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.close())
    }
}

impl Reset for StandardDeviation {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        self.m = 0.0;
        self.m2 = 0.0;
        for i in 0..self.period {
            self.deque[i] = 0.0;
        }
    }
}

impl Default for StandardDeviation {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl fmt::Display for StandardDeviation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SD({})", self.period)
    }
}
