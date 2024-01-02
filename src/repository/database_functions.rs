use std::default;

use log::info;
use serde_json::json;

use crate::repository::database_connection::db_connection;
use crate::models::request_models::*;
use crate::models::response_models::*;
use crate::repository::rabbitmq_publisher::queue_publisher;
use crate::repository::sms_email_function::*;


pub async fn player_creation_sp(IO_LOG:i32,req_stamp:f64,sms_email_url:String,header_value:HeaderModel,first_name:String,last_name:String,email:String,dob:String,password:String,max_deposite_limit:i32,max_bet_limit:i32,kyc_id:i32,kyc_id_number:String,postal_code:String)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_INS_PlayerRegistration '{}',{},'{}','{}','{}',{},'{}','{}','{}','{}','{}','{}',{},{},{},'{}','{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,first_name,last_name,email,dob,password,max_deposite_limit,max_bet_limit,kyc_id,kyc_id_number,postal_code);
    //println!("{:?}",&qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
        let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
        let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("");
        let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
        let out_json = PlayerCreationResponse{
            TVN:String::from(tvn),
            Status_id:String::from(status_id),
            Message:String::from(message),
        };
        let json_string = serde_json::to_string(&out_json)?;
        if res_value.len()==2{
            let sms_email_info:&str=res_value[1][0].try_get(0)?.unwrap_or("null");
            let sms_mail_result = sms_email_function(sms_email_info.to_string(),sms_email_url).await;
                match sms_mail_result{
                    Ok(x)=>{info!("sms_email_api success")},
                    Err(e)=>{info!("sms_email_api")}
                }
        }
    return Ok(json_string); 
    }


