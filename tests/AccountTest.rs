use base::Market;
use base::account::{Account, Order};
use base::model::StockCode;

fn stock_code(code: &str, market: Market) -> StockCode {
    let mut c = [0u8; 6];
    c[6 - code.len()..].copy_from_slice(code.as_bytes());
    StockCode::new(c, market)
}

#[test]
fn test_account() {
    //初始化账户
    let mut account = Account { balance: 1_000_000.0, available_balance: 1_000_000.0, ..Default::default() };
    let code = stock_code("600795", Market::SSE);

    // 模拟买入一单
    let order = Order { market_type: "0".parse().unwrap(), code, time: 1, order_type: "B".parse().unwrap(), price: 1.0, volume: 100.0 };
    account.buy(&order);
    account.on_price_change(code, 1.0);

    // 账户市值
    assert_eq!(account.balance, 1_000_000.0);
    assert_eq!(account.available_balance, 1_000_000.0 - 100.0);
    assert_eq!(account.portfolio_value, 100.0); // 总市值
    assert_eq!(account.profit, 0.0);

    // 持仓
    let pos = account.get_position(code);
    assert_eq!(pos.volume, 100.0); // 持仓量
    assert_eq!(pos.current_price, 1.0); // 当前价
    assert_eq!(pos.market_value, 100.0); // 市值
    assert_eq!(pos.cost_price, 1.0); // 成本价

    // 再买一单
    let code2 = stock_code("601111", Market::SSE);
    let order2 = Order { market_type: "0".parse().unwrap(), code: code2, time: 1, order_type: "B".parse().unwrap(), price: 1.0, volume: 200.0 };
    account.buy(&order2);
    account.on_price_change(code2, 1.0);

    assert_eq!(account.balance, 1_000_000.0);
    assert_eq!(account.available_balance, 1_000_000.0 - 300.0);
    assert_eq!(account.portfolio_value, 300.0); // 总市值
    assert_eq!(account.profit, 0.0);

    // 持仓
    let pos2 = account.get_position(code2);
    assert_eq!(pos2.volume, 200.0); // 持仓量
    assert_eq!(pos2.current_price, 1.0); // 当前价
    assert_eq!(pos2.market_value, 200.0); // 市值
    assert_eq!(pos2.cost_price, 1.0); // 成本价
}
