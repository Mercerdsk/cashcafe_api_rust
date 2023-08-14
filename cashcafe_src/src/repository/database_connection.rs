use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
//use tiberius::QueryStream::QueryItem;
use tokio_util::compat::Compat;
use tiberius::QueryItem;
use futures_util::TryStreamExt;
use futures_util::StreamExt;
use std::path::Path;
use std::fs::File;
use crate::models::request_models::GlobalConfigModel;




pub async fn db_connection()->Result<Client<Compat<TcpStream>>,Box<dyn std::error::Error>>{
    
    let json_file_path= Path::new("./json_files/database_config.json");
    let file = File::open(json_file_path)?;
    let games:GlobalConfigModel=serde_json::from_reader(file)?;
    // println!("{:?}",&games);

    let mut config = Config::new();
    config.host(games.db_host);
    config.port(games.db_port);
    config.database(games.db_name);
    config.authentication(AuthMethod::sql_server(games.db_user_name, games.db_password));
    config.trust_cert(); // on production, it is not a good idea to do this

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true).expect("error 2");
    let mut client = Client::connect(config, tcp.compat_write()).await?;
    Ok(client)
}


// pub async fn db_connection()->Result<Client<Compat<TcpStream>>,Box<dyn std::error::Error>>{
    
//     // let json_file_path= Path::new("/home/pcadmin/rust_projects/cashcafe/json_files/fav_games.json");
//     // let file = File::open(json_file_path)?;
//     // let games:Vec<GlobalConfigModel>=serde_json::from_reader(file)?;

//     let mut config = Config::new();
//     config.host("192.168.10.215");
//     config.port(1433);
//     config.database("CaseCafeOnlineWebDB");
//     config.authentication(AuthMethod::sql_server("sa", "gitech123*gitech"));
//     config.trust_cert(); // on production, it is not a good idea to do this

//     let tcp = TcpStream::connect(config.get_addr()).await?;
//     tcp.set_nodelay(true).expect("error 2");
//     let mut client = Client::connect(config, tcp.compat_write()).await?;
//     Ok(client)
// }