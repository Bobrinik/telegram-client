use error_chain_mini::ErrorKind;
use rtdlib::types::*;

use crate::api::Api;
use crate::errors::{TGError, TGErrorKind, TGResult};
use crate::handler::*;
use crate::listener::Lout;
use crate::tglog;
use crate::types as tg_type;

pub struct ReceiveHandler<'a> {
  api: &'a Api,
  lout: &'a Lout,
}

impl<'a> ReceiveHandler<'a> {
  pub fn new(api: &'a Api, lout: &'a Lout) -> Self {
    Self { api, lout }
  }

  pub fn handle(&self, object: &'a Box<Object>) -> TGResult<()> {
    let td_type = object.td_type();
//    debug!(tglog::telegram(), "Receive td type => {:?}", td_type);

    let rtdname = object.td_name();
    let rtdtype = RTDType::of(rtdname);
    if rtdtype.is_none() {
      return Ok(());
    }
    let json = object.to_json();
    let rtdtype = rtdtype.unwrap();

//    /// auto generate trait handler,
//    /// # Fields
//    /// - `$name` Current update tdtype struct
//    /// - `$trait_fname` this struct sub trait function name
//    /// - `$td_type` sub trait tdtype enum name
//    /// - `$type_item` sub trait tdtype enum item name
//    /// - `$listen_fname` sub trait tdtype => listen function name
//    macro_rules! handler_trait {
//      ($name:ident, $trait_fname:ident, $td_type:ident, ($(($type_item:ident, $listen_fname:ident));*;)) => {{
//        let update = $name::from_json(json).ok_or(TGErrorKind::RTDLibFromError.into_err())?;
//        match update.$trait_fname() {
//          Some(t) => match $td_type::of(t.td_name()) {
//            $(
//              Some($td_type::$type_item) => {
//                if let Some(fnc) = self.lout.$listen_fname() {
//                  (*fnc)((self.api, &rtdlib::types::$type_item::from_json(t.to_json()).unwrap()))
//                }
//              }
//            )*
//            None => {}
//          }
//          None => {}
//        }
//      }}
//    }
//        handler_trait!(UpdateOption, value, RTDOptionValueType, (
//          (OptionValueBoolean, option_bool);
//          (OptionValueEmpty, option_empty);
//          (OptionValueInteger, option_integer);
//          (OptionValueString, option_string);
//        ))


    macro_rules! handler_sec {
      ($obj:tt, $listen_fname:ident) => {{
        match self.lout.$listen_fname() {
          Some(fnc) => match tg_type::$obj::from_json(&json) {
            Some(t) => (*fnc)((self.api, &t)),
            None => warn!(tglog::telegram(), "Json fail => {}", json)
          }
          None => warn!(tglog::telegram(), "Not found handler for {}", rtdname)
        }
      }}
    }

    let r = match rtdtype {
      RTDType::UpdateOption => handler_sec!(TGUpdateOption, option),
      RTDType::UpdateAuthorizationState => handler_sec!(TGAuthorizationState, authorization_state),
      _ => ()
    };
    Ok(r)
  }
}



