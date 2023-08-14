use serde_json::json;

use crate::repository::database_connection::db_connection;
use crate::models::request_models::*;
use crate::models::response_models::*;


pub async fn player_creation_sp(header_value:HeaderModel,first_name:String,last_name:String,email:String,dob:String,password:String,max_deposite_limit:i32,max_bet_limit:i32,kyc_id:i32,kyc_id_number:String,postal_code:String)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let mut array_data:Vec<PlayerCreationResponse> = Vec::new();
    let qry = format!("EXEC CLI_INS_PlayerRegistration '{}',{},'{}','{}','{}',{},'{}','{}','{}','{}','{}','{}',{},{},{},'{}','{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,first_name,last_name,email,dob,password,max_deposite_limit,max_bet_limit,kyc_id,kyc_id_number,postal_code);
    println!("{}",qry);
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
        let tvn:&str = res_value[0][0].get("TVN").unwrap_or("null");
        let status_id:&str = res_value[0][0].get("Status_Id").unwrap_or("");
        let message:&str = res_value[0][0].get("Message").unwrap_or("null");
        let out_json = PlayerCreationResponse{
            TVN:String::from(tvn),
            Status_id:String::from(status_id),
            Message:String::from(message),
        };
        array_data.push(out_json);
        let json_string = serde_json::to_string(&array_data)?;
    return Ok(json_string);
    }


pub async fn player_login_sp(header_value:HeaderModel,password:String,captcha:String)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_PlayerLogin '{}',{},'{}','{}','{}',{},'{}','{}','{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,password,captcha);
    println!("{}",qry);
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
        let status_id:&str = res_value[0][0].get("Status_Id").unwrap_or("null");
        let tvn:&str = res_value[0][0].get("TVN").unwrap_or("null");
        let message:&str = res_value[0][0].get("Message").unwrap_or("null");
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
            let balance:&str=res_value[1][0].get(0).unwrap_or("");
            let win_balance:&str=res_value[1][0].get(1).unwrap_or("");
            let session_id:&str=res_value[1][0].get(2).unwrap_or("");
            let user_name:&str=res_value[1][0].get(3).unwrap_or("");
            let date_time:&str=res_value[1][0].get(4).unwrap_or("");
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message,
                "Balance":balance,
                "Win_balance":win_balance,
                "Session_id":session_id,
                "User_name":user_name,
                "Date_time":date_time
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
    }

pub async fn get_balance_sp(header_value:HeaderModel)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_GET_Balance '{}',{},'{}','{}','{}',{},'{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address);
    println!("{}",qry);
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
        let status_id:&str = res_value[0][0].get("Status_Id").unwrap_or("null");
        let tvn:&str = res_value[0][0].get("TVN").unwrap_or("null");
        let message:&str = res_value[0][0].get("Message").unwrap_or("null");
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
            let balance:i64=res_value[1][0].get(0).unwrap_or(0);
            let win_balance:i64=res_value[1][0].get(1).unwrap_or(0);
            let date_time:&str=res_value[1][0].get(2).unwrap_or("");
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


