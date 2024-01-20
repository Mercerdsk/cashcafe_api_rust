use std::fs::File;
use std::io::{self, Read};
use openssl::rsa::Rsa;
use base64;

pub async fn data_encryption(message:String,public_key_path:String)-> Result<String,Box<dyn std::error::Error>>{
    let pub_key = load_rsa_pub_key(public_key_path.as_str())?;
    let mut ciphertext = vec![0; pub_key.size() as usize];
    let _encrypted_data = pub_key.public_encrypt(message.as_bytes(),&mut ciphertext, openssl::rsa::Padding::PKCS1);
    let encrypted_test=base64::encode(&ciphertext);
    return Ok(encrypted_test);
}

pub async fn data_decryption(cip_text:String,private_key_path:String)-> Result<String,Box<dyn std::error::Error>>{
    let priv_key = load_rsa_key(private_key_path.as_str())?;
    let ciphertext_data = base64::decode(cip_text).unwrap();
    let mut decrypted_text = vec![0; priv_key.size() as usize];
    let _deciphering = priv_key.private_decrypt(&ciphertext_data, &mut decrypted_text, openssl::rsa::Padding::PKCS1);
    let decrypted_text = decrypted_text.iter().cloned().take_while(|&c| c != 0).collect::<Vec<_>>();
    let decrypted_json_string = String::from_utf8(decrypted_text)?;
    
    return Ok(decrypted_json_string);
} 
//---------------------------------_____------------------------------_______---------______--------
fn load_rsa_key(file_path: &str) -> Result<Rsa<openssl::pkey::Private>, io::Error> {
    let mut file = File::open(file_path)?;
    let mut key_pem = Vec::new();
    file.read_to_end(&mut key_pem)?;
    let key = Rsa::private_key_from_pem(&key_pem)?;
    Ok(key)
}
fn load_rsa_pub_key(file_path: &str) -> Result<Rsa<openssl::pkey::Public>, io::Error> {
    let mut file = File::open(file_path)?;
    let mut key_pem = Vec::new();
    file.read_to_end(&mut key_pem)?;
    let key = Rsa::public_key_from_pem(&key_pem)?;
    Ok(key)
}
