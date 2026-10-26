use std::fmt;

use crate::errors::{Result, TaError};
use crate::traits::{Close, Next, Period, Reset};

/// Kaufman's Efficiency Ratio (ER).
///
/// It is calculated by dividing the price change over a period by the absolute sum of the price movements that occurred to achieve that change.
/// The resulting ratio ranges between 0.0 and 1.0 with higher values representing a more efficient or trending market.
///
/// # Parameters
///
/// * _period_ - number of periods (integer greater than 0)
///
/// # Example
///
/// ```
/// use base::indicators::EfficiencyRatio;
/// use base::Next;
///
/// let mut er = EfficiencyRatio::new(4).unwrap();
/// assert_eq!(er.next(10.0), 1.0);
/// assert_eq!(er.next(13.0), 1.0);
/// assert_eq!(er.next(12.0), 0.5);
/// assert_eq!(er.next(13.0), 0.6);
/// assert_eq!(er.next(18.0), 0.8);
/// assert_eq!(er.next(19.0), 0.75);
/// ```

#[doc(alias = "ER")]
#[derive(Debug, Clone)]
pub struct EfficiencyRatio {
    period: usize,
    index: usize,
    count: usize,
    deque: Box<[f64]>,
}

impl EfficiencyRatio {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self { period, index: 0, count: 0, deque: vec![0.0; period].into_boxed_slice() }),
        }
    }
}

impl Period for EfficiencyRatio {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<f64> for EfficiencyRatio {
    type Output = f64;

    fn next(&mut self, input: f64) -> f64 {
        let first = if self.count >= self.period {
            self.deque[self.index]
        } else {
            self.count += 1;
            self.deque[0]
        };
        self.deque[self.index] = input;

        self.index = if self.index + 1 < self.period { self.index + 1 } else { 0 };

        let mut volatility = 0.0;
        let mut previous = first;
        for n in &self.deque[self.index..self.count] {
            volatility += (previous - n).abs();
            previous = *n;
        }
        for n in &self.deque[0..self.index] {
            volatility += (previous - n).abs();
            previous = *n;
        }

        (first - input).abs() / volatility
    }
}

impl<T: Close> Next<&T> for EfficiencyRatio {
    type Output = f64;

    fn next(&mut self, input: &T) -> f64 {
        self.next(input.close())
    }
}

impl Reset for EfficiencyRatio {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        for i in 0..self.period {
            self.deque[i] = 0.0;
        }
    }
}

impl Default for EfficiencyRatio {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for EfficiencyRatio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ER({})", self.period)
    }
}