pub async fn player_login_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel,password:String,captcha:String,type_id:i32)->Result<String,Box<dyn std::error::Error>>{
    let mut client: tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>> = db_connection().await?;
    let qry = format!("EXEC CLI_PlayerLogin '{}',{},'{}','{}','{}',{},'{}','{}','{}','{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,password,captcha,type_id);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    // if IO_LOG ==0{
    //     info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    // }
        let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
        let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
        let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
        if status_id != '0'.to_string(){
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
        else {
            let balance:&str=res_value[1][0].try_get("Balance")?.unwrap_or("");
            let win_balance:&str=res_value[1][0].try_get("Win_Balance")?.unwrap_or("");
            let session_id:&str=res_value[1][0].try_get("Session_Id")?.unwrap_or("");
            let user_name:&str=res_value[1][0].try_get("User_Name")?.unwrap_or("");
            let date_time:&str=res_value[1][0].try_get("Date_Time")?.unwrap_or("");
            let imageinfo:&str=res_value[1][0].try_get("ImageInfo")?.unwrap_or("");
            let created_date:&str=res_value[1][0].try_get("CreateDate")?.unwrap_or("");
            let timeout_seconds:&str=res_value[1][0].try_get("TimoutSeconds")?.unwrap_or("");
            let player_name:&str=res_value[1][0].try_get("playername")?.unwrap_or("");
            let password_status:i32=res_value[1][0].try_get("PasswordStatus")?.unwrap_or_default();
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message,
                "Balance":balance,
                "Win_balance":win_balance,
                "Session_id":session_id,
                "User_name":user_name,
                "Date_time":date_time,
                "Imageinfo":imageinfo,
                "Created_date":created_date,
                "player_name":player_name,
                "timeout_seconds":timeout_seconds,
                "password_status":password_status
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
    }

pub async fn get_balance_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_GET_Balance '{}',{},'{}','{}','{}',{},'{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
        let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
        let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
        let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
        if status_id != '0'.to_string(){
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
        else {
            let balance:i64=res_value[1][0].try_get(0)?.unwrap_or(0);
            let win_balance:i64=res_value[1][0].try_get(1)?.unwrap_or(0);
            let date_time:&str=res_value[1][0].try_get(2)?.unwrap_or("");
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message,
                "Balance":balance,
                "Win_balance":win_balance,
                "Date_time":date_time
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
    }


pub async fn available_games_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel,type_id:i32)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_GET_AvailableGames '{}',{},'{}','{}','{}',{},'{}',{}",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,type_id);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
        let status_id:&str = res_value[0][0].try_get(1)?.unwrap_or("null");
        let tvn:&str = res_value[0][0].try_get(0)?.unwrap_or("null");
        let message:&str = res_value[0][0].try_get(2)?.unwrap_or("null");
        let date_time:&str = res_value[0][0].try_get(3)?.unwrap_or("null");
        let mut out_put:Vec<AvailableGamesTable> = Vec::new();
        if status_id != '0'.to_string(){
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
        else {
            for i in &res_value[1]{
                let out_struct:AvailableGamesTable= AvailableGamesTable { reflot_id:String::from(i.try_get(0)?.unwrap_or("null")),
                    group_id: String::from(i.try_get(1)?.unwrap_or("null")),
                    group_name: String::from(i.try_get(2)?.unwrap_or("null")),
                    game_name: String::from(i.try_get(3)?.unwrap_or("null")),
                    draw_date: String::from(i.try_get(4)?.unwrap_or("null")), 
                    draw_time: String::from(i.try_get(5)?.unwrap_or("null")), 
                    close_time: String::from(i.try_get(6)?.unwrap_or("null")), 
                    interval:String::from(i.try_get(7)?.unwrap_or("null")),
                    end_time:String::from(i.try_get(8)?.unwrap_or("null")),
                    min_max_multi:String::from(i.try_get(9)?.unwrap_or("null")),
                    odds:String::from(i.try_get(10)?.unwrap_or("null")),
                    no_of_balls:String::from(i.try_get(11)?.unwrap_or("null")),
                };
                out_put.push(out_struct);
            }
            
            // let out_json = json!({
            //     "TVN":tvn,
            //     "Status_id":status_id,
            //     "Message":message,
            //     "Balance":balance,
            //     "Win_balance":win_balance,
            //     "Date_time":date_time
            // });
            let json_string = serde_json::to_string(&out_put)?;
            return Ok(json_string);
        }
    }



pub async fn payment_init_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel,amount:i64,pg_type_id:i32,pg_txn_id:String,email:String,item_des:String)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_PayMentInitRequest '{}',{},'{}','{}','{}',{},'{}',{},{},'{}','{}','{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,amount,pg_type_id,pg_txn_id,email,item_des);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
        let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
        let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
        let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
        if status_id != '0'.to_string(){
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
        else {
            let pg_ref_id:&str=res_value[1][0].try_get(0)?.unwrap_or("");
            let callback_url:&str=res_value[1][0].try_get(1)?.unwrap_or("");
            let date_time:&str=res_value[1][0].try_get(2)?.unwrap_or("");
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message,
                "PG_Ref_Id":pg_ref_id,
                "Callback_url":callback_url,
                "Date_time":date_time
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
    }

    
pub async fn add_money_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel,type_id:i32,amount:i64,pg_type_id:i32,pg_txn_id:String,email:String,item_des:String)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_AddMoneyRequest '{}',{},'{}','{}','{}',{},'{}',{},{},{},'{}','{}','{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,type_id,amount,pg_type_id,pg_txn_id,email,item_des);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
        let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
        let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
        let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
        if status_id != '0'.to_string(){
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
        else {
            let pg_txn_id:&str=res_value[1][0].try_get(0)?.unwrap_or("");
            let balance:i64=res_value[1][0].try_get(1)?.unwrap_or(0);
            let win_balance:i64=res_value[1][0].try_get(2)?.unwrap_or(0);
            let date_time:&str=res_value[1][0].try_get(3)?.unwrap_or("");
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message,
                "PG_TXN_Id":pg_txn_id,
                "Balance":balance,
                "Win_Balance":win_balance,
                "Date_time":date_time
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
    }



pub async fn withdraw_money_sp(IO_LOG:i32,req_stamp:f64,sms_email_url:String,header_value:HeaderModel,type_id:i32,amount:i64,pg_type_id:i32,pg_txn_id:String,pg_ref_id:String,pg_data:String,item_des:String)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_WithDrawRequest   '{}',{},'{}','{}','{}',{},'{}',{},{},{},'{}','{}','{}','{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,type_id,amount,pg_type_id,pg_txn_id,pg_ref_id,pg_data,item_des);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
        let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
        let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
        let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
        if status_id != '0'.to_string(){
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message
            });
            let json_string = serde_json::to_string(&out_json)?;
            if res_value.len()==2{
                let sms_email_info:&str=res_value[1][0].try_get(0)?.unwrap_or("null");
                let sms_mail_result = sms_email_function(sms_email_info.to_string(),sms_email_url).await;
                match sms_mail_result{
                    Ok( _x)=>{info!("sms_email_api success")},
                    Err(_e)=>{info!("sms_email_api")}
                }
            }
            return Ok(json_string);
        }
        else {
            let pg_txn_id:&str=res_value[1][0].try_get(0)?.unwrap_or("");
            let balance:i64=res_value[1][0].try_get(1)?.unwrap_or(0);
            let win_balance:i64=res_value[1][0].try_get(2)?.unwrap_or(0);
            let date_time:&str=res_value[1][0].try_get(3)?.unwrap_or("");
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message,
                "PG_TXN_Id":pg_txn_id,
                "Balance":balance,
                "Win_balance":win_balance,
                "Date_time":date_time
            });
            let json_string = serde_json::to_string(&out_json)?;
            if res_value.len()==3{
                let sms_email_info:&str=res_value[2][0].try_get(0)?.unwrap_or("null");
                let sms_mail_result = sms_email_function(sms_email_info.to_string(),sms_email_url).await;
                match sms_mail_result{
                    Ok(x)=>{info!("sms_email_api success")},
                    Err(e)=>{info!("sms_email_api")}
                }
            }
            return Ok(json_string);
        }
    }

    
