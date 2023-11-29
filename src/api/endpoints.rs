use actix_web::{post,get, web,Result, Responder,http::header,HttpRequest,FromRequest};
use log::error;
use log::info;
use serde_json::Value;
use  std::fmt::Display;
use chrono::{Utc,TimeZone};
use std::path::Path;
use std::fs::File;
use crate::models::request_models::*;
use crate::api::extractor_functions::header_extractor;
use crate::repository::database_functions::*;
use crate::api::get_games_function::*;
use reqwest;
use reqwest::Error;
use reqwest::Client;

// ---------------------------------
#[get("/get_version/")]
async fn get_version_handler(req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    
    let parsed: Value = serde_json::from_str("{\"result\":{\"Date\":\"2023-11-29\",\"Version\":\"Version : 1.0.3\"}}")?;
    return Ok(web::Json(parsed)) 
    
}
// ---------------------------------

#[post("/player_creation/")]
async fn player_creation_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<PlayerCreationModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "player creation";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header 
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
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
    let result = player_creation_sp(io_log,req_stamp,header_value,first_name,last_name,email,dob,password,max_deposite_limit,max_bet_limit,kyc_id,kyc_id_number,postal_code).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}


#[post("/player_login/")]
async fn player_login_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<PlayerLoginModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "player login";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let password:String=info.password.to_string();
    let captcha:String=info.captcha.to_string();
    let type_id:i32=info.type_id;
    //json body
    let result = player_login_sp(io_log,req_stamp,header_value,password,captcha,type_id).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            // if io_log ==0{
            //     info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            // }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}



#[get("/get_balance/")]
async fn get_balance_handler(web_config: web::Data<GlobalConfigModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "get balance";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    info!("{},,,,,{}",req_stamp,method);
    //Header Section
    let header_value = header_extractor(req).await?;
    //IO Logging Section
    if io_log ==0{
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ",req_stamp,method,header_value);
    }
    //IO Logging

    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    let result = get_balance_sp(io_log,req_stamp,header_value).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}



#[post("/available_games/")]
async fn available_games_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<AvailableGamesModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "available games";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let type_id:i32=info.type_id;
    //json body
    let result = available_games_sp(io_log,req_stamp,header_value,type_id).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}


#[post("/payment_init/")]
async fn payment_init_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<PaymentInitModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "payment init";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let amount = info.amount;
    let pg_type_id = info.pg_type_id;
    let pg_txn_id = info.pg_txn_id.to_string();
    let email = info.email.to_string();
    let item_desc = info.item_description.to_string();
    //json body
    let result = payment_init_sp(io_log,req_stamp,header_value,amount,pg_type_id,pg_txn_id,email,item_desc).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}


#[post("/addmoney/")]
async fn add_money_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<AddMoneyModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "add money";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let type_id = info.type_id;
    let amount = info.amount;
    let pg_type_id = info.pg_type_id;
    let pg_txn_id = info.pg_txn_id.to_string();
    let email = info.email.to_string();
    let item_desc = info.item_description.to_string();
    //json body
    let result = add_money_sp(io_log,req_stamp,header_value,type_id,amount,pg_type_id,pg_txn_id,email,item_desc).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}

#[post("/withdrawmoney/")]
async fn withdraw_money_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<WithdrawMoneyModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "withdraw money";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let type_id = info.type_id;
    let amount = info.amount;
    let pg_type_id = info.pg_type_id;
    let pg_txn_id = info.pg_txn_id.to_string();
    let pg_ref_id = info.pg_ref_id.to_string();
    let pg_data = info.pg_data.to_string();
    let item_desc = info.item_description.to_string();
    //json body
    let result = withdraw_money_sp(io_log,req_stamp,header_value,type_id,amount,pg_type_id,pg_txn_id,pg_ref_id,pg_data,item_desc).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}

#[post("/otpvalidation/")]
async fn otp_validation_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<OtpValidation>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "otp validation";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let otp = info.otp.to_string();
    //json body
    let result = otp_validation_sp(io_log,req_stamp,header_value,otp).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}

#[post("/otpgeneration/")]
async fn otp_generation_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<OtpGeneration>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "otp generation";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let type_id = info.type_id;
    //json body
    let result = otp_generation_sp(io_log,req_stamp,header_value,type_id).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}

#[get("/getgamefamily/")]
async fn get_games_handler(web_config: web::Data<GlobalConfigModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "get game family";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // let data = serde_json::to_string(&info).expect("failed to serializer");
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ",req_stamp,method,header_value);
    }
    //IO Logging
    // json body
    // let otp = info.otp.to_string();
    //json body
    let result = get_games();
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}

#[post("/getpopulargames/")]
async fn get_fav_games_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<GetFavGamesModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "get poplar games";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let game_group_id = info.game_group_id.to_string();
    //json body
    let result = get_popular_games();
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}




#[get("/getservertime/")]
async fn get_server_time_handler(web_config: web::Data<GlobalConfigModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "get server time";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?}",req_stamp,method,header_value);
    }
    //IO Logging
    // json body
    // let game_group_id = info.game_group_id.to_string();
    //json body
    let result = get_server_time_sp(io_log,req_stamp,header_value).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}



