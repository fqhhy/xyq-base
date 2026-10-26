//! 执行域数据模型最小集（订单/回报）。
//!
//! 对应设计 `01-policy_engine_design.md` §1.1：策略接口契约引用的执行域类型统一
//! 收敛在共享数据模型层，执行引擎与策略引擎在此绑定（`07` §7.2 "收敛进统一订单/
//! 账户模型"）。此处只定义契约引用的类型，不定义撮合/状态机等执行引擎内部实现。

use crate::model::StockCode;

/// 委托方向（A 股只有买/卖，无开平仓语义；`07` §7.2）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// 订单类型（A 股补全，`07` §7.2）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OrderType {
    /// 限价。
    Limit,
    /// 市价。
    Market,
}

/// 委托有效期。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TimeInForce {
    /// 当日有效。
    Day,
}

/// 订单生命周期状态（策略接口层合并视图用）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OrderStatus {
    /// 已受理。
    New,
    /// 已确认。
    Ack,
    /// 部分成交。
    PartiallyFilled,
    /// 全部成交。
    Filled,
    /// 已撤单。
    Canceled,
    /// 已拒绝。
    Rejected,
}

/// 委托请求（策略发单的入参）。
///
/// 字段以现有代码反推（`xyq-execution` 的 `OrderRequest` 用法）+ `07` §7.2 差距
/// 补齐。A 股整手 100 股校验在执行域，不在本类型。
#[derive(Clone, Debug)]
pub struct OrderRequest {
    /// 本地单号（策略侧生成，回报据此对账）。
    pub client_order_id: String,
    /// 标的。
    pub instrument: StockCode,
    /// 委托时间戳（微秒）。
    pub timestamp_us: u64,
    /// 方向。
    pub side: OrderSide,
    /// 订单类型。
    pub order_type: OrderType,
    /// 有效期。
    pub time_in_force: TimeInForce,
    /// 委托价格。
    pub price: f64,
    /// 委托数量。
    pub quantity: f64,
}

impl OrderRequest {
    /// 构造一个限价委托。
    pub fn limit(
        client_order_id: impl Into<String>,
        instrument: StockCode,
        timestamp_us: u64,
        side: OrderSide,
        price: f64,
        quantity: f64,
    ) -> Self {
        Self {
            client_order_id: client_order_id.into(),
            instrument,
            timestamp_us,
            side,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::Day,
            price,
            quantity,
        }
    }

    /// 构造一个市价委托（A 股市价暂以最新价替代，见 `07` §7.2）。
    pub fn market(
        client_order_id: impl Into<String>,
        instrument: StockCode,
        timestamp_us: u64,
        side: OrderSide,
        price: f64,
        quantity: f64,
    ) -> Self {
        Self {
            client_order_id: client_order_id.into(),
            instrument,
            timestamp_us,
            side,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::Day,
            price,
            quantity,
        }
    }
}

/// 订单回报（`on_order` 的入参）。
///
/// 执行域内部 `OrderUpdate`（状态流转）+ `Fill`（成交）的拆分保留；策略接口层
/// 合并为一个视图，避免策略为对齐两个事件各自写分支（`01` §1.1.2）。
#[derive(Clone, Debug)]
pub struct OrderReport {
    /// 本地单号。
    pub client_order_id: String,
    /// 标的。
    pub instrument: StockCode,
    /// 回报时间戳（微秒）。
    pub timestamp_us: u64,
    /// 状态。
    pub status: OrderStatus,
    /// 拒绝原因（拒单时）。
    pub reason: Option<String>,
    /// 本次回报的成交增量。
    pub filled_quantity: f64,
    /// 累计均价（策略算持仓成本用）。
    pub avg_fill_price: f64,
}