pub async fn otp_validation_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel,otp:String)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_OTPValidate '{}',{},'{}','{}','{}',{},'{}','{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,otp);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
    let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
    let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
    let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
    if status_id != '2'.to_string(){
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message
        });
        let json_string = serde_json::to_string(&out_json)?;
        return Ok(json_string);
    }
    else {
        let balance:&str=res_value[1][0].try_get("Balance")?.unwrap_or("");
        let win_balance:&str=res_value[1][0].try_get("Win_Balance")?.unwrap_or("");
        let session_id:&str=res_value[1][0].try_get("Session_Id")?.unwrap_or("");
        let user_name:&str=res_value[1][0].try_get("User_Name")?.unwrap_or("");
        let date_time:&str=res_value[1][0].try_get("Date_Time")?.unwrap_or("");
        let imageinfo:&str=res_value[1][0].try_get("ImageInfo")?.unwrap_or("");
        let created_date:&str=res_value[1][0].try_get("CreateDate")?.unwrap_or("");
        let timeout_seconds:&str=res_value[1][0].try_get("TimoutSeconds")?.unwrap_or("");
        let player_name:&str=res_value[1][0].try_get("playername")?.unwrap_or("");
        let password_status:i32=res_value[1][0].try_get("PasswordStatus")?.unwrap_or_default();
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message,
            "Balance":balance,
            "Win_balance":win_balance,
            "Session_id":session_id,
            "User_name":user_name,
            "Date_time":date_time,
            "Imageinfo":imageinfo,
            "Created_date":created_date,
            "player_name":player_name,
            "timeout_seconds":timeout_seconds,
            "password_status":password_status
        });
        let json_string = serde_json::to_string(&out_json)?;
        return Ok(json_string);
    }
    }


pub async fn otp_generation_sp(IO_LOG:i32,req_stamp:f64,sms_email_url: String,header_value:HeaderModel,type_id:i32)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_OTPGeneration '{}',{},'{}','{}','{}',{},'{}',{}",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,type_id);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
        let status_id:&str = res_value[0][0].try_get(1)?.unwrap_or("null");
        let tvn:&str = res_value[0][0].try_get(0)?.unwrap_or("null");
        let message:&str = res_value[0][0].try_get(2)?.unwrap_or("null");
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message
        });
        let json_string = serde_json::to_string(&out_json)?;
        if res_value.len()==2{
            let sms_email_info:&str=res_value[1][0].try_get(0)?.unwrap_or("null");
            let sms_mail_result = sms_email_function(sms_email_info.to_string(),sms_email_url).await;
                match sms_mail_result{
                    Ok(x)=>{info!("sms_email_api success")},
                    Err(e)=>{info!("sms_email_api")}
                }
        }
        return Ok(json_string);
    }

pub async fn get_server_time_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_GET_ServerTime '{}',{},'{}','{}','{}',{},'{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
        let date_time:&str = res_value[0][0].try_get(0)?.unwrap_or("null");
        let out_json = json!({
            "Status_id":"0",
            "Date_time":date_time
        });
        let json_string = serde_json::to_string(&out_json)?;
        return Ok(json_string);
    }


pub async fn get_player_profile_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_GET_Profile '{}',{},'{}','{}','{}',{},'{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
        let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
        let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
        let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
        if status_id != '0'.to_string(){
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
        else {
            let player_image:&str=res_value[1][0].try_get("PlayerImage")?.unwrap_or("");
            let phone_no:&str=res_value[1][0].try_get("PhoneNo")?.unwrap_or("");
            let player_name:&str=res_value[1][0].try_get("PlayerName")?.unwrap_or("");
            let email:&str=res_value[1][0].try_get("EMAIL")?.unwrap_or("");
            let kyc_no:&str=res_value[1][0].try_get("KYCNo")?.unwrap_or("");
            let dob:&str=res_value[1][0].try_get("DOB")?.unwrap_or("");
            let kyc_status:i32 = res_value[1][0].try_get("Status")?.unwrap_or(0);
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message,
                "player_image":player_image,
                "phone_no":phone_no,
                "player_name":player_name,
                "email":email,
                "kyc_no":kyc_no,
                "dob":dob,
                "kyc_status":kyc_status
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
    }



pub async fn update_player_profile_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel,player_image:String,player_name:String,email:String,kyc_no:String,dob:String)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_UPD_PlayerProfile '{}',{},'{}','{}','{}',{},'{}','{}','{}','{}','{}','{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,player_image,player_name,email,kyc_no,dob);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
        let status_id:&str = res_value[0][0].try_get(1)?.unwrap_or("null");
        let tvn:&str = res_value[0][0].try_get(0)?.unwrap_or("null");
        let message:&str = res_value[0][0].try_get(2)?.unwrap_or("null");
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message
        });
        let json_string = serde_json::to_string(&out_json)?;
        return Ok(json_string);
    }

    
