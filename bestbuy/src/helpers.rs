#[doc(hidden)]
#[macro_export]
macro_rules! state_enum {
  (pub enum $name:ident { $($v:ident,)+ }) => {
    #[derive(Debug, Serialize, Deserialize, Clone, Copy)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum $name {
      $(
        $v
      ),*
    }

    impl ToString for $name {
      fn to_string(&self) -> String {
        match *self {
          $(
            $name::$v => $crate::helpers::to_streaming_snake_case(stringify!($v))
          ),*
        }
      }
    }
  };
}

pub fn to_streaming_snake_case(v: &str) -> String {
  let mut snake = String::new();
  for (i, ch) in v.char_indices() {
    if i > 0 && ch.is_uppercase() {
      snake.push('_');
    }
    snake.push(ch.to_ascii_uppercase());
  }
  snake
}
