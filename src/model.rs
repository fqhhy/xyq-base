use crate::{Close, High, Low, Market, Open, Volume};

/// K线数据 (不可变结构体)
#[derive(Debug, Default, Clone)]
pub struct Bar {
    /// 时间戳 1
    pub time: i64,
    /// 开盘价 7
    pub open: f64,
    /// 最高价 8
    pub high: f64,
    /// 最低价 9
    pub low: f64,
    /// 收盘价 11
    pub close: f64,
    /// 成交量 13
    pub volume: f64,
    /// 成交额 19
    pub amount: f64,
}

impl Open for Bar {
    fn open(&self) -> f64 {
        self.open
    }
}

impl High for Bar {
    fn high(&self) -> f64 {
        self.high
    }
}

impl Low for Bar {
    fn low(&self) -> f64 {
        self.low
    }
}

impl Close for Bar {
    fn close(&self) -> f64 {
        self.close
    }
}

impl Volume for Bar {
    fn volume(&self) -> f64 {
        self.volume
    }
}

/// Tick 数据结构
#[derive(Debug, Clone, Default)]
pub struct Tick10 {
    /// 时间戳 北京时间 UTC+8 "1"
    pub time: i64,
    /// 价格 "10"
    pub price: f64,
    /// 成交方向 "12"
    pub bs: i32,
    /// 成交量 "13"
    pub volume: f64,
    /// 外盘成交量 "14"
    pub outer_volume: f64,
    /// 交易笔数 "18"
    pub trade_num: f64,
    /// 总金额 "19"
    pub amount: f64,
    /// 买1价 "24"
    pub b1_p: f64,
    /// 买1量 "25"
    pub b1_v: f64,
    /// 买2价 "26"
    pub b2_p: f64,
    /// 买2量 "27"
    pub b2_v: f64,
    /// 买3价 "28"
    pub b3_p: f64,
    /// 买3量 "29"
    pub b3_v: f64,
    /// 卖1价 "30"
    pub a1_p: f64,
    /// 卖1量 "31"
    pub a1_v: f64,
    /// 卖2价 "32"
    pub a2_p: f64,
    /// 卖2量 "33"
    pub a2_v: f64,
    /// 卖3价 "34"
    pub a3_p: f64,
    /// 卖3量 "35"
    pub a3_v: f64,
    /// 当前量 "49"
    pub live_vol: f64,
    /// 板块总市值 "92"
    pub total_market_value: f64,
    /// 买6价 "102"
    pub b6_p: f64,
    /// 买6量 "103"
    pub b6_v: f64,
    /// 卖6价 "104"
    pub a6_p: f64,
    /// 卖6量 "105"
    pub a6_v: f64,
    /// 买7价 "106"
    pub b7_p: f64,
    /// 买7量 "107"
    pub b7_v: f64,
    /// 卖7价 "108"
    pub a7_p: f64,
    /// 卖7量 "109"
    pub a7_v: f64,
    /// 买8价 "110"
    pub b8_p: f64,
    /// 买8量 "111"
    pub b8_v: f64,
    /// 卖8价 "112"
    pub a8_p: f64,
    /// 卖8量 "113"
    pub a8_v: f64,
    /// 买9价 "114"
    pub b9_p: f64,
    /// 买9量 "115"
    pub b9_v: f64,
    /// 卖9价 "116"
    pub a9_p: f64,
    /// 卖9量 "117"
    pub a9_v: f64,
    /// 买10价 "118"
    pub b10_p: f64,
    /// 买10量 "119"
    pub b10_v: f64,
    /// 卖10价 "120"
    pub a10_p: f64,
    /// 卖10量 "121"
    pub a10_v: f64,
    /// 加权平均买价 "122"
    pub weighted_avg_bid_price: f64,
    /// 总委买量 "123"
    pub total_bid_volume: f64,
    /// 加权平均卖价 "124"
    pub weighted_avg_ask_price: f64,
    /// 总委卖量 "125"
    pub total_ask_volume: f64,
    /// 买4价 "150"
    pub b4_p: f64,
    /// 买4量 "151"
    pub b4_v: f64,
    /// 卖4价 "152"
    pub a4_p: f64,
    /// 卖4量 "153"
    pub a4_v: f64,
    /// 买5价 "154"
    pub b5_p: f64,
    /// 买5量 "155"
    pub b5_v: f64,
    /// 卖5价 "156"
    pub a5_p: f64,
    /// 卖5量 "157"
    pub a5_v: f64,
}

/// Tick 数据结构
#[derive(Debug, Clone, Default)]
pub struct Tick {
    ///
    pub code: StockCode,
    /// 时间戳 北京时间 UTC+8 "1"
    pub time: i64,
    /// 价格  "10"
    pub price: f64,
    /// 成交方向 key=12 value: 1   5   0   15
    pub bs: i32,
    /// 总成交量  "13"
    pub volume: f64,
    /// 外盘成交量  "14"
    pub outer_volume: f64,
    /// 交易笔数 "18"
    pub trade_num: f64,
    /// 总成交额 "19"
    pub amount: f64,
    /// 当前量 "49"
    pub live_vol: f64,

