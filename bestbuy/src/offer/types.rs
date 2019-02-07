use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Offer {
  pub active: bool,
  pub available_end_date: Option<DateTime<Utc>>,
  pub available_start_date: Option<DateTime<Utc>>,
  pub all_prices: Vec<ApplicablePricing>,
  pub allow_quote_requests: bool,
  pub applicable_pricing: ApplicablePricing,
  pub category_code: String,
  pub category_label: String,
  pub channels: Vec<String>,
  pub currency_iso_code: String,
  pub description: Option<String>,
  pub discount: Option<Discount>,
  pub logistic_class: LogisticClass,
  pub min_quantity_alert: Option<i64>,
  pub min_shipping_price: Option<f64>,
  pub min_shipping_price_additional: Option<f64>,
  pub min_shipping_type: Option<String>,
  pub min_shipping_zone: Option<String>,
  pub offer_additional_fields: Vec<OfferAdditionalField>,
  pub offer_id: i64,
  pub price: f64,
  pub price_additional_info: Option<Value>,
  pub product_references: Vec<ProductReference>,
  pub product_sku: String,
  pub product_title: String,
  pub quantity: i64,
  pub shop_sku: String,
  pub state_code: String,
  pub total_price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discount {
  pub start_date: Option<DateTime<Utc>>,
  pub end_date: Option<DateTime<Utc>>,
  pub discount_price: f64,
  pub origin_price: f64,
  pub ranges: Vec<Range>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
  pub price: f64,
  pub quantity_threshold: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicablePricing {
  pub channel_code: Option<Value>,
  pub discount_end_date: Option<Value>,
  pub discount_start_date: Option<Value>,
  pub price: f64,
  pub unit_discount_price: Option<Value>,
  pub unit_origin_price: f64,
  pub volume_prices: Vec<VolumePrice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumePrice {
  pub price: f64,
  pub quantity_threshold: i64,
  pub unit_discount_price: Option<Value>,
  pub unit_origin_price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogisticClass {
  pub code: String,
  pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfferAdditionalField {
  pub code: String,
  #[serde(rename = "type")]
  pub type_: String,
  pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductReference {
  pub reference: String,
  pub reference_type: String,
}
