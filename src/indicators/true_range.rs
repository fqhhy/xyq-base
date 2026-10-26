use std::fmt;

use crate::{Close, High, Low, Next, Reset};

/// The range of a day's trading is simply _high_ - _low_.
/// The true range extends it to yesterday's closing price if it was outside of today's range.
///
/// The true range is the largest of one the following:
///
/// * Most recent period's high minus the most recent period's low
/// * Absolute value of the most recent period's high minus the previous close
/// * Absolute value of the most recent period's low minus the previous close
///
/// # Formula
///
/// TR = max[(high - low), abs(high - close<sub>prev</sub>), abs(low - close<sub>prev</sub>)]
///
/// # Example
///
/// ```
/// #[macro_use] extern crate assert_approx_eq;
///
/// use base::Next;
/// use base::model::Bar;
/// use base::indicators::TrueRange;
///
/// fn main() {
///     let data = vec![
///         // open, high, low, close, tr
///         (9.7   , 10.0, 9.0, 9.5  , 1.0),  // tr = high - low = 10.0 - 9.0 = 1.0
///         (9.9   , 10.4, 9.8, 10.2 , 0.9),  // tr = high - prev_close = 10.4 - 9.5 = 0.9
///         (10.1  , 10.7, 9.4, 9.7  , 1.3),  // tr = high - low = 10.7 - 9.4 = 1.3
///         (9.1   , 9.2 , 8.1, 8.4  , 1.6),  // tr = prev_close - low = 9.7 - 8.1 = 1.6
///     ];
///     let mut indicator = TrueRange::new();
///
///     for (open, high, low, close, tr) in data {
///         let di = Bar {high,low,close,open,volume:1000.0,..Default::default()};
///         assert_approx_eq!(indicator.next(&di), tr);
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct TrueRange {
    prev_close: Option<f64>,
}

impl TrueRange {
    pub fn new() -> Self {
        Self { prev_close: None }
    }
}

impl Default for TrueRange {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for TrueRange {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TRUE_RANGE()")
    }
}

impl Next<f64> for TrueRange {
    type Output = f64;

    fn next(&mut self, input: f64) -> Self::Output {
        let distance = match self.prev_close {
            Some(prev) => (input - prev).abs(),
            None => 0.0,
        };
        self.prev_close = Some(input);
        distance
    }
}

impl<T: High + Low + Close> Next<&T> for TrueRange {
    type Output = f64;

    fn next(&mut self, bar: &T) -> Self::Output {
        let max_dist = match self.prev_close {
            Some(prev_close) => {
                let dist1 = bar.high() - bar.low();
                let dist2 = (bar.high() - prev_close).abs();
                let dist3 = (bar.low() - prev_close).abs();
                max3(dist1, dist2, dist3)
            }
            None => bar.high() - bar.low(),
        };
        self.prev_close = Some(bar.close());
        max_dist
    }
}

impl Reset for TrueRange {
    fn reset(&mut self) {
        self.prev_close = None;
    }
}

pub fn max3(a: f64, b: f64, c: f64) -> f64 {
    a.max(b).max(c)
}
