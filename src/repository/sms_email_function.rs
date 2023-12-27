use reqwest;
use serde_json::{json,Value};
use actix_web::web::Json;


pub async fn sms_email_function(sms_email_info:String,sms_email_url:String)->Result<Value,Box<dyn std::error::Error>>{
    // let url = "http://192.168.10.227/CashCafeSmsEmailApi/api/user/sendEmail";
    //Live 
    let url = "http://192.168.151.21:10050/CashCafeSmsEmailApi/api/user/sendEmail";

    // json body

    // let json_data = json!(
    //     {"data":"2^arjundsk98@gmail.com|REG-CashCafe|Thamizh^971588163463|test sms by thamil"}
    // );

    let json_data = json!(
        {"data":sms_email_info}
    );
    // Send the POST request
    let client = reqwest::Client::new();
    let response = client.post(sms_email_url)
        .header("authorization", "Basic cashcafe_ae:%cash#cafe@2023*")
        .header("Content-Type", "application/json")
        .json(&json_data)
        .send().await?;
      
      if response.status().is_success() {
        let response_text: Value = response.json().await?;
      return Ok(response_text)
      }
      else{
        let response_text: Value = response.json().await?;
      return Ok(response_text)
      }
}