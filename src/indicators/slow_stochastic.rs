use std::fmt;

use crate::errors::Result;
use crate::indicators::{ExponentialMovingAverage, FastStochastic};
use crate::{Close, High, Low, Next, Period, Reset};

/// Slow stochastic oscillator.
///
/// Basically it is a fast stochastic oscillator smoothed with exponential moving average.
///
/// # Parameters
///
/// * _stochastic_period_ - number of periods for fast stochastic (integer greater than 0). Default is 14.
/// * _ema_period_ - period for EMA (integer greater than 0). Default is 3.
///
/// # Example
///
/// ```
/// use base::indicators::SlowStochastic;
/// use base::Next;
///
/// let mut stoch = SlowStochastic::new(3, 2).unwrap();
/// assert_eq!(stoch.next(10.0), 50.0);
/// assert_eq!(stoch.next(50.0).round(), 83.0);
/// assert_eq!(stoch.next(50.0).round(), 94.0);
/// assert_eq!(stoch.next(30.0).round(), 31.0);
/// assert_eq!(stoch.next(55.0).round(), 77.0);
/// ```
#[derive(Clone, Debug)]
pub struct SlowStochastic {
    fast_stochastic: FastStochastic,
    ema: ExponentialMovingAverage,
}

impl SlowStochastic {
    pub fn new(stochastic_period: usize, ema_period: usize) -> Result<Self> {
        Ok(Self { fast_stochastic: FastStochastic::new(stochastic_period)?, ema: ExponentialMovingAverage::new(ema_period)? })
    }
}

impl Next<f64> for SlowStochastic {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.ema.next(self.fast_stochastic.next(input))
    }
}

impl<T: High + Low + Close> Next<&T> for SlowStochastic {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.ema.next(self.fast_stochastic.next(input))
    }
}

impl Reset for SlowStochastic {
    fn reset(&mut self) {
        self.fast_stochastic.reset();
        self.ema.reset();
    }
}

impl Default for SlowStochastic {
    fn default() -> Self {
        Self::new(14, 3).unwrap()
    }
}

impl fmt::Display for SlowStochastic {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SLOW_STOCH({}, {})", self.fast_stochastic.period(), self.ema.period())
    }
}
