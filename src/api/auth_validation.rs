use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn generate_token(user_id:String) -> Result<String,Box<dyn std::error::Error>> {
    
    // Create JWT claims
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: (chrono::Utc::now() + chrono::Duration::minutes(1)).timestamp() as usize,
    };

    // Encode the claims into a JWT token
    let encoding_key = EncodingKey::from_secret("your_secret_key".as_ref());
    let token = encode(&Header::default(), &claims, &encoding_key).unwrap();
    return Ok(token);
}

pub async fn protected(req: HttpRequest) -> Result<String,Box<dyn std::error::Error>> {
    // Extract the Authorization header from the request
    if let Some(authorization) = req.headers().get("Authorization") {
        if let Ok(authorization_str) = authorization.to_str() {
            if let Some(token) = authorization_str.strip_prefix("Bearer ") {
                // Decode and verify the JWT token
                let decoding_key = DecodingKey::from_secret("your_secret_key".as_ref());
                match decode::<Claims>(token, &decoding_key, &Validation::new(Algorithm::HS256)) {
                    Ok(token_data) => {
                        // Token is valid, you can access token_data.claims.sub for user ID
                        let user_id = token_data.claims.sub;
                        return Ok(user_id);
                    }
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            }
        }
    }
    return Err("Not Authorized".to_string().into());
}