#[post("/getslotgames/")]
async fn get_slot_games_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<GetSlotGames>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "get slot games";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let game_type_id = info.game_type_id;
    //json body
    let result = get_slot_games(game_type_id);
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}


#[get("/getplayerprofile/")]
async fn get_player_profile_handler(web_config: web::Data<GlobalConfigModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "get player profile";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?}",req_stamp,method,header_value);
    }
    //IO Logging
    let result = get_player_profile_sp(io_log,req_stamp,header_value).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }   
}


#[post("/updplayerprofile/")]
async fn upd_player_profile_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<PlayerProfileUpdate>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "update player profile";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // let data = serde_json::to_string(&info).expect("failed to serializer");
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let player_image = info.player_image.to_string();
    let player_name = info.player_name.to_string();
    let email = info.email.to_string();
    let kyc_no = info.kyc_no.to_string();
    let dob = info.dob.to_string();
    //json body
    let result = update_player_profile_sp(io_log,req_stamp,header_value,player_image,player_name,email,kyc_no,dob).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}



#[post("/sellticket/")]
async fn buy_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<BuyModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "sell ticket";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let reflot = info.reflot;
    let group_id = info.group_id;
    let draw_time = info.draw_time.to_string();
    let bet_info = info.bet_info.to_string();
    let client_transid = info.client_transid.to_string();
    let amount = info.amount;
    let type_id = info.type_id;
    
    //json body
    let result = buy_sp(io_log,req_stamp,header_value,reflot,group_id,draw_time,bet_info,client_transid,amount,type_id).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}


#[post("/kycverify/")]
async fn kyc_verification_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<KycVerifyModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "kyc verify";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let player_name = info.player_name.to_string();
    let dob = info.dob.to_string();
    let type_id = info.type_id;
    let nationality = info.nationality.to_string();
    let id_type = info.id_type.to_string();
    let id_no = info.id_no.to_string();
    let address = info.address.to_string();
    let proof = info.proof.to_string();
    let proof2 = info.proof2.to_string();
    
    
    //json body
    let result = kyc_verification_sp(io_log,req_stamp,header_value,type_id,player_name,dob,nationality,id_type,id_no,address,proof,proof2).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}

#[post("/getcurrentresult/")]
async fn get_current_result_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<GetCurrentResult>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "get current result";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let game_groupid = info.game_group_id;
    let draw_time = info.draw_time.to_string();
    
    //json body
    let result = get_current_result_sp(io_log,req_stamp,header_value,game_groupid,draw_time).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}



#[post("/getlatestresult/")]
async fn get_latest_result_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<GetLatestResult>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "get latest result";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let game_groupid = info.game_group_id;
    
    //json body
    let result = get_previous_result_sp(io_log,req_stamp,header_value,game_groupid).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}


#[post("/transactionhistory/")]
async fn transaction_history_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<TransHistoryModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "transaction history";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let from_date = info.from_date.to_string();
    let to_date = info.to_date.to_string();
    let type_id = info.type_id;
    
    //json body
    let result = transaction_history_sp(io_log,req_stamp,header_value,from_date,to_date,type_id).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}


#[post("/playerreports/")]
async fn player_reports_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<PlayerReportModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "player reports";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let from_date = info.from_date.to_string();
    let to_date = info.to_date.to_string();
    let type_id = info.type_id;
    
    //json body
    let result = player_reports_sp(io_log,req_stamp,header_value,from_date,to_date,type_id).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}


#[post("/getresults/")]
async fn result_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<ResultModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "get result";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let date = info.date.to_string();
    let game_group_id = info.game_group_id;
    
    //json body
    let result = result_sp(io_log,req_stamp,header_value,date,game_group_id).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}

#[post("/passwordchange/")]
async fn password_change_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<PasswordModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "password change";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let old_passsword = info.old_password.to_string();
    let new_password = info.new_password.to_string();
    let flag = info.flag;
    
    //json body
    let result = password_change_sp(io_log,req_stamp,header_value,old_passsword,new_password,flag).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}


#[post("/ticketinfo/")]
async fn ticket_info_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<TicketInfoModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "ticket info";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let transaction_id = info.transaction_id.to_string();
    let type_id = info.type_id;
    
    //json body
    let result = ticket_info_sp(io_log,req_stamp,header_value,transaction_id,type_id).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}

#[post("/captchaverify/")]
async fn captcha_verify_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<CaptchaModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "captchaverify";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let security_key = info.secret_key.to_string();
    let captcha = info.recaptcha.to_string();
    let request_url = format!("https://www.google.com/recaptcha/api/siteverify?secret=${security_key}&response=${captcha}",
    security_key = security_key,
    captcha = captcha);
    let response = Client::new()
    .post(request_url)
    .header("Content-Length","0")
    .send().await?;
    let out_res = &response.text().await?;
    let parsed: Value = serde_json::from_str(&out_res)?;
    return Ok(web::Json(parsed));
}


