use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Debug, Clone, Serialize)]
pub struct Pagination {
  pub max: Option<i32>,
  pub offset: Option<i32>,
}

#[derive(Debug)]
pub enum Sort<K: Serialize> {
  Asc(K),
  Desc(K),
}

impl<K: Serialize> Serialize for Sort<K> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut state = serializer.serialize_struct("Sort", 2)?;
    match *self {
      Sort::Asc(ref k) => {
        state.serialize_field("sort", k)?;
        state.serialize_field("order", "asc")?;
      }
      Sort::Desc(ref k) => {
        state.serialize_field("sort", k)?;
        state.serialize_field("order", "desc")?;
      }
    }
    state.end()
  }
}
