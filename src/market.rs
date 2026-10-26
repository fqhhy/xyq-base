use serde::{Deserialize, Serialize};

/// A股交易所。
///
/// 判别值 = `StockCode` 的 market 字节（见 `docs/design/03-market_data_model.md`
/// 3.4 的编号表）：0=深、1=沪、2=北。3-255 为预留未定义市场，接入时再登记。
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Market {
    /// 深圳（SZSE）
    #[default]
    SZSE = 0,
    /// 上海（SSE）
    SSE = 1,
    /// 北京/北交所（BJSE）
    BJSE = 2,
}

impl Market {
    /// 对应的 market 字节（与 `StockCode` 布局一致）。
    pub fn as_byte(self) -> u8 {
        self as u8
    }

    /// 由 `StockCode` 的 market 字节转换；3-255 为预留未定义，暂按深市兜底。
    pub fn from_byte(b: u8) -> Market {
        match b {
            1 => Market::SSE,
            2 => Market::BJSE,
            _ => Market::SZSE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Market;

    #[test]
    fn discriminants_match_stock_code_market_bytes() {
        assert_eq!(Market::SZSE as u8, 0);
        assert_eq!(Market::SSE as u8, 1);
        assert_eq!(Market::BJSE as u8, 2);
        assert_eq!(std::mem::size_of::<Market>(), 1);
    }

    #[test]
    fn byte_roundtrip() {
        for m in [Market::SZSE, Market::SSE, Market::BJSE] {
            assert_eq!(Market::from_byte(m.as_byte()), m);
        }
        assert_eq!(Market::from_byte(0), Market::SZSE);
        assert_eq!(Market::from_byte(7), Market::SZSE); // 预留未定义 → 兜底
    }
}
