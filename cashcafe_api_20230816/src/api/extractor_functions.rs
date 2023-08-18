use crate::models::request_models::HeaderModel;
use actix_web::HttpRequest;

pub async fn header_extractor(req:HttpRequest)->Result<HeaderModel,Box<dyn std::error::Error>>{
    let mut user_id_value="";
    let user_id=req.headers().get("User_Id");
    match user_id {
        Some(x)=>{user_id_value=x.to_str()?}
        None =>{return Err("user id header not found".into());}
    }

    let mut channel_id_value="";
    let channel_id = req.headers().get("Channel_Id");
    match channel_id {
        Some(x)=>{channel_id_value=x.to_str()?}
        None =>{return Err("Channel id header not found".into());}
    }
    
    let mut version_value ="";
    let version = req.headers().get("Version");
    match version {
        Some(x)=>{version_value=x.to_str()?}
        None =>{return Err("version header not found".into());}
    }

    let mut tvn_value="";
    let tvn = req.headers().get("TVN");
    match tvn {
        Some(x)=>{tvn_value=x.to_str()?}
        None =>{return Err("TVN header not found".into());}
    }

    let mut sno_value="";
    let sno = req.headers().get("SNO");
    match sno {
        Some(x)=>{sno_value=x.to_str()?}
        None =>{return Err("SNO header not found".into());}
    }

    let mut lang_id_value="";
    let language_id = req.headers().get("Language_Id");
    match language_id {
        Some(x)=>{lang_id_value=x.to_str()?}
        None =>{return Err("Language id header not found".into());}
    }
    
    let mut ip_address_value = "";
    let ip_address = req.headers().get("IP_Address");
    match ip_address {
        Some(x)=>{ip_address_value=x.to_str()?}
        None =>{return Err("Ip address header not found".into());}
    }

    let header_value = HeaderModel{
        user_id:String::from(user_id_value),
        channel_id:channel_id_value.parse()?,
        version:String::from(version_value),
        TVN:String::from(tvn_value),
        SNO:String::from(sno_value),
        language_id:lang_id_value.parse()?,
        ip_address:String::from(ip_address_value),
    };
    return Ok(header_value);
}