pub async fn buy_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel,reflotid:i32,group_id:i32,draw_time:String,bet_info:String,client_transid:String,amount:i64,type_id:i32)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC TRA_OnelineSales '{}',{},'{}','{}','{}',{},'{}',{},{},'{}','{}','{}','{}',{}",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,reflotid,group_id,draw_time,bet_info,client_transid,amount,type_id);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
        let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
        let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
        let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
        if status_id != '0'.to_string(){
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
        else {
            let client_trans_id:&str=res_value[1][0].try_get("Client_TransId")?.unwrap_or("");
            let trans_id:&str=res_value[1][0].try_get("TransId")?.unwrap_or("");
            let print_info:&str=res_value[1][0].try_get("PrintInfo")?.unwrap_or("");
            let date_time:&str=res_value[1][0].try_get("Date_Time")?.unwrap_or("");
            let balance:&str=res_value[1][0].try_get("Balance")?.unwrap_or("null");
            let win_balance:&str=res_value[1][0].try_get("Win_Balance")?.unwrap_or("null");
            
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message,
                "client_transaction_id":client_trans_id,
                "transaction_id":trans_id,
                "print_info":print_info,
                "date_time":date_time,
                "Balance":balance,
                "Win_Balance":win_balance
            });
            let json_string = serde_json::to_string(&out_json)?;
            //println!("{}",json_string);
            return Ok(json_string);
        }
    }


pub async fn kyc_verification_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel,type_id:i32,player_name:String,dob:String,nationality:String,id_type:String,id_no:String,address:String,proof:String,proof2:String)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_GET_KYCVerifiCation '{}',{},'{}','{}','{}',{},'{}','{}','{}','{}','{}','{}','{}','{}','{}','{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,type_id,player_name,dob,nationality,id_type,id_no,address,proof,proof2);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
        let status_id:&str = res_value[0][0].try_get(1)?.unwrap_or("null");
        let tvn:&str = res_value[0][0].try_get(0)?.unwrap_or("null");
        let message:&str = res_value[0][0].try_get(2)?.unwrap_or("null");
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message
        });
        let json_string = serde_json::to_string(&out_json)?;
        return Ok(json_string);
    }

pub async fn get_current_result_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel,game_group_id:i32,draw_time:String)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_GET_Currentresult '{}',{},'{}','{}','{}',{},'{}',{},'{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,game_group_id,draw_time);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
    let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
    let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
    let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
    if status_id != '0'.to_string(){
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message
        });
        let json_string = serde_json::to_string(&out_json)?;
        return Ok(json_string);
    }
    else {
        let draw_no :&str=res_value[1][0].try_get("DrawNo")?.unwrap_or("");
        let draw_date :&str=res_value[1][0].try_get("DrawDate")?.unwrap_or("");
        let draw_time :&str=res_value[1][0].try_get("DrawTime")?.unwrap_or("");
        let win_nods :&str=res_value[1][0].try_get("WinNos")?.unwrap_or("");
        
        
        
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message,
            "draw_no":draw_no,
            "draw_date":draw_date,
            "draw_time":draw_time,
            "win_nos":win_nods
            
        });
        let json_string = serde_json::to_string(&out_json)?;
        //println!("{}",json_string);
        return Ok(json_string);
    }
    }

pub async fn get_previous_result_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel,game_group_id:i32)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_GET_PreviousDrawResult '{}',{},'{}','{}','{}',{},'{}',{}",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,game_group_id);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
    let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
    let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
    let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
    let mut out_json:Vec<LastResultModel> = Vec::new();
    if status_id != '0'.to_string(){
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message
        });
        let json_string = serde_json::to_string(&out_json)?;
        return Ok(json_string);
    }
    else {
            for i in &res_value[1]{
                let out=LastResultModel{
                    draw_no:String::from(i.try_get("DrawNo")?.unwrap_or("")),
                    draw_date:String::from(i.try_get("DrawDate")?.unwrap_or("")),
                    draw_time:String::from(i.try_get("DrawTime")?.unwrap_or("")),
                    win_nods:String::from(i.try_get("WinNos")?.unwrap_or(""))
                };
                out_json.push(out);
            }    
        let json_string = serde_json::to_string(&out_json)?;
        //println!("{}",json_string);
        return Ok(json_string);
    }
    }


