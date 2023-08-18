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
    pub interval:String,
    pub end_time:String,
    pub min_max_multi:String,
    pub odds:String,
    pub no_of_balls:String,

}

#[derive(Serialize,Debug,Deserialize)]
pub struct GamesModel{
    gamegroupID: i32,
    gamegroupName: String,
}

#[derive(Serialize,Debug,Deserialize)]
pub struct FavGameModel{
    gameID: i32,
    gamegroupID: i32,
    gameName: String,
    favID:i32,
}


#[derive(Serialize,Debug,Deserialize)]
pub struct TransType1Model{
         pub transaction_id:String,
         pub draw_date_time:String,
         pub transaction_date_time:String,
         pub amount:String,
         pub transaction_status:String
        
}

#[derive(Serialize,Deserialize,Debug)]
pub struct TransType23Model{
    pub  transaction_id:String,
    pub  transaction_date_time:String,
    pub  amount:String,
    pub  transaction_status:String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct ResultTableModel{
    pub game_name:String,
    pub draw_date:String,
    pub result:String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct LastResultModel{
    pub draw_no:String,
    pub draw_date:String,
    pub draw_time:String,
    pub win_nods:String,
}
