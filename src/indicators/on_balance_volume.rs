use std::fmt;

use crate::{Close, Next, Reset, Volume};

/// On Balance Volume (OBV).
///
/// The OBV is an volume and price based oscillator which gives cumulative total volumes.
/// OBV measures buying and selling pressure as a cumulative indicator,
/// adding volume on up days and subtracting it on down days.
///
/// # Formula
///
/// If the closing price is above the prior close price then:
/// Current OBV = Previous OBV + Current Volume
///
/// If the closing price is below the prior close price then:
/// Current OBV = Previous OBV  -  Current Volume
///
/// If the closing prices equals the prior close price then:
/// Current OBV = Previous OBV
///
/// Where:
///
/// obv - on the balance volume
///
/// # Example
///
/// ```
/// use base::indicators::OnBalanceVolume;
/// use base::Next;
/// use base::model::Bar;
///
/// let mut obv = OnBalanceVolume::new();
///
/// let di1 = Bar{high:3.0,low:1.0,close:2.0,open:1.5,volume:1000.0,..Default::default()};
/// let di2 = Bar{high:3.0,low:1.0,close:1.5,open:1.5,volume:300.0,..Default::default()};
/// assert_eq!(obv.next(&di1), 1000.0);
/// assert_eq!(obv.next(&di2), 700.0);
/// ```
///
/// # Links
///
/// * [On Balance Volume, Wikipedia](https://en.wikipedia.org/wiki/On-balance_volume)
/// * [On Balance Volume, stockcharts](https://stockcharts.com/school/doku.php?id=chart_school:technical_indicators:on_balance_volume_obv)

#[doc(alias = "OBV")]
#[derive(Debug, Clone)]
pub struct OnBalanceVolume {
    obv: f64,
    prev_close: f64,
}

impl OnBalanceVolume {
    pub fn new() -> Self {
        Self { obv: 0.0, prev_close: 0.0 }
    }
}

impl<T: Close + Volume> Next<&T> for OnBalanceVolume {
    type Output = f64;

    fn next(&mut self, input: &T) -> f64 {
        if input.close() > self.prev_close {
            self.obv = self.obv + input.volume();
        } else if input.close() < self.prev_close {
            self.obv = self.obv - input.volume();
        }
        self.prev_close = input.close();
        self.obv
    }
}

impl Default for OnBalanceVolume {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for OnBalanceVolume {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OBV")
    }
}

impl Reset for OnBalanceVolume {
    fn reset(&mut self) {
        self.obv = 0.0;
        self.prev_close = 0.0;
    }
}
