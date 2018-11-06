use client::*;
use result::BestbuyResult;

mod types;

pub use self::types::*;

#[derive(Serialize)]
pub enum OfferSort {
  #[serde(rename = "totalPrice")]
  TotalPrice,
  #[serde(rename = "price")]
  Price,
  #[serde(rename = "productTitle")]
  ProductTitle,
}

use types::{Pagination, Sort};

pub type ListOffersSort = Sort<OfferSort>;

#[derive(Default, Serialize, Clone)]
pub struct ListOffersParams {
  pub offer_state_codes: Option<Vec<String>>,
  pub sku: Option<String>,
  pub product_id: Option<String>,
  pub favorite: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ListOffersResponse {
  pub offers: Vec<Offer>,
  pub total_count: i32,
}

pub trait OfferApi {
  fn list_offers(
    &self,
    params: &ListOffersParams,
    sort: Option<ListOffersSort>,
    page: Option<Pagination>,
  ) -> BestbuyResult<ListOffersResponse>;
}

impl OfferApi for BestbuyClient {
  fn list_offers(
    &self,
    params: &ListOffersParams,
    sort: Option<ListOffersSort>,
    page: Option<Pagination>,
  ) -> BestbuyResult<ListOffersResponse> {
    let mut req = self.request(Method::Get, "/api/offers");

    req.query(&params);

    if let Some(sort) = sort {
      req.query(&sort);
    }

    if let Some(page) = page {
      req.query(&page);
    }
    req.send()?.get_response()
  }
}
