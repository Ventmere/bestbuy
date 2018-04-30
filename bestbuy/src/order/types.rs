use chrono::{DateTime, Utc};
use serde_json::Value;

state_enum! {
  pub enum OrderState {
    Staging,
    WaitingAcceptance,
    WaitingDebit,
    WaitingDebitPayment,
    Shipping,
    Shipped,
    ToCollect,
    Received,
    Closed,
    Refused,
    Canceled,
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
  pub acceptance_decision_date: Option<String>,
  pub can_cancel: bool,
  pub channel: Option<Channel>,
  pub commercial_id: String,
  pub created_date: DateTime<Utc>,
  pub currency_iso_code: String,
  pub customer: Customer,
  pub customer_debited_date: Option<DateTime<Utc>>,
  pub has_customer_message: bool,
  pub has_incident: bool,
  pub has_invoice: bool,
  pub last_updated_date: DateTime<Utc>,
  pub leadtime_to_ship: i64,
  pub order_additional_fields: Vec<Value>,
  pub order_id: String,
  pub order_lines: Vec<OrderLine>,
  pub order_state: OrderState,
  pub order_state_reason_code: Option<String>,
  pub order_state_reason_label: Option<String>,
  pub payment_type: Option<String>,
  pub payment_workflow: Option<String>,
  pub price: f64,
  pub promotions: Promotions,
  pub quote_id: Option<Value>,
  pub shipping_carrier_code: Option<String>,
  pub shipping_company: Option<String>,
  pub shipping_price: f64,
  pub shipping_tracking: Option<String>,
  pub shipping_tracking_url: Option<String>,
  pub shipping_type_code: String,
  pub shipping_type_label: String,
  pub shipping_zone_code: String,
  pub shipping_zone_label: String,
  pub total_commission: f64,
  pub total_price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Channel {
  pub code: Option<String>,
  pub label: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
  pub city: String,
  pub civility: String,
  pub company: String,
  pub country: String,
  pub country_iso_code: Option<String>,
  pub firstname: String,
  pub lastname: String,
  pub phone: String,
  pub phone_secondary: String,
  pub state: String,
  pub street_1: String,
  pub street_2: String,
  pub zip_code: Option<String>,
  pub additional_info: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
  pub billing_address: Option<Address>,
  pub civility: String,
  pub customer_id: String,
  pub firstname: String,
  pub lastname: String,
  pub locale: Option<String>,
  pub shipping_address: Option<Address>,
}

state_enum! {
  pub enum OrderLineState {
    Staging,
    WaitingAcceptance,
    WaitingDebit,
    WaitingDebitPayment,
    Shipping,
    ToCollect,
    Received,
    Closed,
    Refused,
    Canceled,
    IncidentOpen,
    Refunded,
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderLine {
  pub can_refund: bool,
  pub cancelations: Vec<Value>,
  pub category_code: Option<String>,
  pub category_label: Option<String>,
  pub commission_fee: f64,
  pub commission_rate_vat: f64,
  pub commission_taxes: Vec<CommissionTax>,
  pub commission_vat: f64,
  pub created_date: DateTime<Utc>,
  pub debited_date: Option<DateTime<Utc>>,
  pub description: Option<Value>,
  pub last_updated_date: Option<DateTime<Utc>>,
  pub offer_id: i64,
  pub offer_sku: String,
  pub offer_state_code: String,
  pub order_line_additional_fields: Vec<Value>,
  pub order_line_id: String,
  pub order_line_index: i32,
  pub order_line_state: OrderLineState,
  pub order_line_state_reason_code: Option<String>,
  pub order_line_state_reason_label: Option<String>,
  pub price: f64,
  pub price_additional_info: Option<Value>,
  pub price_unit: f64,
  pub product_medias: Vec<Value>,
  pub product_sku: String,
  pub product_title: String,
  pub promotions: Vec<Value>,
  pub quantity: i32,
  pub received_date: Option<DateTime<Utc>>,
  pub refunds: Vec<Value>,
  pub shipped_date: Option<DateTime<Utc>>,
  pub shipping_price: f64,
  pub shipping_price_additional_unit: Option<Value>,
  pub shipping_price_unit: Option<Value>,
  pub shipping_taxes: Vec<Tax>,
  pub taxes: Vec<Tax>,
  pub total_commission: f64,
  pub total_price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommissionTax {
  pub amount: f64,
  pub code: String,
  pub rate: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tax {
  pub amount: Option<f64>,
  pub code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Promotions {
  pub applied_promotions: Vec<Value>,
  pub total_deduced_amount: f64,
}