pub async fn transaction_history_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel,from_date:String,to_date:String,type_id:i32)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_GET_TransactionDetails '{}',{},'{}','{}','{}',{},'{}','{}','{}',{}",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,from_date,to_date,type_id);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
    let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
    let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
    let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
    if status_id != '0'.to_string(){
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message
        });
        let json_string = serde_json::to_string(&out_json)?;
        return Ok(json_string);
    }
    else {
        let mut output_vec:Vec<TransType1Model> = Vec::new();
        if type_id==1{
            for i in &res_value[1]{
            let out_json:TransType1Model= TransType1Model{
                transaction_id:String::from(i.try_get("Transaction_ID")?.unwrap_or("")),
                draw_date_time:String::from(i.try_get("Draw_Date_Time")?.unwrap_or("")),
                transaction_date_time:String::from(i.try_get("Transaction_Date_Time")?.unwrap_or("")),
                amount:String::from(i.try_get("Amount")?.unwrap_or("")),
                transaction_status:String::from(i.try_get("Status")?.unwrap_or("")),
                win_amount:String::from(i.try_get("WinAmount")?.unwrap_or("")),
                result:String::from(i.try_get("Result")?.unwrap_or("")),
                game_name:String::from(i.try_get("GameName")?.unwrap_or("")),
                
            };
            output_vec.push(out_json);
            }
            
            let json_string = serde_json::to_string(&output_vec)?;
            //println!("{}",json_string);
            return Ok(json_string);
        }
        else if type_id==2 || type_id==3 {
            let mut output_vec:Vec<TransType23Model> = Vec::new();
            for i in &res_value[1]{
                let out_json:TransType23Model= TransType23Model{
                    transaction_id:String::from(i.try_get("Transaction_ID")?.unwrap_or("")),
                    transaction_date_time:String::from(i.try_get("Transaction_Date_Time")?.unwrap_or("")),
                    amount:String::from(i.try_get("Amount")?.unwrap_or("")),
                    transaction_status:String::from(i.try_get("Status")?.unwrap_or(""))
                    
                };
                output_vec.push(out_json);
            }
            
            let json_string = serde_json::to_string(&output_vec)?;
            //println!("{}",json_string);
            return Ok(json_string);
        }
        else {
            let out_json = json!({
                "TVN":tvn,
                "Status_id":"1",
                "Message":"Invalid type_id"
                
            });
            let json_string = serde_json::to_string(&out_json)?;
            //println!("{}",json_string);
            return Ok(json_string);
        }
        
    }
    }

pub async fn player_reports_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel,from_date:String,to_date:String,type_id:i32)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_GET_PlayerReports  '{}',{},'{}','{}','{}',{},'{}','{}','{}',{}",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,from_date,to_date,type_id);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
    let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
    let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
    let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
    
    if status_id != '0'.to_string(){
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message
        });
        let json_string = serde_json::to_string(&out_json)?;
        return Ok(json_string);
    }
    else {
        if &res_value.len().to_string()=="1"{
            let out ="[]".to_string();
            return Ok(out);
        }
        else{
            let mut json_array:Vec<PlayerReportsResponse>=Vec::new();
            if type_id==4{
            for i in &res_value[1]{
                let out_json:PlayerReportsResponse=PlayerReportsResponse {
                    from_date: String::from(i.try_get("FromDate")?.unwrap_or("")),
                    to_date:String::from(i.try_get("ToDate")?.unwrap_or("")), 
                    bet_amount:String::from(i.try_get("BetAmount")?.unwrap_or("")), 
                    win_amount:String::from(i.try_get("WinAmount")?.unwrap_or("")), 
                    add_money:String::from(i.try_get("Addmoney")?.unwrap_or("")), 
                    withdraw_money:String::from(i.try_get("WithDraw")?.unwrap_or("")), 
                    bonus_amount:String::from(i.try_get("BonusAmount")?.unwrap_or("")), 
                    net_amount:String::from(i.try_get("NetAmount")?.unwrap_or("")) };
                json_array.push(out_json);
            }
            let json_string = serde_json::to_string(&json_array)?;
            //println!("{}",json_string);
            return Ok(json_string);
            }
            else{
                let mut json_array:Vec<PlayerActivityResponse>=Vec::new();
                for i in &res_value[1]{
                    let out_json:PlayerActivityResponse=PlayerActivityResponse { 
                        transaction_id: String::from(i.try_get("TransactionID")?.unwrap_or("")),
                        type_desc:String::from(i.try_get("TypeDesc")?.unwrap_or("")), 
                        trans_date_time:String::from(i.try_get("TransDateTime")?.unwrap_or("")), 
                        opening_balance:String::from(i.try_get("OpeningBalance")?.unwrap_or("")),                                    
                        amount:String::from(i.try_get("Amount")?.unwrap_or("")), 
                        closing_balance:String::from(i.try_get("ClosingBalance")?.unwrap_or("")),
                        colour_code:String::from(i.try_get("ColourCode")?.unwrap_or("")) 
                    };
                      
                    json_array.push(out_json);
                }  
                let json_string = serde_json::to_string(&json_array)?;
                //println!("{}",json_string);
                return Ok(json_string);              
            }
            // let json_string = serde_json::to_string(&json_array)?;
            // //println!("{}",json_string);
            // return Ok(json_string);
        }
    }
    }



pub async fn result_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel,date:String,game_group_id:i32)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_GET_ResultDetails'{}',{},'{}','{}','{}',{},'{}','{}',{}",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,date,game_group_id);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
        let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
        let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
        let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
        let mut out_put:Vec<ResultTableModel> = Vec::new();
        if status_id != '0'.to_string(){
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
        else {
            for i in &res_value[1]{
                let out_struct:ResultTableModel= ResultTableModel { game_name:String::from(i.try_get("GameName")?.unwrap_or("null")),
                    draw_date: String::from(i.try_get("DrawDate")?.unwrap_or("null")),
                    result: String::from(i.try_get("Result")?.unwrap_or("null")),
                };
                out_put.push(out_struct);
            }
            let json_string = serde_json::to_string(&out_put)?;
            return Ok(json_string);
        }
    }


