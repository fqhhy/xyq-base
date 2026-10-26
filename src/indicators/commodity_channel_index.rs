use std::fmt;

use crate::errors::Result;
use crate::indicators::{MeanAbsoluteDeviation, SimpleMovingAverage};
use crate::{Close, High, Low, Next, Period, Reset};

/// Commodity Channel Index (CCI)
///
/// The commodity channel index is an oscillator originally introduced by Donald Lambert in 1980.
///
/// Since its introduction, the indicator has grown in popularity and is now a very common tool for
/// traders in identifying cyclical trends not only in commodities but also equities and currencies.
/// The CCI can be adjusted to the timeframe of the market traded on by changing the averaging period.
///
/// # Formula
///
/// CCI(_period_) = (TP - SMA(_period_) of TP) / (MAD(_period_) * 0.015)
///
/// # Parameters
///
/// * _period_ - number of periods (integer greater than 0). Default is 20.
///
/// # Links
///
/// * [Commodity Channel Index, Wikipedia](https://en.wikipedia.org/wiki/Commodity_channel_index)
/// * [Commodity Channel Index, StockCharts](https://school.stockcharts.com/doku.php?id=technical_indicators:commodity_channel_index_cci)
///
#[derive(Debug, Clone)]
pub struct CommodityChannelIndex {
    sma: SimpleMovingAverage,
    mad: MeanAbsoluteDeviation,
}

impl CommodityChannelIndex {
    pub fn new(period: usize) -> Result<Self> {
        Ok(Self { sma: SimpleMovingAverage::new(period)?, mad: MeanAbsoluteDeviation::new(period)? })
    }
}

impl Period for CommodityChannelIndex {
    fn period(&self) -> usize {
        self.sma.period()
    }
}

impl<T: Close + High + Low> Next<&T> for CommodityChannelIndex {
    type Output = f64;

    fn next(&mut self, input: &T) -> Self::Output {
        let tp = (input.close() + input.high() + input.low()) / 3.0;
        let sma = self.sma.next(tp);
        let mad = self.mad.next(input);

        if mad == 0.0 {
            return 0.0;
        }

        (tp - sma) / (mad * 0.015)
    }
}

impl Reset for CommodityChannelIndex {
    fn reset(&mut self) {
        self.sma.reset();
        self.mad.reset();
    }
}

impl Default for CommodityChannelIndex {
    fn default() -> Self {
        Self::new(20).unwrap()
    }
}

impl fmt::Display for CommodityChannelIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CCI({})", self.sma.period())
    }
}
