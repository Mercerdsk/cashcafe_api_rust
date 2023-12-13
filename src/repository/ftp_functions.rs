extern crate ftp;
use serde_json::json;
use std::str;
use std::io::Cursor;
use ftp::FtpStream;
use base64::decode;
use futures_util::__private::async_await;

pub async fn image_upload(base64_image:String,image_name:String)->Result<String,Box<dyn std::error::Error>> {
    // Create a connection to an FTP server and authenticate to it.
    let mut ftp_stream = FtpStream::connect("192.168.10.225:21").unwrap();
    let _ = ftp_stream.login("uitest", "u1Test2*").unwrap();

    // Get the current directory that the client will be reading from and writing to.
    println!("Current directory: {}", ftp_stream.pwd().unwrap());
    
    // Change into a new directory, relative to the one we are currently in.
    let _ = ftp_stream.cwd("Suriname/UploadFiles/cashcafe/Kyc_verify").unwrap();

    println!("Current directory: {}", ftp_stream.pwd().unwrap());
    
    // let base64_image="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";
    let image_data = decode(base64_image).unwrap();
    let mut image_cursor = Cursor::new(image_data);


    // Store (PUT) a file from the client to the current working directory of the server.
    // let _ = ftp_stream.put("big_img.png", &mut image_cursor);
    let _ = ftp_stream.put(image_name.as_str(), &mut image_cursor);
    println!("Successfully wrote");

    // Terminate the connection to the server.
    let _ = ftp_stream.quit();
    let out_json = json!(
        {
            "TVN":"tvn",
            "Status_id":0,
            "Message":"Successfully wrote"
        }
    );
    let json_string = serde_json::to_string(&out_json)?;
    return Ok(json_string);
}