pub async fn password_change_sp(IO_LOG:i32,req_stamp:f64,sms_email_url: String,header_value:HeaderModel,old_password:String,new_password:String,flag:i32)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_UPD_UpdatePassword '{}',{},'{}','{}','{}',{},'{}','{}','{}',{}",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,old_password,new_password,flag);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
    let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
    let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
    let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message
        });
        let json_string = serde_json::to_string(&out_json)?;
        if res_value.len()==2{
            //println!("password change");
            let sms_email_info:&str=res_value[1][0].try_get(0)?.unwrap_or("null");
            let sms_mail_result = sms_email_function(sms_email_info.to_string(),sms_email_url).await;
                match sms_mail_result{
                    Ok(x)=>{info!("sms_email_api success")},
                    Err(e)=>{info!("sms_email_api")}
                }
            // let json_data = json!({
            //     "data":sms_email_info
            // });
            // let sms_mail_result = queue_publisher(json_data).await;
            // match sms_mail_result{
            //     Ok(x)=>{info!("sms_email_api success")},
            //     Err(e)=>{info!("sms_email_api")}
            // }
        }
        return Ok(json_string);
    }



pub async fn ticket_info_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel,transaction_id:String,type_id:i32)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_GET_TicketInfo  '{}',{},'{}','{}','{}',{},'{}','{}',{}",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,transaction_id,type_id);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
    let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
    let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
    let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
    if status_id != '0'.to_string(){
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message
        });
        let json_string = serde_json::to_string(&out_json)?;
        return Ok(json_string);
    }
    else {
        let ticket_info :&str=res_value[1][0].try_get("TicketInfo")?.unwrap_or("");
        
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message,
            "print_info":ticket_info
        });
        let json_string = serde_json::to_string(&out_json)?;
        //println!("{}",json_string);
        return Ok(json_string);
    }
    }



pub async fn odds_config_scheme_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel,game_group_id:i32)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_GET_OddsConfigScheme '{}',{},'{}','{}','{}',{},'{}',{}",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,game_group_id);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
    let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
    let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
    let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
    let info:&str = res_value[0][0].try_get("Info")?.unwrap_or("null");
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message,
            "Info":info
        });
        let json_string = serde_json::to_string(&out_json)?;
        return Ok(json_string);
    }


pub async fn player_login_image_sp(header_value:HeaderModel)->Result<String,Box<dyn std::error::Error>>{
    let mut client: tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>> = db_connection().await?;
    let qry = format!("EXEC CLI_PlayerLoginImage '{}',{},'{}','{}','{}',{},'{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address);
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
        let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
        let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
        let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
        if status_id != '0'.to_string(){
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
        else {
            let imageinfo:&str=res_value[1][0].try_get("ImageInfo")?.unwrap_or("");
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message,
                "Imageinfo":imageinfo
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
    }



pub async fn get_game_wise_bet_info_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel,game_group_id:i32,date_time:String)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_GET_GameWiseBetInfo  '{}',{},'{}','{}','{}',{},'{}',{},'{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,game_group_id,date_time);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
    let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
    let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
    let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
    
    if status_id != '0'.to_string(){
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message
        });
        let json_string = serde_json::to_string(&out_json)?;
        return Ok(json_string);
    }
    else {
        let mut json_array:Vec<GetGamewiseBetInfoResponse>=Vec::new();
        if &res_value.len().to_string()=="1"{
            let out ="[]".to_string();
            return Ok(out);
        }
        else{
            for i in &res_value[1]{
                let out_json:GetGamewiseBetInfoResponse=GetGamewiseBetInfoResponse { 
                    bet_amount: String::from(i.try_get("BetAmount")?.unwrap_or("")),
                    bet_info:String::from(i.try_get("BetInfo")?.unwrap_or(""))};
                json_array.push(out_json);
            }
            let json_string = serde_json::to_string(&json_array)?;
            //println!("{}",json_string);
            return Ok(json_string);
        }
    }
    }



pub async fn get_available_race_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel,game_group_id:i32)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_GET_AvailableRace '{}',{},'{}','{}','{}',{},'{}',{}",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,game_group_id);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
        let status_id:&str = res_value[0][0].try_get(1)?.unwrap_or("null");
        let tvn:&str = res_value[0][0].try_get(0)?.unwrap_or("null");
        let message:&str = res_value[0][0].try_get(2)?.unwrap_or("null");
        let mut out_put:Vec<AvailableRaceResponse> = Vec::new();
        if status_id != '0'.to_string(){
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
        else {
            for i in &res_value[1]{
                let out_struct:AvailableRaceResponse= AvailableRaceResponse { 
                    game_group_id:i.try_get("GameGroupID")?.unwrap_or_default(),
                    race_group_name:String::from(i.try_get("RaceGroupName")?.unwrap_or("null")),
                    meeting_id:String::from(i.try_get("MeetingID")?.unwrap_or("null")),
                    race_id:i.try_get("RaceID")?.unwrap_or_default(),
                    race_name:String::from(i.try_get("RaceName")?.unwrap_or("null")),
                    race_date:String::from(i.try_get("RaceDate")?.unwrap_or("null")),
                    race_time:String::from(i.try_get("RaceTime")?.unwrap_or("null")),
                    race_no:i.try_get("RaceNo")?.unwrap_or_default(),
                    race_distance:i.try_get("RaceDistance")?.unwrap_or_default(),
                    meeting_length:i.try_get("MeetingLength")?.unwrap_or_default(),

                    
                };
                out_put.push(out_struct);
            }
            
            // let out_json = json!({
            //     "TVN":tvn,
            //     "Status_id":status_id,
            //     "Message":message,
            //     "Balance":balance,
            //     "Win_balance":win_balance,
            //     "Date_time":date_time
            // });
            let json_string = serde_json::to_string(&out_put)?;
            return Ok(json_string);
        }
    }


