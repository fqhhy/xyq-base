use std::fmt;

use crate::errors::Result;
use crate::indicators::{AverageTrueRange, ExponentialMovingAverage};
use crate::{Close, High, Low, Next, Period, Reset};

/// Keltner Channel (KC).
///
/// A Keltner Channel is an indicator showing the Average True Range (ATR) of a
/// price surrounding a central moving average. The ATR bands are typically
/// shown 'k' times moved away from the moving average.
///
/// # Formula
///
/// See EMA, ATR documentation.
///
/// KC is composed as:
///
///  * _KC<sub>Middle Band</sub>_ - Exponential Moving Average (EMA).
///  * _KC<sub>Upper Band</sub>_ = EMA + ATR of observation * multipler (usually 2.0)
///  * _KC<sub>Lower Band</sub>_ = EMA - ATR of observation * multipler (usually 2.0)
///
/// # Example
///
///```
/// use base::indicators::{KeltnerChannel, KeltnerChannelOutput};
/// use base::Next;
///
/// let mut kc = KeltnerChannel::new(3, 2.0_f64).unwrap();
///
/// let out_0 = kc.next(2.0);
///
/// let out_1 = kc.next(5.0);
///
/// assert_eq!(out_0.average, 2.0);
/// assert_eq!(out_0.upper, 2.0);
/// assert_eq!(out_0.lower, 2.0);
///
/// assert_eq!(out_1.average, 3.5);
/// assert_eq!(out_1.upper, 6.5);
/// assert_eq!(out_1.lower, 0.5);
/// ```
///
/// # Links
///
/// * [Keltner channel, Wikipedia](https://en.wikipedia.org/wiki/Keltner_channel)
#[doc(alias = "KC")]
#[derive(Debug, Clone)]
pub struct KeltnerChannel {
    period: usize,
    multiplier: f64,
    atr: AverageTrueRange,
    ema: ExponentialMovingAverage,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeltnerChannelOutput {
    pub average: f64,
    pub upper: f64,
    pub lower: f64,
}

impl KeltnerChannel {
    pub fn new(period: usize, multiplier: f64) -> Result<Self> {
        Ok(Self { period, multiplier, atr: AverageTrueRange::new(period)?, ema: ExponentialMovingAverage::new(period)? })
    }

    pub fn multiplier(&self) -> f64 {
        self.multiplier
    }
}

impl Period for KeltnerChannel {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<f64> for KeltnerChannel {
    type Output = KeltnerChannelOutput;

    fn next(&mut self, input: f64) -> Self::Output {
        let atr = self.atr.next(input);
        let average = self.ema.next(input);

        Self::Output { average, upper: average + atr * self.multiplier, lower: average - atr * self.multiplier }
    }
}

impl<T: Close + High + Low> Next<&T> for KeltnerChannel {
    type Output = KeltnerChannelOutput;

    fn next(&mut self, input: &T) -> Self::Output {
        let typical_price = (input.close() + input.high() + input.low()) / 3.0;

        let average = self.ema.next(typical_price);
        let atr = self.atr.next(input);

        Self::Output { average, upper: average + atr * self.multiplier, lower: average - atr * self.multiplier }
    }
}

impl Reset for KeltnerChannel {
    fn reset(&mut self) {
        self.atr.reset();
        self.ema.reset();
    }
}

impl Default for KeltnerChannel {
    fn default() -> Self {
        Self::new(10, 2_f64).unwrap()
    }
}

impl fmt::Display for KeltnerChannel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KC({}, {})", self.period, self.multiplier)
    }
}