pub async fn available_games_sp(header_value:HeaderModel,type_id:i32)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_GET_AvailableGames '{}',{},'{}','{}','{}',{},'{}',{}",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,type_id);
    println!("{}",qry);
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
        let status_id:&str = res_value[0][0].get(1).unwrap_or("null");
        let tvn:&str = res_value[0][0].get(0).unwrap_or("null");
        let message:&str = res_value[0][0].get(2).unwrap_or("null");
        let date_time:&str = res_value[0][0].get(3).unwrap_or("null");
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
                let out_struct:AvailableGamesTable= AvailableGamesTable { reflot_id:String::from(i.get(0).unwrap_or("null")),
                    group_id: String::from(i.get(1).unwrap_or("null")),
                    group_name: String::from(i.get(2).unwrap_or("null")),
                    game_name: String::from(i.get(3).unwrap_or("null")),
                    draw_date: String::from(i.get(4).unwrap_or("null")), 
                    draw_time: String::from(i.get(5).unwrap_or("null")), 
                    close_time: String::from(i.get(6).unwrap_or("null")), 
                    interval:String::from(i.get(7).unwrap_or("null")),
                    end_time:String::from(i.get(8).unwrap_or("null")),
                    min_max_multi:String::from(i.get(9).unwrap_or("null")),
                    odds:String::from(i.get(10).unwrap_or("null")),
                    no_of_balls:String::from(i.get(11).unwrap_or("null")),
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



pub async fn payment_init_sp(header_value:HeaderModel,amount:i64,pg_type_id:i32,pg_txn_id:String,email:String,item_des:String)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_PayMentInitRequest '{}',{},'{}','{}','{}',{},'{}',{},{},'{}','{}','{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,amount,pg_type_id,pg_txn_id,email,item_des);
    println!("{}",qry);
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
        let status_id:&str = res_value[0][0].get("Status_Id").unwrap_or("null");
        let tvn:&str = res_value[0][0].get("TVN").unwrap_or("null");
        let message:&str = res_value[0][0].get("Message").unwrap_or("null");
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
            let pg_ref_id:&str=res_value[1][0].get(0).unwrap_or("");
            let callback_url:&str=res_value[1][0].get(1).unwrap_or("");
            let date_time:&str=res_value[1][0].get(2).unwrap_or("");
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

    
pub async fn add_money_sp(header_value:HeaderModel,type_id:i32,amount:i64,pg_type_id:i32,pg_txn_id:String,email:String,item_des:String)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_AddMoneyRequest '{}',{},'{}','{}','{}',{},'{}',{},{},{},'{}','{}','{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,type_id,amount,pg_type_id,pg_txn_id,email,item_des);
    println!("{}",qry);
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
        let status_id:&str = res_value[0][0].get("Status_Id").unwrap_or("null");
        let tvn:&str = res_value[0][0].get("TVN").unwrap_or("null");
        let message:&str = res_value[0][0].get("Message").unwrap_or("null");
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
            let pg_txn_id:&str=res_value[1][0].get(0).unwrap_or("");
            let balance:i64=res_value[1][0].get(1).unwrap_or(0);
            let win_balance:i64=res_value[1][0].get(2).unwrap_or(0);
            let date_time:&str=res_value[1][0].get(3).unwrap_or("");
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



pub async fn withdraw_money_sp(header_value:HeaderModel,type_id:i32,amount:i64,pg_type_id:i32,pg_txn_id:String,pg_ref_id:String,pg_data:String,item_des:String)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_WithDrawRequest   '{}',{},'{}','{}','{}',{},'{}',{},{},{},'{}','{}','{}','{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,type_id,amount,pg_type_id,pg_txn_id,pg_ref_id,pg_data,item_des);
    println!("{}",qry);
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
        let status_id:&str = res_value[0][0].get("Status_Id").unwrap_or("null");
        let tvn:&str = res_value[0][0].get("TVN").unwrap_or("null");
        let message:&str = res_value[0][0].get("Message").unwrap_or("null");
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
            let pg_txn_id:&str=res_value[1][0].get(0).unwrap_or("");
            let balance:i64=res_value[1][0].get(1).unwrap_or(0);
            let win_balance:i64=res_value[1][0].get(2).unwrap_or(0);
            let date_time:&str=res_value[1][0].get(3).unwrap_or("");
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
            return Ok(json_string);
        }
    }

    
pub async fn otp_validation_sp(header_value:HeaderModel,otp:String)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_OTPValidate '{}',{},'{}','{}','{}',{},'{}','{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,otp);
    println!("{}",qry);
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
        let status_id:&str = res_value[0][0].get(1).unwrap_or("null");
        let tvn:&str = res_value[0][0].get(0).unwrap_or("null");
        let message:&str = res_value[0][0].get(2).unwrap_or("null");
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message
        });
        let json_string = serde_json::to_string(&out_json)?;
        return Ok(json_string);
    }


pub async fn otp_generation_sp(header_value:HeaderModel,type_id:i32)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_OTPGeneration '{}',{},'{}','{}','{}',{},'{}',{}",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,type_id);
    println!("{}",qry);
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
        let status_id:&str = res_value[0][0].get(1).unwrap_or("null");
        let tvn:&str = res_value[0][0].get(0).unwrap_or("null");
        let message:&str = res_value[0][0].get(2).unwrap_or("null");
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message
        });
        let json_string = serde_json::to_string(&out_json)?;
        return Ok(json_string);
    }

pub async fn get_server_time_sp(header_value:HeaderModel)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_GET_ServerTime '{}',{},'{}','{}','{}',{},'{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address);
    println!("{}",qry);
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
        let date_time:&str = res_value[0][0].get(0).unwrap_or("null");
        let out_json = json!({
            "Status_id":"0",
            "Date_time":date_time
        });
        let json_string = serde_json::to_string(&out_json)?;
        return Ok(json_string);
    }


