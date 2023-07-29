use serde::{Serialize,Deserialize};


#[derive(Serialize,Deserialize,Debug)]
pub struct PlayerCreationModel{
    pub user_id:String,
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


///////test
#[derive(Serialize,Deserialize,Debug)]
pub struct output_json{
    pub id:i32,
    pub name:String,
    pub official_name:String,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct input_json{
    id:i32
}


