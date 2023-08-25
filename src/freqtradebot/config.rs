use std::collections::HashMap;
use std::fs;

use serde::{Deserialize, Serialize};
use crate::freqtradebot::constant::{MarginMode, OrderType, PriceSide, TimeUnit, TradeMode};

#[derive(Debug, Serialize, Deserialize)]
pub struct UnfilledTimeoutConfig {
    entry: Option<u32>,
    exit: Option<u32>,
    unit: Option<TimeUnit>,
    exit_timeout_count: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckMarketDepthConfig {
    enabled: bool,
    bids_to_ask_delta: Option<f32>,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryPriceConfig {
    price_side: Option<PriceSide>,
    price_last_balance: Option<f32>,
    use_order_book: Option<bool>,
    order_book_top: Option<u8>,
    check_depth_of_market: Option<CheckMarketDepthConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExitPriceConfig {
    price_side: Option<PriceSide>,
    price_last_balance: Option<f32>,
    use_order_book: Option<bool>,
    order_book_top: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderTypesConfig {
    entry: OrderType,
    exit: OrderType,
    emergency_exit: OrderType,
    force_entry: OrderType,
    force_exit: OrderType,
    stoploss: OrderType,
    stoploss_on_exchange: bool,
    stoploss_price_type: Option<String>,
    stoploss_on_exchange_interval: u32,
    stoploss_on_exchange_limit_ratio: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EdgeConfig {
    enabled: Option<bool>,
    process_throttle_secs: Option<u32>,
    calculate_since_number_of_days: Option<u32>,
    allowed_risk: Option<f32>,
    stoploss_range_min: Option<f32>,

}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeConfig {
    name: String,
    sandbox: Option<bool>,
    key: Option<String>,
    secret: Option<String>,
    password: Option<String>,
    uid: Option<String>,
    pair_whitelist: Option<Vec<String>>,
    pair_blacklist: Option<Vec<String>>,
    ccxt_config: Option<HashMap<String, String>>,
    ccxt_sync_config: Option<HashMap<String, String>>,
    ccxt_async_config: Option<HashMap<String, String>>,
    markets_refresh_interval: Option<u32>,
    skip_pair_validation: Option<bool>,
    skip_open_order_update: Option<bool>,
    unknown_fee_rate: Option<f32>,
    log_responses: Option<bool>,
    block_bad_exchanges: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    max_open_trades: Option<u8>,
    stake_currency: Option<String>,
    stake_amount: Option<f32>,
    tradable_balance_ratio: Option<f32>,
    available_capital: Option<f32>,
    amend_last_stake_amount: Option<bool>,
    last_stake_amount_min_ratio: Option<f32>,
    amount_reserve_percent: Option<f32>,
    timeframe: Option<String>,
    fiat_display_currency: Option<String>,
    dry_run: Option<bool>,
    dry_run_wallet: Option<f32>,
    cancel_open_orders_on_exit: Option<bool>,
    process_only_new_candles: Option<bool>,
    minimal_roi: Option<HashMap<u32, f32>>,
    stoploss: Option<f32>,
    trailing_stop: Option<bool>,
    trailing_stop_positive: Option<f32>,
    trailing_stop_positive_offset: Option<f32>,
    trailing_only_offset_is_reached: Option<bool>,
    fee: Option<f32>,
    futures_funding_rate: Option<f32>,
    trading_mode: Option<TradeMode>,
    margin_mode: Option<MarginMode>,
    liquidation_buffer: Option<f32>,
    unfilledtimeout: Option<UnfilledTimeoutConfig>,
    entry_pricing: Option<EntryPriceConfig>,
    exit_pricing: Option<ExitPriceConfig>,
    custom_price_max_distance_ratio: Option<f32>,
    use_exit_signal: Option<bool>,
    exit_profit_only: Option<bool>,
    exit_profit_offset: Option<f32>,
    ignore_roi_if_entry_signal: Option<bool>,
    ignore_buying_expired_candle_after: Option<u32>,
    order_types: Option<OrderTypesConfig>,
    order_time_in_force: Option<HashMap<String, String>>,
    position_adjustment_enable: Option<bool>,
    max_entry_position_adjustment: Option<i32>,
    exchange: ExchangeConfig,
    edge: Option<EdgeConfig>,

}


#[test]
fn test_parse() {
    let data = fs::read_to_string("config/config_full.example.json").expect("Unable to read file");
    let json: Configuration = serde_json::from_str(&data).unwrap();
    println!("{:?}", json);
}