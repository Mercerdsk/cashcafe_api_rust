use std::path::Path;
use std::fs::File;
use crate::models::response_models::*;


pub fn get_games()->Result<String,Box<dyn std::error::Error>>{
    let json_file_path= Path::new("/home/pcadmin/rust_projects/cashcafe/json_files/games.json");
    let file = File::open(json_file_path)?;
    let games:Vec<GamesModel>=serde_json::from_reader(file)?;
    // println!("{:?}",&games);
    let games_string = serde_json::to_string(&games)?;
    return Ok(games_string);
}

pub fn get_popular_games()->Result<String,Box<dyn std::error::Error>>{
    let json_file_path= Path::new("/home/pcadmin/rust_projects/cashcafe/json_files/fav_games.json");
    let file = File::open(json_file_path)?;
    let games:Vec<FavGameModel>=serde_json::from_reader(file)?;
    // println!("{:?}",&games);
    let games_string = serde_json::to_string(&games)?;
    return Ok(games_string);
}

pub fn get_slot_games(game_type_id:i32)->Result<String,Box<dyn std::error::Error>>{
    let mut json_file_path = Path::new("");
    if game_type_id == 1{json_file_path = Path::new("/home/pcadmin/rust_projects/cashcafe/json_files/RapidWin.json")}
    if game_type_id == 2{json_file_path = Path::new("/home/pcadmin/rust_projects/cashcafe/json_files/Scratch.json")}
    if game_type_id == 3{json_file_path = Path::new("/home/pcadmin/rust_projects/cashcafe/json_files/Slots.json")}
    let file = File::open(json_file_path)?;

    let games:serde_json::Value=serde_json::from_reader(file)?;
    // println!("{:?}",&games);
    let games_string = serde_json::to_string(&games)?;
    return Ok(games_string);
    
}