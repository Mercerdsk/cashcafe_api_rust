use amiquip::{
    Connection, Exchange, Publish, Result,
};
use serde_json::{Value,json};
const Q_URL:&str = "amqp://cash:gitech123*gitech@192.168.151.21:5672";
const Q_NAME:&str = "sms_email";
pub async fn queue_publisher(sms_email_info:String,sms_email_url:String) -> Result<String,Box<dyn std::error::Error>> {
    let json_data = json!(
        {"data":sms_email_info}
    );
    let mut connection = Connection::insecure_open(Q_URL)?;
    let channel = connection.open_channel(None)?;

    let exchange = Exchange::direct(&channel);

    // Define the message to be sent
    let message_string = serde_json::to_string(&json_data)?;
    exchange.publish(Publish::new(message_string.as_bytes(), Q_NAME))?;
    connection.close();
    return Ok("Message sent".to_string());
}