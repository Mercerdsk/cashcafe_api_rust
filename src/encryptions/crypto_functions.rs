use std::io::{self, Read};
use base64;
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;
use std::str;
use std::env;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

// pub async fn data_encryption(message:String,public_key_path:String)-> Result<String,Box<dyn std::error::Error>>{
//     let pub_key = load_rsa_pub_key(public_key_path.as_str())?;
//     let mut ciphertext = vec![0; pub_key.size() as usize];
//     let _encrypted_data = pub_key.public_encrypt(message.as_bytes(),&mut ciphertext, openssl::rsa::Padding::PKCS1);
//     // let encrypted_test=base64::encode(&ciphertext);
//     let encrypted_test=hex::encode(ciphertext);
//     return Ok(encrypted_test);
// }

// pub async fn data_decryption(cip_text:String,private_key_path:String)-> Result<String,Box<dyn std::error::Error>>{
//     let priv_key = load_rsa_key(private_key_path.as_str())?;
//     let ciphertext_data = base64::decode(cip_text).unwrap();
//     let mut decrypted_text = vec![0; priv_key.size() as usize];
//     let _deciphering = priv_key.private_decrypt(&ciphertext_data, &mut decrypted_text, openssl::rsa::Padding::PKCS1);
//     let decrypted_text = decrypted_text.iter().cloned().take_while(|&c| c != 0).collect::<Vec<_>>();
//     let decrypted_json_string = String::from_utf8(decrypted_text)?;
    
//     return Ok(decrypted_json_string);
// } 
// //---------------------------------_____------------------------------_______---------______--------
// fn load_rsa_key(file_path: &str) -> Result<Rsa<openssl::pkey::Private>, io::Error> {
//     let mut file = File::open(file_path)?;
//     let mut key_pem = Vec::new();
//     file.read_to_end(&mut key_pem)?;
//     let key = Rsa::private_key_from_pem(&key_pem)?;
//     Ok(key)
// }
// fn load_rsa_pub_key(file_path: &str) -> Result<Rsa<openssl::pkey::Public>, io::Error> {
//     let mut file = File::open(file_path)?;
//     let mut key_pem = Vec::new();
//     file.read_to_end(&mut key_pem)?;
//     let key = Rsa::public_key_from_pem(&key_pem)?;
//     Ok(key)
// }

pub async fn data_encryption(message_data:String,public_key_path:String)-> Result<String,Box<dyn std::error::Error>>{
    let mut mykey =String::from("000102030405060708090A0B0C0D0E0F");
    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let mut message = message_data;
    let args: Vec<String> = env::args().collect();
    if args.len() >1 {
        message = args[1].clone();
    }
    if args.len() >2 {
        mykey = args[2].clone();
    }
    println!("Message: {}",message);
    println!("Key: {}",mykey);
    println!("IV: f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let plaintext=message.as_bytes();
    let key = hex::decode(mykey).expect("Decoding failed");
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    let pos = plaintext.len();
    let mut buffer = [0u8; 50000];
    buffer[..pos].copy_from_slice(plaintext);
    let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();
    let result_string =base64::encode(ciphertext);
    return Ok(result_string);
}

pub async fn data_decryption(cip_text:String,private_key_path:String)-> Result<String,Box<dyn std::error::Error>>{
    let mut mykey =String::from("000102030405060708090A0B0C0D0E0F");
    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let key = hex::decode(mykey).expect("Decoding failed");
    println!("IV: f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let decoded_bytes = base64::decode(cip_text).expect("Failed to decode Base64");
    let decoded_array: Vec<u8> = decoded_bytes.into();
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    let mut buf = decoded_array.to_vec();
    let decrypted_ciphertext = cipher.decrypt(&mut buf).unwrap();
    
    let result_string = str::from_utf8(decrypted_ciphertext).unwrap();
    return Ok(result_string.to_string());
} 