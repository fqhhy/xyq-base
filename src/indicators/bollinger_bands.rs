use std::fmt;

use crate::errors::Result;
use crate::indicators::StandardDeviation as Sd;
use crate::{Close, Next, Period, Reset};

/// A Bollinger Bands (BB).
/// (BB).
/// It is a type of infinite impulse response filter that calculates Bollinger Bands using Exponential Moving Average.
/// The Bollinger Bands are represented by Average EMA and standard deviaton that is moved 'k' times away in both directions from calculated average value.
///
/// # Formula
///
/// See SMA, SD documentation.
///
/// BB is composed as:
///
///  * _BB<sub>Middle Band</sub>_ - Simple Moving Average (SMA).
///  * _BB<sub>Upper Band</sub>_ = SMA + SD of observation * multipler (usually 2.0)
///  * _BB<sub>Lower Band</sub>_ = SMA - SD of observation * multipler (usually 2.0)
///
/// # Example
///
///```
/// use base::indicators::{BollingerBands, BollingerBandsOutput};
/// use base::Next;
///
/// let mut bb = BollingerBands::new(3, 2.0_f64).unwrap();
///
/// let out_0 = bb.next(2.0);
///
/// let out_1 = bb.next(5.0);
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
/// * [Bollinger Bands, Wikipedia](https://en.wikipedia.org/wiki/Bollinger_Bands)
#[doc(alias = "BB")]
#[derive(Debug, Clone)]
pub struct BollingerBands {
    period: usize,
    multiplier: f64,
    sd: Sd,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BollingerBandsOutput {
    pub average: f64,
    pub upper: f64,
    pub lower: f64,
}

impl BollingerBands {
    pub fn new(period: usize, multiplier: f64) -> Result<Self> {
        Ok(Self { period, multiplier, sd: Sd::new(period)? })
    }

    pub fn multiplier(&self) -> f64 {
        self.multiplier
    }
}

impl Period for BollingerBands {
    fn period(&self) -> usize {
        self.period
    }
}

impl Next<f64> for BollingerBands {
    type Output = BollingerBandsOutput;

    fn next(&mut self, input: f64) -> Self::Output {
        let sd = self.sd.next(input);
        let mean = self.sd.mean();

        Self::Output { average: mean, upper: mean + sd * self.multiplier, lower: mean - sd * self.multiplier }
    }
}

impl<T: Close> Next<&T> for BollingerBands {
    type Output = BollingerBandsOutput;

    fn next(&mut self, input: &T) -> Self::Output {
        self.next(input.close())
    }
}

impl Reset for BollingerBands {
    fn reset(&mut self) {
        self.sd.reset();
    }
}

impl Default for BollingerBands {
    fn default() -> Self {
        Self::new(9, 2_f64).unwrap()
    }
}

impl fmt::Display for BollingerBands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BB({}, {})", self.period, self.multiplier)
    }
}
