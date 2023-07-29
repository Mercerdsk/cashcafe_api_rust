use actix_web::{post, web,Result, Responder,http::header,HttpRequest,FromRequest};
use serde_json::Value;
use  std::fmt::Display;
use chrono::{Utc,TimeZone};
use crate::models::request_models::*;
use crate::repository::database_functions::player_creation_sp;

#[post("/player_creation/")]
async fn player_creation_handler(info:web::Json<input_json>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "player creation";
    let data = serde_json::to_string(&info).expect("failed to serializer");
    // request logger....
    // let user_id:String=info.user_id.to_string();
    // let first_name:String=info.first_name.to_string();
    // let last_name:String=info.last_name.to_string();
    // let email:String=info.email.to_string();
    // let dob:String=info.dob.to_string();
    // let password:String=info.password.to_string();
    // let max_deposite_limit:i32=info.max_deposite_limit;
    // let max_bet_limit:i32=info.max_bet_limit;
    // let kyc_id:i32=info.kyc_id;
    // let kyc_id_number:String=info.kyc_id_number.to_string();
    // let postal_code:String=info.postal_code.to_string();
    let result = player_creation_sp().await;
    match result {
        Some(x)=>{
            let j = serde_json::to_string(&x)?;
            let parsed: Value = serde_json::from_str(&j)?;
            return Ok(web::Json(parsed));
        }
        None =>{
            let parsed: Value = serde_json::from_str("{\"resid\":1,\"resdesc\":\"Internal Server Error\"}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}