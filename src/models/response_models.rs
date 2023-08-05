use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct StatusModel{
    pub TVN:String,
    pub Status_id:String,
    pub Message:String,
}

#[derive(Serialize,Deserialize)]
pub struct PlayerCreationResponse{
    pub TVN:String,
    pub Status_id:String,
    pub Message:String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct PlayerLogin{
    pub balance:String,
    pub win_balance:String,
    pub session_id:String,
    pub user_name:String,
    pub date_time:String,
}


#[derive(Serialize,Deserialize,Debug)]
pub struct AvailableGamesTable{
    pub reflot_id:String,
    pub group_id:String,
    pub group_name:String,
    pub game_name:String,
    pub draw_date:String,
    pub draw_time:String,
    pub close_time:String,
    pub validity:String,
}