    /// 买1价 "20"
    pub b1_p: f64,
    /// 买1量 "25"
    pub b1_v: f64,
    /// 买2价 "26"
    pub b2_p: f64,
    /// 买2量  "27"
    pub b2_v: f64,
    /// 买3价  "28"
    pub b3_p: f64,
    /// 买3量 "29"
    pub b3_v: f64,

    /// 卖1价 "21"
    pub a1_p: f64,
    /// 卖1量 "31"
    pub a1_v: f64,
    /// 卖2价 "32"
    pub a2_p: f64,
    /// 卖2量 "33"
    pub a2_v: f64,
    /// 卖3价  "34"
    pub a3_p: f64,
    /// 卖3量 "35"
    pub a3_v: f64,

    /// 买4价  "150"
    pub b4_p: f64,
    /// 买4量 "151"
    pub b4_v: f64,
    /// 卖4价 "152"
    pub a4_p: f64,
    /// 卖4量 "153"
    pub a4_v: f64,
    /// 买5价 "154"
    pub b5_p: f64,
    /// 买5量 "155"
    pub b5_v: f64,
    /// 卖5价 "156"
    pub a5_p: f64,
    /// 卖5量 "157"
    pub a5_v: f64,
}

/// 订阅推送的 3s Tick 数据
#[derive(Debug, Clone, Default)]
pub struct TickSub {
    ///  key=5
    pub code: StockCode,
    /// 时间戳 北京时间 UTC+8  key=1
    pub time: i64,
    /// 昨收盘价  key=6
    pub prev_close: f64,
    /// 开盘价 7
    pub open: f64,
    /// 最高价 8
    pub high: f64,
    /// 最低价 9
    pub low: f64,
    /// 最新价格  key=10
    pub price: f64,
    /// 成交方向 key=12 value: 1   5   0   15
    pub bs: i32,
    /// 总成交量  key=13
    pub volume: f64,
    /// 外盘成交量  "14"
    pub outer_volume: f64,
    /// 交易笔数 "18"
    pub trade_num: f64,
    /// 总成交额 "19"
    pub amount: f64,
    /// 当前量 "49"
    pub live_vol: f64,

    /// 买1价 "20"
    pub b1_p: f64,
    /// 买1量 "25"
    pub b1_v: f64,
    /// 买2价 "26"
    pub b2_p: f64,
    /// 买2量  "27"
    pub b2_v: f64,
    /// 买3价  "28"
    pub b3_p: f64,
    /// 买3量 "29"
    pub b3_v: f64,

    /// 卖1价 "21"
    pub a1_p: f64,
    /// 卖1量 "31"
    pub a1_v: f64,
    /// 卖2价 "32"
    pub a2_p: f64,
    /// 卖2量 "33"
    pub a2_v: f64,
    /// 卖3价  "34"
    pub a3_p: f64,
    /// 卖3量 "35"
    pub a3_v: f64,

    /// 买4价  "150"
    pub b4_p: f64,
    /// 买4量 "151"
    pub b4_v: f64,
    /// 卖4价 "152"
    pub a4_p: f64,
    /// 卖4量 "153"
    pub a4_v: f64,
    /// 买5价 "154"
    pub b5_p: f64,
    /// 买5量 "155"
    pub b5_v: f64,
    /// 卖5价 "156"
    pub a5_p: f64,
    /// 卖5量 "157"
    pub a5_v: f64,
}

/// 逐笔成交数据
#[derive(Debug, Clone, Default)]
pub struct L2Order {
    /// 合约代码（协议本身不带，由各数据源从附加字段回填）
    pub code: StockCode,
    /// 顺序号 "1"
    pub seq: i64,
    /// 挂单时间 "56"
    pub time: i64,
    /// 价格 "10"
    pub price: f64,
    /// 成交方向 "12"
    pub bs: i32,
    /// 成交量 "13"
    pub volume: f64,
    /// 买单订单ID "74"
    pub bid_order_id: i64,
    /// 卖单订单ID "75"
    pub ask_order_id: i64,
    /// 交易笔数 "18"
    pub trade_num: f64,
}

/// 订阅盘口数据
#[derive(Debug, Default)]
pub struct OrderBookSnapshot {
    /// 合约代码
    pub code: StockCode,
    /// 时间戳
    pub time: i64,
    pub whole_pankou: bool,
    pub ask_count: i32,
    pub bid_count: i32,
    pub ticks: Vec<DepthLevel>,
}

#[derive(Debug, Default)]
pub struct DepthLevel {
    /// 12 ask bid标志,1买ask   2 卖bid
    pub side: i32,
    /// 10 委托价格
    pub price: f64,
    /// 13 盘口数量
    pub volume: f64,
    /// 53 顺序号, 相同价格的数据包, 按时间排序的顺序号
    pub seq: i64,
    /// 54 均价, 沪深品种没有值
    pub avg_price: f64,
}

