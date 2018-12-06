extern crate bestbuy;
extern crate chrono;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate clap;

mod helpers;

macro_rules! dispatch {
  ($matches:expr => $head:tt $($rest:tt)*) => {
    dispatch!(ITEM $matches, $head);
    dispatch!($matches => $($rest)*);
  };

  ($matches:expr => ) => {};

  (ITEM $matches:expr, ($handler:expr)) => {
    ($handler as fn(&clap::ArgMatches))(&$matches)
  };

  (ITEM $matches:expr, ($cmd:ident => $($sub:tt)+)) => {
    if let Some(matches) = $matches.subcommand_matches(stringify!($cmd)) {
      dispatch!(matches => $($sub)*);
    }
  };
}

fn main() {
  ::dotenv::dotenv().unwrap();

  let matches = clap_app!(myapp =>
    (@subcommand order =>
      (about: "Manage orders")
      (@subcommand list_orders =>
      )
      (@subcommand test_orders =>
        (@arg FILE: +required "JSON file contains an order array.")
      )
      (@subcommand inspect_order =>
        (about: "Display order items and statuses")
        (@arg ORDER_ID: +required "Bestbuy order id")
      )
      (@subcommand ship =>
        (about: "ship order")
        (@arg ORDER_ID: +required "Sets the order id")
        (@arg carrier_code: -c --carrier_code +takes_value "Sets the carrier code")
        (@arg tracking_number: -t --tracking_number +takes_value "Sets the tracking number")
        (@arg tracking_url: -u --tracking_url +takes_value "Sets the tracking url")
      )
    )
    (@subcommand offer =>
      (about: "Manage offers")
      (@subcommand dump =>
      )
      (@subcommand deserialize_all =>
        (@arg FILE: +required "File path")
      )
    )
  )
  .get_matches();

  dispatch! {
    matches =>
      (order =>
        (list_orders =>
          (|_| {
            use bestbuy::order::*;
            use chrono::{Utc, Duration};
            let client = helpers::get_client();
            let mut params = ListOrdersParams::default();
            params.start_date = Some(Utc::now() - Duration::days(7));
            helpers::dump_json(client.list_orders(
              &params,
              None,
              None,
            ).unwrap())
          })
        )

        (test_orders =>
          (|m| {
            use std::fs::{self, File};
            use serde_json::Value;
            let path = m.value_of("FILE").unwrap();

            println!("Loading json file: {}", path);

            let file = File::open(path).unwrap();
            let items: Vec<Value> = serde_json::from_reader(file).unwrap();

            println!("Items: {}", items.len());

            for (i, item) in items.into_iter().enumerate() {
              let text = serde_json::to_string_pretty(&item).unwrap();
              fs::write("last_order.json", &text).unwrap();

              println!("Testing {}...", i);
              serde_json::from_str::<::bestbuy::order::Order>(&text).unwrap();
            }

            println!("OK.");

            fs::remove_file("last_order.json").unwrap();
          })
        )

        (inspect_order =>
          (|m| {
            use bestbuy::order::*;
            let order_id = m.value_of("ORDER_ID").unwrap();
            let client = helpers::get_client();
            let mut params = ListOrdersParams::default();
            params.order_ids = Some(vec![order_id.to_string()]);
            helpers::inspect_order(client.list_orders(
              &params,
              None,
              None,
            ).unwrap().orders.pop().unwrap())
          })
        )

        (ship =>
          (|m| {
            use bestbuy::order::*;
            let order_id = m.value_of("ORDER_ID").unwrap();
            let client = helpers::get_client();
            let carrier_code = m.value_of("carrier_code").and_then(|code| {
              match code.trim().to_lowercase().as_ref() {
                "canada post" => Some(CarrierCode::CPCL),
                "purolator" => Some(CarrierCode::PRLA),
                "ups" => Some(CarrierCode::UPSN),
                "fedex" => Some(CarrierCode::FEDX),
                "dhl" => Some(CarrierCode::DHL),
                _ => None,
              }
            });
            let (carrier_name, carrier_url) = if let None = carrier_code.as_ref() {
              (
                m.value_of("carrier_code").map(ToString::to_string),
                m.value_of("tracking_url").map(ToString::to_string)
              )
            } else {
              (None, None)
            };
            let t = OrderTracking {
              carrier_code,
              carrier_name,
              carrier_url,
              tracking_number: m.value_of("tracking_number").map(ToString::to_string),
            };
            client.update_tracking(&order_id, &t).expect("update_tracking");
            client.ship(&order_id).expect("ship");
          })
        )
      )
      (offer =>
        (dump =>
          (|_| {
            use bestbuy::client::Method;
            use serde_json::{self, Value};
            let client = helpers::get_client();

            let mut items = vec![];
            loop {
              let mut res = client.request(Method::Get, &format!(
                 "/api/offers?max=100&offset={}", items.len()
              )).send().unwrap();
              let value: serde_json::Value = res.json().unwrap();
              let mut page_items: Vec<Value> = value.as_object()
                .and_then(|v| v.get("offers"))
                .and_then(|v| v.as_array())
                .unwrap()
                .clone();
              if page_items.is_empty() {
                break
              }
              items.append(&mut page_items);
            }
            serde_json::to_writer_pretty(::std::io::stdout(), &items).unwrap();
          })
        )

        (deserialize_all =>
          (|m| {
            use bestbuy::offer::Offer;
            use std::fs::File;
            use serde_json::{self, Value};
            let path = m.value_of("FILE").unwrap();
            let file = File::open(path).unwrap();
            let values: Vec<Value> = serde_json::from_reader(file).unwrap();
            let len = values.len();
            for (i, v) in values.into_iter().enumerate() {
              println!("testing {} of {}...", i + 1, len);
              serde_json::from_value::<Offer>(v.clone()).unwrap();
            }
          })
        )
      )
  }
}
