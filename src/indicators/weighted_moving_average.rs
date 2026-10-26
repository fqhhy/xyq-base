use std::fmt;

use crate::errors::{Result, TaError};
use crate::{Close, Next, Period, Reset};

/// Weighted moving average (WMA).
///
/// A moving average that assigns weights that decrease in arithmetical progression.
/// In an _n_-day WMA the latest day has weight _n_, the second latest _n−1_, etc., down to one.
///
/// # Formula
///
/// ![WMA formula](https://wikimedia.org/api/rest_v1/media/math/render/svg/7780333af18da7e27a1186a3d566e28da21b2840)
///
/// Where:
///
/// * _WMA<sub>M</sub>_ - is the value of the WMA at time _m_
/// * _n_ - is the period.
/// * _p<sub>M</sub>_ - is the input value at a time period t.
///
/// # Example
///
/// ```
/// use base::indicators::WeightedMovingAverage;
/// use base::Next;
///
/// let mut wma = WeightedMovingAverage::new(3).unwrap();
/// assert_eq!(wma.next(10.0), 10.0);
/// assert_eq!(wma.next(13.0), 12.0);
/// assert_eq!(wma.next(16.0), 14.0);
/// assert_eq!(wma.next(14.0), 14.5);
/// ```
///
/// # Links
///
/// * [Weighted moving average, Wikipedia](https://en.wikipedia.org/wiki/Moving_average#Weighted_moving_average)
///

#[doc(alias = "WMA")]
#[derive(Debug, Clone)]
pub struct WeightedMovingAverage {
    period: usize,
    index: usize,
    count: usize,
    weight: f64,
    sum: f64,
    sum_flat: f64,
    deque: Box<[f64]>,
}

impl WeightedMovingAverage {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self { period, index: 0, count: 0, weight: 0.0, sum: 0.0, sum_flat: 0.0, deque: vec![0.0; period].into_boxed_slice() }),
        }
    }
}

impl Period for WeightedMovingAverage {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<f64> for WeightedMovingAverage {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let old_val: f64 = self.deque[self.index];
        self.deque[self.index] = input;

        self.index = if self.index + 1 < self.period { self.index + 1 } else { 0 };

        if self.count < self.period {
            self.count += 1;
            self.weight = self.count as f64;
            self.sum += input * self.weight
        } else {
            self.sum = self.sum - self.sum_flat + (input * self.weight);
        }
        self.sum_flat = self.sum_flat - old_val + input;
        self.sum / (self.weight * (self.weight + 1.0) / 2.0)
    }
}

impl<T: Close> Next<&T> for WeightedMovingAverage {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.close())
    }
}

impl Reset for WeightedMovingAverage {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        self.weight = 0.0;
        self.sum = 0.0;
        self.sum_flat = 0.0;
        for i in 0..self.period {
            self.deque[i] = 0.0;
        }
    }
}

impl Default for WeightedMovingAverage {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl fmt::Display for WeightedMovingAverage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WMA({})", self.period)
    }
}
