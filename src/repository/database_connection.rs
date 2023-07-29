use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;
//use tiberius::QueryStream::QueryItem;
use tokio_util::compat::Compat;
use tiberius::QueryItem;
use futures_util::TryStreamExt;
use futures_util::StreamExt;



pub async fn db_connection()->Result<Client<Compat<TcpStream>>,Box<dyn std::error::Error>>{
    let mut config = Config::new();

    config.host("192.168.10.215");
    config.port(1433);
    config.authentication(AuthMethod::sql_server("sa", "gitech123*gitech"));
    config.trust_cert(); // on production, it is not a good idea to do this

    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true).expect("error 2");
    let mut client = Client::connect(config, tcp.compat_write()).await?;
    Ok(client)
}
