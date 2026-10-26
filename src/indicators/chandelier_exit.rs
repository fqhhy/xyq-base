use std::fmt;

use crate::errors::Result;
use crate::indicators::{AverageTrueRange, Maximum, Minimum};
use crate::{Close, High, Low, Next, Period, Reset};

/// Chandelier Exit (CE).
///
/// Developed by Charles Le Beau and featured in Alexander Elder's books, the Chandelier Exit sets
/// a trailing stop-loss based on the Average True Range (ATR). The indicator is designed to keep
/// traders in a trend and prevent an early exit as long as the trend extends. Typically, the
/// Chandelier Exit will be above prices during a downtrend and below prices during an uptrend.
///
/// # Formula
///
/// Chandelier Exit (long) = Max(_period_) - ATR(_period_) * _multipler_
/// Chandelier Exit (short) = Min(_period_) + ATR(_period_) * _multipler_
///
/// # Parameters
///
/// * _period_ - number of periods (integer greater than 0). Default is 22.
/// * _multipler_ - ATR factor. Default is 3.
///
/// # Example
///
/// ```
/// use base::indicators::ChandelierExit;
/// use base::Next;
/// use base::model::Bar;
///
///
/// let value1 = Bar{open:21.0,high:22.0,low:20.0,close:21.0,volume:1.0,..Default::default()};
/// let value2 = Bar{open:23.0,high:24.0,low:22.0,close:23.0,volume:1.0,..Default::default()};
///
/// let mut ce = ChandelierExit::default();
///
/// let first = ce.next(&value1);
/// assert_eq!(first.long, 16.0);
/// assert_eq!(first.short, 26.0);
///
/// let second = ce.next(&value2);
/// assert_eq!((second.long * 100.0).round() / 100.0, 17.74);
/// assert_eq!((second.short * 100.0).round() / 100.0, 26.26);
/// ```
///
/// # Links
///
/// * [Chandelier Exit, StockCharts](https://school.stockcharts.com/doku.php?id=technical_indicators:chandelier_exit)
///
#[doc(alias = "CE")]
#[derive(Debug, Clone)]
pub struct ChandelierExit {
    atr: AverageTrueRange,
    min: Minimum,
    max: Maximum,
    multiplier: f64,
}

impl ChandelierExit {
    pub fn new(period: usize, multiplier: f64) -> Result<Self> {
        Ok(Self { atr: AverageTrueRange::new(period)?, min: Minimum::new(period)?, max: Maximum::new(period)?, multiplier })
    }

    pub fn multiplier(&self) -> f64 {
        self.multiplier
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChandelierExitOutput {
    pub long: f64,
    pub short: f64,
}

impl From<ChandelierExitOutput> for (f64, f64) {
    fn from(ce: ChandelierExitOutput) -> Self {
        (ce.long, ce.short)
    }
}

impl Period for ChandelierExit {
    fn period(&self) -> usize {
        self.atr.period()
    }
}

impl<T: Low + High + Close> Next<&T> for ChandelierExit {
    type Output = ChandelierExitOutput;

    fn next(&mut self, input: &T) -> Self::Output {
        let atr = self.atr.next(input) * self.multiplier;
        let min = self.min.next(input);
        let max = self.max.next(input);

        ChandelierExitOutput { long: max - atr, short: min + atr }
    }
}

impl Reset for ChandelierExit {
    fn reset(&mut self) {
        self.atr.reset();
        self.min.reset();
        self.max.reset();
    }
}

impl Default for ChandelierExit {
    fn default() -> Self {
        Self::new(22, 3.0).unwrap()
    }
}

impl fmt::Display for ChandelierExit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CE({}, {})", self.atr.period(), self.multiplier)
    }
}
