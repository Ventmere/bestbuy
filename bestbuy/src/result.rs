use reqwest::StatusCode;

error_chain! {
  errors {
    Request(path: String, status: StatusCode, body: String) {
      description("request error")
      display("request error: path = '{}', status = '{}', body = '{}'", path, status, body)
    }
    Deserialize(msg: String, body: String) {
      description("deserialize body error")
      display("deserialize body error: {}, body = '{}'", msg, body)
    }
  }

  foreign_links {
    Http(::reqwest::Error);
    Json(::serde_json::Error);
  }
}

pub type BestbuyResult<T> = ::std::result::Result<T, Error>;
