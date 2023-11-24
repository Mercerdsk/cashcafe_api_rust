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
    icon:String,
    gameUrl:String,
    favID:i32,
    ggId:String,
}


#[derive(Serialize,Debug,Deserialize)]
pub struct TransType1Model{
         pub transaction_id:String,
         pub draw_date_time:String,
         pub transaction_date_time:String,
         pub amount:String,
         pub transaction_status:String,
         pub result:String,
         pub game_name:String,
         pub win_amount:String,
        
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

#[derive(Serialize,Deserialize,Debug)]
pub struct PlayerReportsResponse{
    pub from_date:String,
    pub to_date:String,
    pub bet_amount:String,
    pub win_amount:String,
    pub add_money:String,
    pub withdraw_money:String,
    pub bonus_amount:String,
    pub net_amount:String,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct PlayerActivityResponse{
    pub transaction_id:String,
    pub type_desc:String,
    pub trans_date_time:String,
    pub opening_balance:String,
    pub amount:String,
    pub closing_balance:String,
    pub colour_code:String,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct GetGamewiseBetInfoResponse{
    pub bet_amount:String,
    pub bet_info:String,
}


#[derive(Serialize,Deserialize,Debug)]
pub struct AvailableRaceResponse{
    pub game_group_id:i32,
    pub race_group_name:String,
    pub meeting_id:String,
    pub race_id:i64,
    pub race_name:String,
    pub race_date:String,
    pub race_time:String,
    pub race_no:i32,
    pub race_distance:i32,
    pub meeting_length:i32,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct ParticipantsArray{
    pub dog_id:i32,
    pub icon:String,
    pub dog_name:String,
    pub win:String,
    pub place:f32,
    pub show:f32,
    pub last_5_pos:String,
    pub no_of_star:i32
}

#[derive(Serialize,Deserialize,Debug)]
pub struct CountryResponse{
    pub id:i32,
    pub country_name:String,
    pub country_code:String,
}