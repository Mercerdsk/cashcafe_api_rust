use serde::{Serialize,Deserialize};


#[derive(Serialize,Deserialize,Debug)]
pub struct GlobalConfigModel{
    pub db_host:String,
    pub db_port:u16,
    pub db_name:String,
    pub db_user_name:String,
    pub db_password:String,
    pub toggle_log:i32,
    pub log_file_path:String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct HeaderModel{
    pub user_id:String,
    pub channel_id:i32,
    pub version:String,
    pub TVN:String,
    pub SNO:String,
    pub language_id:i32,
    pub ip_address:String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct PlayerCreationModel{
    pub first_name:String,
    pub last_name:String,
    pub email:String,
    pub dob:String,
    pub password:String,
    pub max_deposite_limit:i32,
    pub max_bet_limit:i32,
    pub kyc_id:i32,
    pub kyc_id_number:String,
    pub postal_code:String,

}

#[derive(Serialize,Deserialize,Debug)]
pub struct PlayerLoginModel{
    pub password:String,
    pub captcha:String,
    pub type_id:i32,
}


#[derive(Serialize,Deserialize,Debug)]
pub struct AvailableGamesModel{
    pub type_id:i32,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct PaymentInitModel{
    pub amount :i64,
    pub pg_type_id:i32,
    pub pg_txn_id:String,
    pub email:String,
    pub item_description:String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct AddMoneyModel{
    pub type_id:i32,
    pub amount:i64,
    pub pg_type_id:i32,
    pub pg_txn_id:String,
    pub email:String,
    pub item_description:String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct WithdrawMoneyModel{
    pub type_id:i32,
    pub amount:i64,
    pub pg_type_id:i32,
    pub pg_txn_id:String,
    pub pg_ref_id:String,
    pub pg_data:String,
    pub item_description:String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct OtpValidation{
    pub otp:String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct GetFavGamesModel{
    pub game_group_id:i32,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct OtpGeneration{
    pub type_id:i32,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct GetSlotGames{
    pub game_type_id:i32
}


#[derive(Serialize,Deserialize,Debug)]
pub struct PlayerProfileUpdate{
    pub player_image:String,
    pub player_name:String,
    pub email:String,
    pub kyc_no:String,
    pub dob:String,
}


#[derive(Serialize,Deserialize,Debug)]
pub struct BuyModel{
    pub reflot:i32,
    pub group_id:i32,
    pub draw_time:String,
    pub bet_info:String,
    pub client_transid:String,
    pub amount:i64,
    pub type_id:i32,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct KycVerifyModel{
    pub type_id:i32,
    pub player_name:String,
    pub dob:String,
    pub nationality:String,
    pub id_type:String,
    pub id_no:String,
    pub address:String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct GetCurrentResult{
    pub game_group_id:i32,
    pub draw_time:String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct GetLatestResult{
    pub game_group_id:i32,
}

#[derive(Serialize,Deserialize,Debug)]
pub  struct TransHistoryModel{
    pub from_date:String,
    pub to_date:String,
    pub type_id:i32,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct PlayerReportModel{
    pub from_date:String,
    pub to_date:String,
    pub type_id:i32,
}


#[derive(Serialize,Deserialize,Debug)]
pub struct ResultModel{
    pub date:String,
    pub game_group_id:i32,
}


#[derive(Serialize,Deserialize,Debug)]
pub struct PasswordModel{
    pub old_password:String,
    pub new_password:String,
    pub flag:i32,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct CaptchaModel{
    pub secret_key:String,
    pub recaptcha:String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct TicketInfoModel{
    pub transaction_id:String,
    pub type_id:i32,
}