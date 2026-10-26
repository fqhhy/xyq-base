use std::fmt;

use crate::errors::{Result, TaError};
use crate::{Close, High, Low, Next, Period, Reset, Volume};

/// Money Flow Index (MFI).
///
/// The MFI is an volume and price based oscillator which gives moneyflow over n periods.
/// MFI is used to measure buying and selling pressure.
/// MFI is also known as volume-weighted RSI.
///
/// # Formula
///
/// Typical Price(TP) = (High + Low + Close)/3
///
/// Money Flow(MF) = Typical Price x Volume
///
/// MF is positive when currennt TP is greater that previous period TP and
/// negative when current TP is less than preivous TP.
///
/// Positive money flow (PMF)- calculated by adding the money flow of all the days RMF is positive.
///
/// Negative money flow (NMF)- calculated by adding the money flow of all the days RMF is negative.
///
/// Money Flow Index(MFI) = PMF / (PMF + NMF) * 100
///
///
/// # Parameters
///
/// * _period_ - number of periods, integer greater than 0
///
/// # Example
///
/// ```
/// use base::indicators::MoneyFlowIndex;
/// use base::Next;
/// use base::model::Bar;
///
/// let mut mfi = MoneyFlowIndex::new(3).unwrap();
/// let di = Bar {high:3.0,low:1.0,close:2.0,open:1.5,volume:1000.0,..Default::default()};
/// mfi.next(&di);
///
/// ```
/// # Links
/// * [Money Flow Index, Wikipedia](https://en.wikipedia.org/wiki/Money_flow_index)
/// * [Money Flow Index, stockcharts](https://stockcharts.com/school/doku.php?id=chart_school:technical_indicators:money_flow_index_mfi)

#[doc(alias = "MFI")]
#[derive(Debug, Clone)]
pub struct MoneyFlowIndex {
    period: usize,
    index: usize,
    count: usize,
    previous_typical_price: f64,
    total_positive_money_flow: f64,
    total_negative_money_flow: f64,
    deque: Box<[f64]>,
}

impl MoneyFlowIndex {
    pub fn new(period: usize) -> Result<Self> {
        match period {
            0 => Err(TaError::InvalidParameter),
            _ => Ok(Self { period, index: 0, count: 0, previous_typical_price: 0.0, total_positive_money_flow: 0.0, total_negative_money_flow: 0.0, deque: vec![0.0; period].into_boxed_slice() }),
        }
    }
}

impl Period for MoneyFlowIndex {
    fn period(&self) -> usize {
        self.period
    }
}

impl<T: High + Low + Close + Volume> Next<&T> for MoneyFlowIndex {
    type Output = f64;

    fn next(&mut self, input: &T) -> f64 {
        let tp = (input.close() + input.high() + input.low()) / 3.0;

        self.index = if self.index + 1 < self.period { self.index + 1 } else { 0 };

        if self.count < self.period {
            self.count = self.count + 1;
            if self.count == 1 {
                self.previous_typical_price = tp;
                return 50.0;
            }
        } else {
            let popped = self.deque[self.index];
            if popped.is_sign_positive() {
                self.total_positive_money_flow -= popped;
            } else {
                self.total_negative_money_flow += popped;
            }
        }

        if tp > self.previous_typical_price {
            let raw_money_flow = tp * input.volume();
            self.total_positive_money_flow += raw_money_flow;
            self.deque[self.index] = raw_money_flow;
        } else if tp < self.previous_typical_price {
            let raw_money_flow = tp * input.volume();
            self.total_negative_money_flow += raw_money_flow;
            self.deque[self.index] = -raw_money_flow;
        } else {
            self.deque[self.index] = 0.0;
        }
        self.previous_typical_price = tp;

        self.total_positive_money_flow / (self.total_positive_money_flow + self.total_negative_money_flow) * 100.0
    }
}

impl Default for MoneyFlowIndex {
    fn default() -> Self {
        Self::new(14).unwrap()
    }
}

impl fmt::Display for MoneyFlowIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MFI({})", self.period)
    }
}

impl Reset for MoneyFlowIndex {
    fn reset(&mut self) {
        self.index = 0;
        self.count = 0;
        self.previous_typical_price = 0.0;
        self.total_positive_money_flow = 0.0;
        self.total_negative_money_flow = 0.0;
        for i in 0..self.period {
            self.deque[i] = 0.0;
        }
    }
}