/// 标的身份，`u64` 打包。布局定稿见 `docs/design/03-market_data_model.md` 3.4：
///
/// ```text
/// bits 0..7    market（统一市场编号：0=深 1=沪 2=北，其余预留待定）
/// bits 8..55   6 字符 ASCII 代码（左对齐、右补 NUL，如 "600519"/"000001"）
/// bits 56..63  预留（恒 0）
/// ```
///
/// 纯值类型：唯一创建入口是 [`StockCode::new`]（传 6 字节代码 + 市场编码）。
/// 各数据源的字符串解析（THS `USHA`/`USZA` 前缀等）在各源自己的包内实现，
/// 不在 base。热路径（比较/哈希）是 u64 整数运算、零堆分配；`Display` 仅在
/// 边界/日志使用。
#[derive(Default, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StockCode(u64);

impl StockCode {
    /// 唯一创建 API：`code` 为 6 字节 ASCII 代码（左对齐、右补 NUL），
    /// `market` 为所属交易所（判别值即市场字节，见 [`Market`]）。
    pub fn new(code: [u8; 6], market: Market) -> Self {
        let mut v = 0u64;
        for (i, shift) in [48, 40, 32, 24, 16, 8].into_iter().enumerate() {
            v |= (code[i] as u64) << shift;
        }
        Self(v | market.as_byte() as u64)
    }

    /// 所属交易所（读 market 字节，0=深 1=沪 2=北）。
    pub fn market(&self) -> Market {
        Market::from_byte((self.0 & 0xFF) as u8)
    }

    /// 6 字节代码（左对齐，右补 NUL）。
    pub fn code(&self) -> [u8; 6] {
        let mut code = [0u8; 6];
        for (i, shift) in [48, 40, 32, 24, 16, 8].into_iter().enumerate() {
            code[i] = ((self.0 >> shift) & 0xFF) as u8;
        }
        code
    }
}

impl std::fmt::Display for StockCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let code = self.code();
        let end = code.iter().position(|&b| b == 0).unwrap_or(6);
        if end == 0 {
            return Ok(()); // 全 0（默认值）：输出空串
        }
        let s = std::str::from_utf8(&code[..end]).unwrap_or("");
        let suffix = match self.market() {
            Market::SZSE => ".SZ",
            Market::SSE => ".SH",
            Market::BJSE => ".BJ",
        };
        write!(f, "{s}{suffix}")
    }
}
impl std::fmt::Debug for StockCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

/// 标的基础信息。
#[derive(Clone, Copy, Debug, Default)]
pub struct ContractInfo {
    /// 是否 T+0（true=T+0, false=T+1）。
    pub is_t0: bool,
    /// 整手（A股 100）。
    pub lot_size: f64,
    /// 最小变动价位。
    pub price_tick: f64,
}

#[cfg(test)]
mod stock_code_tests {
    use super::*;
    use std::collections::HashMap;

    fn stock_code(code: &str, market: Market) -> StockCode {
        let mut c = [0u8; 6];
        let d: Vec<u8> = code.bytes().collect();
        c[6 - d.len()..].copy_from_slice(&d);
        StockCode::new(c, market)
    }

    #[test]
    fn new_and_display() {
        assert_eq!(stock_code("600519", Market::SSE).to_string(), "600519.SH");
        assert_eq!(stock_code("000001", Market::SZSE).to_string(), "000001.SZ");
        assert_eq!(stock_code("830001", Market::BJSE).to_string(), "830001.BJ");
        // 默认值（全 0）输出空串
        assert_eq!(StockCode::default().to_string(), "");
    }

    #[test]
    fn accessors() {
        let c = stock_code("600519", Market::SSE);
        assert_eq!(c.market(), Market::SSE);
        assert_eq!(c.code(), *b"600519");
        assert_eq!(stock_code("000001", Market::SZSE).market(), Market::SZSE);
        assert_eq!(stock_code("830001", Market::BJSE).market(), Market::BJSE);
        assert_eq!(stock_code("920001", Market::BJSE).market(), Market::BJSE);
    }

    #[test]
    fn is_copy_and_ordered() {
        let a = stock_code("000001", Market::SZSE);
        let b = a; // Copy
        assert_eq!(a, b);
        assert!(stock_code("000001", Market::SZSE) < stock_code("000002", Market::SZSE));
        assert!(stock_code("600519", Market::SSE) > stock_code("000001", Market::SZSE));
    }

    #[test]
    fn contract_info_t0_default_and_lookup() {
        let mut contracts: HashMap<StockCode, ContractInfo> = HashMap::new();
        let sse = stock_code("600519", Market::SSE);

        // 未收录默认 T+1
        assert!(!contracts.get(&sse).map(|i| i.is_t0).unwrap_or(false));

        contracts.insert(
            sse,
            ContractInfo {
                is_t0: false,
                lot_size: 100.0,
                price_tick: 0.01,
            },
        );
        assert!(!contracts.get(&sse).unwrap().is_t0);
        assert_eq!(contracts.get(&sse).unwrap().lot_size, 100.0);
    }
}
