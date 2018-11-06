use reqwest::{Client, Response, StatusCode};
pub use reqwest::{Method, RequestBuilder};
use result::{BestbuyError, BestbuyResult};
use serde::Deserialize;
use serde_json;

pub struct BestbuyClient {
  http: Client,
  token: String,
}

impl BestbuyClient {
  pub fn new(token: &str) -> Self {
    Self::with_http_client(token, Client::new())
  }

  pub fn with_http_client(token: &str, http: Client) -> Self {
    Self {
      token: token.to_owned(),
      http,
    }
  }

  pub fn request(&self, method: Method, path: &str) -> RequestBuilder {
    use reqwest::{
      header::{qitem, Accept, Authorization, CacheControl, CacheDirective},
      mime,
    };
    let mut b = self
      .http
      .request(method, &format!("https://marketplace.bestbuy.ca{}", path));
    b.header(Authorization(self.token.clone()))
      .header(CacheControl(vec![CacheDirective::NoCache]))
      .header(Accept(vec![qitem(mime::APPLICATION_JSON)]));
    b
  }
}

pub trait BestbuyResponse {
  fn get_response<T: for<'de> Deserialize<'de>>(&mut self) -> BestbuyResult<T>;
}

impl BestbuyResponse for Response {
  fn get_response<T: for<'de> Deserialize<'de>>(&mut self) -> BestbuyResult<T> {
    let body = self.text()?;

    if self.status() != StatusCode::Ok {
      return Err(BestbuyError::Request {
        path: self.url().to_string(),
        status: self.status(),
        body,
      });
    }

    match serde_json::from_str(&body) {
      Ok(v) => Ok(v),
      Err(err) => {
        return Err(BestbuyError::Deserialize {
          msg: err.to_string(),
          body,
        })
      }
    }
  }
}
