use std::fmt;

use crate::errors::{Result, TaError};
use crate::traits::{Close, Next, Period, Reset};

/// Rate of Change (ROC)
///
/// # Formula
///
/// ROC = (Price<sub>t</sub> - Price<sub>t-n</sub>) / Price<sub>t-n</sub> * 100
///
/// Where:
///
/// * ROC - current value of Rate of Change indicator
/// * P<sub>t</sub> - price at the moment
/// * P<sub>t-n</sub> - price _n_ periods ago
///
/// # Parameters
///
/// * _period_ - number of periods integer greater than 0
///
/// # Example
///
/// ```
/// use base::indicators::RateOfChange;
/// use base::Next;
///
/// let mut roc = RateOfChange::new(2).unwrap();
/// assert_eq!(roc.next(10.0), 0.0);            //  0
/// assert_eq!(roc.next(9.7).round(), -3.0);    //  (9.7 - 10) / 10  * 100 = -3
/// assert_eq!(roc.next(20.0).round(), 100.0);  //  (20 - 10)  / 10  * 100 = 100
/// assert_eq!(roc.next(20.0).round(), 106.0);  //  (20 - 9.7) / 9.7 * 100 = 106
/// ```
///
/// # Links
///
/// * [Rate of Change, Wikipedia](https://en.wikipedia.org/wiki/Momentum_(technical_analysis))
///
#[doc(alias = "ROC")]
#[derive(Debug, Clone)]
pub struct RateOfChange {
    period: usize,
    index: usize,
    count: usize,
    deque: Box<[f64]>,
}

impl RateOfChange {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self { period, index: 0, count: 0, deque: vec![0.0; period].into_boxed_slice() }),
        }
    }
}

impl Period for RateOfChange {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<f64> for RateOfChange {
    type Output = f64;

    fn next(&mut self, input: f64) -> f64 {
        let previous = if self.count > self.period {
            self.deque[self.index]
        } else {
            self.count += 1;
            if self.count == 1 { input } else { self.deque[0] }
        };
        self.deque[self.index] = input;

        self.index = if self.index + 1 < self.period { self.index + 1 } else { 0 };

        (input - previous) / previous * 100.0
    }
}

impl<T: Close> Next<&T> for RateOfChange {
    type Output = f64;

    fn next(&mut self, input: &T) -> f64 {
        self.next(input.close())
    }
}

impl Default for RateOfChange {
    fn default() -> Self {
        Self::new(9).unwrap()
    }
}

impl fmt::Display for RateOfChange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ROC({})", self.period)
    }
}

impl Reset for RateOfChange {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        for i in 0..self.period {
            self.deque[i] = 0.0;
        }
    }
}
