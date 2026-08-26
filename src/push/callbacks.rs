//! 推送回调函数类型定义
//!
//! 每种推送类型独立回调，连接状态回调独立管理。
//! 回调参数使用 Protobuf 生成的 `pb::*` 类型。

use super::pb;
use std::sync::Arc;

/// 所有回调函数的集合
#[derive(Default, Clone)]
pub struct Callbacks {
    // 行情推送回调
    pub on_quote: Option<Arc<dyn Fn(pb::QuoteData) + Send + Sync>>,
    pub on_tick: Option<Arc<dyn Fn(pb::TradeTickData) + Send + Sync>>,
    pub on_depth: Option<Arc<dyn Fn(pb::QuoteDepthData) + Send + Sync>>,
    pub on_option: Option<Arc<dyn Fn(pb::QuoteData) + Send + Sync>>,
    pub on_future: Option<Arc<dyn Fn(pb::QuoteData) + Send + Sync>>,
    pub on_kline: Option<Arc<dyn Fn(pb::KlineData) + Send + Sync>>,
    pub on_stock_top: Option<Arc<dyn Fn(pb::StockTopData) + Send + Sync>>,
    pub on_option_top: Option<Arc<dyn Fn(pb::OptionTopData) + Send + Sync>>,
    pub on_full_tick: Option<Arc<dyn Fn(pb::TickData) + Send + Sync>>,
    pub on_quote_bbo: Option<Arc<dyn Fn(pb::QuoteData) + Send + Sync>>,
    // 账户推送回调
    pub on_asset: Option<Arc<dyn Fn(pb::AssetData) + Send + Sync>>,
    pub on_position: Option<Arc<dyn Fn(pb::PositionData) + Send + Sync>>,
    pub on_order: Option<Arc<dyn Fn(pb::OrderStatusData) + Send + Sync>>,
    pub on_transaction: Option<Arc<dyn Fn(pb::OrderTransactionData) + Send + Sync>>,
    // 连接状态回调
    pub on_connect: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_disconnect: Option<Arc<dyn Fn() + Send + Sync>>,
    pub on_error: Option<Arc<dyn Fn(String) + Send + Sync>>,
    pub on_kickout: Option<Arc<dyn Fn(String) + Send + Sync>>,
    /// Invoked once per decoded heartbeat response received from the
    /// server. Proof of inbound liveness during an otherwise-quiet
    /// account — heartbeat responses are received every few seconds
    /// regardless of subscribed-data activity, so this is the only
    /// signal that distinguishes a silently-dead connection from a
    /// healthy one with nothing to report. `None` preserves the
    /// original silent-ignore behavior.
    pub on_heartbeat: Option<Arc<dyn Fn() + Send + Sync>>,
}