#[post("/getoddsconfigscheme/")]
async fn get_odds_config_scheme_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<OddsConfigSchemeModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "get odds config scheme";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let game_group_id = info.game_group_id;
    
    //json body
    let result = odds_config_scheme_sp(io_log,req_stamp,header_value,game_group_id).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}


#[post("/playerloginimage/")]
async fn player_login_image_handler(web_config: web::Data<GlobalConfigModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "player login image";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    //json body
    let result = player_login_image_sp(header_value).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}



#[post("/getgamewisebetinfo/")]
async fn get_game_wise_bet_info_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<GameWiseBetinfoModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "get odds config scheme";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let game_group_id = info.game_group_id;
    let date_time = info.datetime.to_string();
    
    //json body
    let result = get_game_wise_bet_info_sp(io_log,req_stamp,header_value,game_group_id,date_time).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}

#[post("/get_available_race/")]
async fn get_available_race_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<AvailableRaceModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "get_available_race";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let game_group_id = info.game_group_id;
    
    //json body
    let result = get_available_race_sp(io_log,req_stamp,header_value,game_group_id).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}


#[post("/get_game_race_details/")]
async fn get_game_race_details_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<RaceDetailsModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "get_available_race";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let game_group_id = info.game_group_id;
    let race_id = info.race_id;
    
    //json body
    let result = get_game_race_details_sp(header_value,game_group_id,race_id).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}

#[get("/get_country/")]
async fn get_country_handler(web_config: web::Data<GlobalConfigModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "get country";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    info!("{},,,,,{}",req_stamp,method);
    //Header Section
    let header_value = header_extractor(req).await?;
    //IO Logging Section
    if io_log ==0{
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ",req_stamp,method,header_value);
    }
    //IO Logging

    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    let result = get_country_sp(io_log,req_stamp,header_value).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}

#[post("/deposit_init/")]
async fn deposit_init_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<DepositeInitModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "deposit init";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let type_id = info.type_id;
    let amount = info.amount;
    let pg_type_id = info.pg_type_id;
    let pg_ref_id = info.pg_ref_id.to_string();
    let pg_default = info.pg_default.to_string();
    let pg_item_desc = info.pg_item_desc.to_string();
    let addmoney_type = info.addmoney_type;
    let device_id = info.device_id;
    
    //json body
    let result = deposit_init_sp(io_log,req_stamp,header_value,type_id,amount,pg_type_id,pg_ref_id,pg_default,pg_item_desc,addmoney_type,device_id).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}


#[post("/addmoney_confirm/")]
async fn addmoney_conformation_handler(web_config: web::Data<GlobalConfigModel>,info:web::Json<AddMoneyConformationModel>,req:HttpRequest)-> Result<impl Responder,Box<dyn std::error::Error>>{
    let dt = Utc::now();
    let req_stamp = dt.timestamp() as f64 + dt.timestamp_subsec_nanos() as f64 / 1_000_000_000.0;
    let method = "add money confirm";
    let io_log = web_config.io_log;
    let error_log = web_config.error_log;
    // request logger....
    //Header Section
    let header_value = header_extractor(req).await?;
    // let user_id = req.headers().get("APIKEY").unwrap();
    //Header Section
    //IO Logging Section
    if io_log ==0{
        let data = serde_json::to_string(&info).expect("failed to serializer");
        info!("STAMP : {:?}, REQUEST ,METHOD : {:?}, HEADER : {:?} ,BODY : {:?}",req_stamp,method,header_value,data);
    }
    //IO Logging
    // json body
    let type_id = info.type_id;
    let amount = info.amount;
    let pg_type_id = info.pg_type_id;
    let status = info.status;
    let pg_ref_code = info.pg_ref_code.to_string();
    let pg_txn_id = info.pg_txn_id.to_string();
    let pg_ref_id = info.pg_ref_id.to_string();
    let pg_data = info.pg_data.to_string();
    let item_description = info.item_description.to_string();
    let tax_amount = info.tax_amount.to_string();
    let transaction_commission = info.transaction_commission.to_string();
    let info_string = info.info.to_string();
    
    //json body
    let result = addmoney_confirm_sp(io_log,req_stamp,header_value,type_id,amount,pg_type_id,status,pg_ref_code,pg_txn_id,pg_ref_id,pg_data,item_description,tax_amount,transaction_commission,info_string).await;
    match result {
        Ok(x)=>{
            let j = format!("{{\"result\":{}}}",x);
            let parsed: Value = serde_json::from_str(&j)?;
            if io_log ==0{
                info!("STAMP : {:?}, RESPONSE ,METHOD : {:?} ,BODY : {:?}",req_stamp,method,parsed);
            }
            return Ok(web::Json(parsed));
        }
        Err(e) =>{
            if error_log ==0{
                error!("stamp : {:?}method : {:?},,ERROR : {:?}",req_stamp,method,e);
            }
            let parsed: Value = serde_json::from_str("{\"result\":{\"Status_Id\":1,\"Message\":\"Internal Server Error\"}}")?;
            return Ok(web::Json(parsed)) 
        }
    }
    
}