pub async fn get_game_race_details_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel,game_group_id:i32,race_id:i32)->Result<String,Box<dyn std::error::Error>>{
    let mut client: tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>> = db_connection().await?;
    let qry = format!("EXEC CLI_GET_GameRaceDetails '{}',{},'{}','{}','{}',{},'{}',{},{}",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,race_id,game_group_id);
    //println!("{:?}",&qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
        let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
        let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
        let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
        if status_id != '0'.to_string(){
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
        else {
            //set 2
            let meeting_id:&str=res_value[1][0].try_get("MeetingID")?.unwrap_or_default();
            let race_id:&str=res_value[1][0].try_get("RaceID")?.unwrap_or_default();
            let race_name:&str=res_value[1][0].try_get("RaceName")?.unwrap_or_default();
            let race_no:&str=res_value[1][0].try_get("RaceNo")?.unwrap_or_default();
            let race_date_time:&str=res_value[1][0].try_get("RaceDateTime")?.unwrap_or_default();
            //set 3
            let mut part_array:Vec<ParticipantsArray>=Vec::new();
            for i in res_value[2].iter(){
                let json_object:ParticipantsArray=ParticipantsArray{
                    dog_id:String::from(i.try_get("DogID")?.unwrap_or("")),
                    icon:String::from(i.try_get("Icon")?.unwrap_or("")),
                    dog_name:String::from(i.try_get("DogName")?.unwrap_or("")),
                    win:String::from(i.try_get("Win")?.unwrap_or("null")),
                    place:String::from(i.try_get("Place")?.unwrap_or("")),
                    show:String::from(i.try_get("Show")?.unwrap_or("")),
                    last_5_pos:String::from(i.try_get("Last5Pos")?.unwrap_or("")),
                    no_of_star:String::from(i.try_get("NoOfStar")?.unwrap_or(""))
                };
                part_array.push(json_object);
            }
            // set 4
            let info_string:&str=res_value[3][0].try_get("rsdesc")?.unwrap_or("");
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message,
                "meeting_id":meeting_id,
                "race_id":race_id,
                "race_name":race_name,
                "race_no":race_no,
                "race_date_time":race_date_time,
                "participants":part_array,
                "info_string":info_string
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
    }


pub async fn get_country_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = "EXEC CLI_GET_Country ";
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
    let mut out_json:Vec<CountryResponse>=Vec::new();
            for i in res_value[0].iter(){
                let out:CountryResponse=CountryResponse{
                    id:i.try_get("Countryid")?.unwrap_or_default(),
                    country_name:String::from(i.try_get("CountryDesc")?.unwrap_or("null")),
                    country_code:String::from(i.try_get("CountryCode")?.unwrap_or("null"))
                };
                out_json.push(out)
            }
    let json_string = serde_json::to_string(&out_json)?;
    return Ok(json_string);
    }

pub async fn deposit_init_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel,type_id:i32,amount:i32,pg_type_id:i32,pg_ref_id:String,pg_default:String,pg_item_desc:String,addmoney_type:i32,device_id:i32)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_PG_DEPOSIT_INIT  '{}',{},'{}','{}','{}',{},'{}',{},{},{},'{}','{}','{}',{},{}",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,type_id,amount,pg_type_id,pg_ref_id,pg_default,pg_item_desc,addmoney_type,device_id);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
    let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
    let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
    let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
    if status_id != '0'.to_string(){
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message
        });
        let json_string = serde_json::to_string(&out_json)?;
        return Ok(json_string);
    }
    else {
        let pg_ref_id :&str=res_value[0][0].try_get("PG_Ref_Id")?.unwrap_or("");
        let callback_url :&str=res_value[0][0].try_get("Callback_url")?.unwrap_or("");
        
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message,
            "pg_ref_id":pg_ref_id,
            "callback_url":callback_url
        });
        let json_string = serde_json::to_string(&out_json)?;
        //println!("{}",json_string);
        return Ok(json_string);
    }
    }



