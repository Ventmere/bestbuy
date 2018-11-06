use reqwest::StatusCode;

#[derive(Fail, Debug)]
pub enum BestbuyError {
  #[fail(
    display = "request error: path = '{}', status = '{}', body = '{}'",
    path, status, body
  )]
  Request {
    path: String,
    status: StatusCode,
    body: String,
  },

  #[fail(display = "deserialize body error: msg = '{}', body = '{}'", msg, body)]
  Deserialize { msg: String, body: String },

  #[fail(display = "http error: {}", _0)]
  Http(::reqwest::Error),

  #[fail(display = "json error: {}", _0)]
  Json(::serde_json::Error),
}

impl BestbuyError {
  pub fn should_try_again(&self) -> bool {
    match *self {
      BestbuyError::Request { status, .. } => {
        let code = status.as_u16();
        // 429 Too Many Requests
        code == 429 || code == 500 || code == 503
      }
      _ => false,
    }
  }
}

pub type BestbuyResult<T> = ::std::result::Result<T, BestbuyError>;

macro_rules! impl_from {
  ($v:ident($t:ty)) => {
    impl From<$t> for BestbuyError {
      fn from(e: $t) -> Self {
        BestbuyError::$v(e)
      }
    }
  };
}

impl_from!(Http(::reqwest::Error));
impl_from!(Json(::serde_json::Error));
