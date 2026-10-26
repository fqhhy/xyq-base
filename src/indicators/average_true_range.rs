use std::fmt;

use crate::errors::Result;
use crate::indicators::{ExponentialMovingAverage, TrueRange};
use crate::{Close, High, Low, Next, Period, Reset};

/// Average true range (ATR).
///
/// A technical analysis volatility indicator, originally developed by J. Welles Wilder.
/// The average true range is an N-day smoothed moving average of the true range values.
/// This implementation uses exponential moving average.
///
/// # Formula
///
/// ATR(period)<sub>t</sub> = EMA(period) of TR<sub>t</sub>
///
/// Where:
///
/// * _EMA(period)_ - [exponential moving average](struct.ExponentialMovingAverage.html) with smoothing period
/// * _TR<sub>t</sub>_ - [true range](struct.TrueRange.html) for period _t_
///
/// # Parameters
///
/// * _period_ - smoothing period of EMA (integer greater than 0)
///
/// # Example
///
/// ```
/// #[macro_use] extern crate assert_approx_eq;
///
/// use base::Next;
/// use base::model::Bar;
/// use base::indicators::AverageTrueRange;
///
/// fn main() {
///     let data = vec![
///         // open, high, low, close, atr
///         (9.7   , 10.0, 9.0, 9.5  , 1.0),    // tr = high - low = 10.0 - 9.0 = 1.0
///         (9.9   , 10.4, 9.8, 10.2 , 0.95),   // tr = high - prev_close = 10.4 - 9.5 = 0.9
///         (10.1  , 10.7, 9.4, 9.7  , 1.125),  // tr = high - low = 10.7 - 9.4 = 1.3
///         (9.1   , 9.2 , 8.1, 8.4  , 1.3625), // tr = prev_close - low = 9.7 - 8.1 = 1.6
///     ];
///     let mut indicator = AverageTrueRange::new(3).unwrap();
///
///     for (open, high, low, close, atr) in data {
///        let di = Bar {high,low,close,open,volume:1000.0,..Default::default()};
///        assert_approx_eq!(indicator.next(&di), atr);
///     }
/// }
#[doc(alias = "ATR")]
#[derive(Debug, Clone)]
pub struct AverageTrueRange {
    true_range: TrueRange,
    ema: ExponentialMovingAverage,
}

impl AverageTrueRange {
    pub fn new(period: usize) -> Result<Self> {
        Ok(Self { true_range: TrueRange::new(), ema: ExponentialMovingAverage::new(period)? })
    }
}

impl Period for AverageTrueRange {
    fn period(&self) -> usize {
        self.ema.period()
    }
}

impl Next<f64> for AverageTrueRange {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        self.ema.next(self.true_range.next(input))
    }
}

impl<T: High + Low + Close> Next<&T> for AverageTrueRange {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        self.ema.next(self.true_range.next(input))
    }
}

impl Reset for AverageTrueRange {
    fn reset(&mut self) {
        self.true_range.reset();
        self.ema.reset();
    }
}

impl Default for AverageTrueRange {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for AverageTrueRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ATR({})", self.ema.period())
    }
}
