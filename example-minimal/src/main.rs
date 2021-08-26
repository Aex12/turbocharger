#[cfg(target_arch = "wasm32")]
use turbocharger::console_log;
use turbocharger::{backend, server_only, wasm_only};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
 #[wasm_bindgen(js_namespace = console)]
 fn log(s: &str);
}

#[cfg(target_arch = "wasm32")]
#[allow(non_camel_case_types)]
#[wasm_bindgen]
pub struct wasm_only;

#[cfg(target_arch = "wasm32")]
#[allow(non_camel_case_types)]
#[wasm_bindgen]
pub struct backend;

#[wasm_only]
pub async fn get_local_greeting1() -> String {
 "Hello from WASM one.".to_string()
}

#[wasm_only]
pub async fn get_local_greeting2() -> String {
 "Hello from WASM two.".to_string()
}

#[backend]
pub async fn get_backend_test() -> String {
 "Hello from get_backend_test.".to_string()
}

#[backend]
pub async fn get_backend_test_with_string(name: String) -> String {
 format!("Hello from get_backend_test_with_string, {}!", name).to_string()
}

//#[backend]
#[cfg(not(target_arch = "wasm32"))]
pub async fn get_remote_greeting(name: String) -> String {
 eprintln!("get_remote_greeting called");
 tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
 format!("Hello from backend, {}!", name).to_string()
}

#[cfg(not(target_arch = "wasm32"))]
mod _tc_get_remote_greeting {
 use ::turbocharger::typetag;
 #[::turbocharger::typetag::serde(name = "get_remote_greeting")]
 #[::turbocharger::async_trait]
 impl ::turbocharger::RPC for super::_tc_dispatch_get_remote_greeting {
  async fn execute(&self) -> Vec<u8> {
   let response = super::_tc_resp_get_remote_greeting {
    txid: self.txid,
    result: super::get_remote_greeting(self.params.0.clone()).await,
   };
   ::turbocharger::bincode::serialize(&response).unwrap()
  }
 }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_class = backend)]
impl backend {
 #[wasm_bindgen]
 pub async fn get_remote_greeting(name: String) -> String {
  {
   let t = ::turbocharger::_Transaction::new();
   let txid = t.txid;
   let req = ::turbocharger::bincode::serialize(&_tc_req_get_remote_greeting {
    typetag_const_one: 1,
    dispatch_name: "get_remote_greeting",
    txid: t.txid,
    params: (name,),
   })
   .unwrap();
   let response = t.run(req).await;
   let _tc_resp_get_remote_greeting { result, .. } =
    ::turbocharger::bincode::deserialize(&response).unwrap();
   console_log!("tx {}: {:?}", txid, result);
   result
  }
 }
}

#[allow(non_camel_case_types)]
#[derive(::turbocharger::serde::Serialize, ::turbocharger::serde::Deserialize)]
#[serde(crate = "::turbocharger::serde")]
struct _tc_req_get_remote_greeting {
 typetag_const_one: i64,
 dispatch_name: &'static str,
 txid: i64,
 params: (String,),
}

#[allow(non_camel_case_types)]
#[derive(::turbocharger::serde::Serialize, ::turbocharger::serde::Deserialize, Debug)]
#[serde(crate = "::turbocharger::serde")]
struct _tc_dispatch_get_remote_greeting {
 txid: i64,
 params: (String,),
}

#[allow(non_camel_case_types)]
#[derive(::turbocharger::serde::Serialize, ::turbocharger::serde::Deserialize)]
#[serde(crate = "::turbocharger::serde")]
struct _tc_resp_get_remote_greeting {
 txid: i64,
 result: String,
}

#[server_only]
#[tokio::main]
async fn main() {
 let event: &dyn ::turbocharger::RPC =
  &_tc_dispatch_get_remote_greeting { txid: 42, params: ("foo".to_string(),) };
 let b = ::turbocharger::bincode::serialize(&event).unwrap();
 println!("{:?}", b);

 let event = _tc_req_get_remote_greeting {
  typetag_const_one: 1,
  dispatch_name: "get_remote_greeting",
  txid: 42,
  params: ("foo".to_owned(),),
 };
 let b = ::turbocharger::bincode::serialize(&event).unwrap();
 println!("{:?}", b);

 eprintln!("Serving on http://127.0.0.1:8080");
 warp::serve(turbocharger::warp_routes()).run(([127, 0, 0, 1], 8080)).await;
}