pub async fn get_player_profile_sp(header_value:HeaderModel)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_GET_Profile '{}',{},'{}','{}','{}',{},'{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address);
    println!("{}",qry);
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
        let status_id:&str = res_value[0][0].get("Status_Id").unwrap_or("null");
        let tvn:&str = res_value[0][0].get("TVN").unwrap_or("null");
        let message:&str = res_value[0][0].get("Message").unwrap_or("null");
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
            let player_image:&str=res_value[1][0].get("PlayerImage").unwrap_or("");
            let phone_no:&str=res_value[1][0].get("PhoneNo").unwrap_or("");
            let player_name:&str=res_value[1][0].get("PlayerName").unwrap_or("");
            let email:&str=res_value[1][0].get("EMAIL").unwrap_or("");
            let kyc_no:&str=res_value[1][0].get("KYCNo").unwrap_or("");
            let dob:&str=res_value[1][0].get("DOB").unwrap_or("");
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message,
                "player_image":player_image,
                "phone_no":phone_no,
                "player_name":player_name,
                "email":email,
                "kyc_no":kyc_no,
                "dob":dob
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
    }



pub async fn update_player_profile_sp(header_value:HeaderModel,player_image:String,player_name:String,email:String,kyc_no:String,dob:String)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_UPD_PlayerProfile '{}',{},'{}','{}','{}',{},'{}','{}','{}','{}','{}','{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,player_image,player_name,email,kyc_no,dob);
    println!("{}",qry);
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
        let status_id:&str = res_value[0][0].get(1).unwrap_or("null");
        let tvn:&str = res_value[0][0].get(0).unwrap_or("null");
        let message:&str = res_value[0][0].get(2).unwrap_or("null");
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message
        });
        let json_string = serde_json::to_string(&out_json)?;
        return Ok(json_string);
    }

    
pub async fn buy_sp(header_value:HeaderModel,reflotid:i32,group_id:i32,draw_time:String,bet_info:String,client_transid:String,amount:i64,type_id:i32)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC TRA_OnelineSales '{}',{},'{}','{}','{}',{},'{}',{},{},'{}','{}','{}','{}',{}",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,reflotid,group_id,draw_time,bet_info,client_transid,amount,type_id);
    println!("{}",qry);
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
        let status_id:&str = res_value[0][0].get("Status_Id").unwrap_or("null");
        let tvn:&str = res_value[0][0].get("TVN").unwrap_or("null");
        let message:&str = res_value[0][0].get("Message").unwrap_or("null");
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
            let client_trans_id:&str=res_value[1][0].get("Client_TransId").unwrap_or("");
            let trans_id:&str=res_value[1][0].get("TransId").unwrap_or("");
            let print_info:&str=res_value[1][0].get("PrintInfo").unwrap_or("");
            let date_time:&str=res_value[1][0].get("Date_Time").unwrap_or("");
            
            let out_json = json!({
                "TVN":tvn,
                "Status_id":status_id,
                "Message":message,
                "client_transaction_id":client_trans_id,
                "transaction_id":trans_id,
                "print_info":print_info,
                "date_time":date_time
            });
            let json_string = serde_json::to_string(&out_json)?;
            return Ok(json_string);
        }
    }


pub async fn kyc_verification_sp(header_value:HeaderModel,player_name:String,dob:String,nationality:String,id_type:String,id_no:String,address:String)->Result<String,Box<dyn std::error::Error>>{
    let mut client = db_connection().await?;
    let qry = format!("EXEC CLI_GET_KYCVerifiCation '{}',{},'{}','{}','{}',{},'{}','{}','{}','{}','{}','{}','{}'",header_value.user_id,header_value.channel_id,header_value.version,header_value.TVN,header_value.SNO,header_value.language_id,header_value.ip_address,player_name,dob,nationality,id_type,id_no,address);
    println!("{}",qry);
    let res = client.query(qry,&[]).await?;
    let res_value=res.into_results().await?;
        let status_id:&str = res_value[0][0].get(1).unwrap_or("null");
        let tvn:&str = res_value[0][0].get(0).unwrap_or("null");
        let message:&str = res_value[0][0].get(2).unwrap_or("null");
        let out_json = json!({
            "TVN":tvn,
            "Status_id":status_id,
            "Message":message
        });
        let json_string = serde_json::to_string(&out_json)?;
        return Ok(json_string);
    }
    