pub async fn addmoney_confirm_sp(IO_LOG:i32,req_stamp:f64,sms_email_url: String,header_value:HeaderModel,type_id:i32,amount:i32,pg_type_id:i32,status:i32,pg_ref_code:String,pg_txn_id:String,pg_ref_id:String,pg_data:String,item_description:String,tax_amount:String,transaction_commission:String,info_string:String)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_AddMoneyRequest_ConforMation  '{}',{},'{}','{}','{}',{},'{}',{},{},{},{},'{}','{}','{}','{}','{}','{}','{}','{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,type_id,amount,pg_type_id,status,pg_ref_code,pg_txn_id,pg_ref_id,pg_data,item_description,tax_amount,transaction_commission,info_string);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
    let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
    let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
    let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
    if status_id != '0'.to_string(){
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message
        });
        let json_string = serde_json::to_string(&out_json)?;
        if res_value.len()==2{
            let sms_email_info:&str=res_value[1][0].try_get(0)?.unwrap_or("null");
            let sms_mail_result = sms_email_function(sms_email_info.to_string(),sms_email_url).await;
                match sms_mail_result{
                    Ok(x)=>{info!("sms_email_api success")},
                    Err(e)=>{info!("sms_email_api")}
                }
        }
        return Ok(json_string);
    }
    else {
        let pg_txn_id :&str=res_value[1][0].try_get("PG_TXN_Id")?.unwrap_or("");
        let balance :&str=res_value[1][0].try_get("Balance")?.unwrap_or("");
        let win_balance :&str=res_value[1][0].try_get("Win_Balance")?.unwrap_or("");
        let date_time :&str=res_value[1][0].try_get("Date_Time")?.unwrap_or("");
        let pg_code :&str=res_value[1][0].try_get("Status")?.unwrap_or("");
        let pg_msg :&str=res_value[1][0].try_get("Msg")?.unwrap_or("");
        
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message,
            "pg_txn_id":pg_txn_id,
            "balance":balance,
            "win_balance":win_balance,
            "date_time":date_time,
            "pg_code":pg_code,
            "pg_msg":pg_msg
        });
        let json_string = serde_json::to_string(&out_json)?;
        if res_value.len()==3{
            let sms_email_info:&str=res_value[2][0].try_get(0)?.unwrap_or("null");
            let sms_mail_result = sms_email_function(sms_email_info.to_string(),sms_email_url).await;
                match sms_mail_result{
                    Ok(x)=>{info!("sms_email_api success")},
                    Err(e)=>{info!("sms_email_api")}
                }
        }
        return Ok(json_string);
    }
}


pub async fn vdr_vhr_buy_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel,bet_info:String,cli_trans_id:String,total_bet_count:i32,total_amount:String,total_estimated_win:String,requery:i32)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC TRA_VDR_OnlineBetting '{}',{},'{}','{}','{}',{},'{}','{}','{}',{},'{}','{}',{}",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,bet_info,cli_trans_id,total_bet_count,total_amount,total_estimated_win,requery);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
        let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
        let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
        let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
        if status_id != '0'.to_string(){
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
        else {
            let client_trans_id:&str=res_value[1][0].try_get("Client_TransId")?.unwrap_or("");
            let trans_id:&str=res_value[1][0].try_get("TransId")?.unwrap_or("");
            let print_info:&str=res_value[1][0].try_get("PrintInfo")?.unwrap_or("");
            let date_time:&str=res_value[1][0].try_get("Date_Time")?.unwrap_or("");
            let balance:&str=res_value[1][0].try_get("Balance")?.unwrap_or("null");
            let win_balance:&str=res_value[1][0].try_get("Win_Balance")?.unwrap_or("null");
            
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message,
                "client_transaction_id":client_trans_id,
                "transaction_id":trans_id,
                "print_info":print_info,
                "date_time":date_time,
                "Balance":balance,
                "Win_Balance":win_balance
            });
            let json_string = serde_json::to_string(&out_json)?;
            //println!("{}",json_string);
            return Ok(json_string);
        }
    }



pub async fn vdr_result_sp(IO_LOG:i32,req_stamp:f64,header_value:HeaderModel,game_group_id:i32)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_GET_VDRResult  '{}',{},'{}','{}','{}',{},'{}',{}",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,game_group_id);
    //println!("{}",qry);
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-REQUEST ,QUERY : {:?}",req_stamp,&qry);
    }
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
    if IO_LOG ==0{
        info!("STAMP : {:?}, DB-RESPONSE ,RESULT-SET : {:?}",req_stamp,&res_value);
    }
        let status_id:&str = res_value[0][0].try_get("Status_Id")?.unwrap_or("null");
        let tvn:&str = res_value[0][0].try_get("TVN")?.unwrap_or("null");
        let message:&str = res_value[0][0].try_get("Message")?.unwrap_or("null");
        if status_id != '0'.to_string(){
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
        else {
            let info_string:&str=res_value[1][0].try_get(0)?.unwrap_or("");
            
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message,
                "info_string":info_string
            });
            let json_string = serde_json::to_string(&out_json)?;
            //println!("{}",json_string);
            return Ok(json_string);
        }
    }