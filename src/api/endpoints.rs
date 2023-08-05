use actix_web::{post,get, web,Result, Responder,http::header,HttpRequest,FromRequest};
use log::info;
use serde_json::Value;
use  std::fmt::Display;
use chrono::{Utc,TimeZone};
use crate::models::request_models::*;
use crate::api::extractor_functions::header_extractor;
use crate::repository::database_functions::*;

//
#[post("/test/")]
async fn test(info:web::Json<PlayerLoginModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    return Ok(web::Json("hola"));
    }

//
#[post("/player_creation/")]
async fn player_creation_handler(info:web::Json<PlayerCreationModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "player creation";
    let data = serde_json::to_string(&info).expect("failed to serializer");
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    // json body
    let first_name:String=info.first_name.to_string();
    let last_name:String=info.last_name.to_string();
    let email:String=info.email.to_string();
    let dob:String=info.dob.to_string();
    let password:String=info.password.to_string();
    let max_deposite_limit:i32=info.max_deposite_limit;
    let max_bet_limit:i32=info.max_bet_limit;
    let kyc_id:i32=info.kyc_id;
    let kyc_id_number:String=info.kyc_id_number.to_string();
    let postal_code:String=info.postal_code.to_string();
    //json body
    let result = player_creation_sp(header_value,first_name,last_name,email,dob,password,max_deposite_limit,max_bet_limit,kyc_id,kyc_id_number,postal_code).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            println!("{}",e);
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}


#[post("/player_login/")]
async fn player_login_handler(info:web::Json<PlayerLoginModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "player login";
    let data = serde_json::to_string(&info).expect("failed to serializer");
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    // json body
    let password:String=info.password.to_string();
    let captcha:String=info.captcha.to_string();
    //json body
    let result = player_login_sp(header_value,password,captcha).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}



#[get("/get_balance/")]
async fn get_balance_handler(req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "get balance";
    // request logger....
    info!("{},,,,,{}",req_stamp,method);
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    let result = get_balance_sp(header_value).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}



#[post("/available_games/")]
async fn available_games_handler(info:web::Json<AvailableGamesModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "available games";
    let data = serde_json::to_string(&info).expect("failed to serializer");
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    // json body
    let type_id:i32=info.type_id;
    //json body
    let result = available_games_sp(header_value,type_id).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}


#[post("/payment_init/")]
async fn payment_init_handler(info:web::Json<PaymentInitModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "payment init";
    let data = serde_json::to_string(&info).expect("failed to serializer");
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    // json body
    let amount = info.amount;
    let pg_type_id = info.pg_type_id;
    let pg_txn_id = info.pg_txn_id.to_string();
    let email = info.email.to_string();
    let item_desc = info.item_description.to_string();
    //json body
    let result = payment_init_sp(header_value,amount,pg_type_id,pg_txn_id,email,item_desc).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}


#[post("/addmoney/")]
async fn add_money_handler(info:web::Json<AddMoneyModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "add money";
    let data = serde_json::to_string(&info).expect("failed to serializer");
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    // json body
    let type_id = info.type_id;
    let amount = info.amount;
    let pg_type_id = info.pg_type_id;
    let pg_txn_id = info.pg_txn_id.to_string();
    let email = info.email.to_string();
    let item_desc = info.item_description.to_string();
    //json body
    let result = add_money_sp(header_value,type_id,amount,pg_type_id,pg_txn_id,email,item_desc).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}





#[post("/withdrawmoney/")]
async fn withdraw_money_handler(info:web::Json<WithdrawMoneyModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "add money";
    let data = serde_json::to_string(&info).expect("failed to serializer");
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    // json body
    let type_id = info.type_id;
    let amount = info.amount;
    let pg_type_id = info.pg_type_id;
    let pg_txn_id = info.pg_txn_id.to_string();
    let pg_ref_id = info.pg_ref_id.to_string();
    let pg_data = info.pg_data.to_string();
    let item_desc = info.item_description.to_string();
    //json body
    let result = withdraw_money_sp(header_value,type_id,amount,pg_type_id,pg_txn_id,pg_ref_id,pg_data,item_desc).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}


