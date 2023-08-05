use serde::{Serialize,Deserialize};
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

