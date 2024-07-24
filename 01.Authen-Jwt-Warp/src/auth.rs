use core::{fmt};
use serde::Serialize;
use jsonwebtoken::{Header, Algorithm, encode, EncodingKey};
use chrono::{prelude::*, Duration};
use crate::error::Error;
// 1.2 Set the data model(Role)
pub enum Role{
    Admin,
    User,
}

impl Role{
    pub fn from_str(role: &str) -> Self{
        match role{
            "Admin" => Role::Admin,
            _ => Role::User
        }
    }
}

impl fmt::Display for Role{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Role::Admin => write!(f,"Admin"),
            Role::User => write!(f, "User"),
        }
    }
}

// 4.1.2 set the claims
#[derive(Serialize)]
struct Claims{
    sub: String,
    role: String,
    exp: usize, 
}

// 4.2 create jwt
pub fn create_jwt(uid: &str, role: &Role)-> Result<String, Error>{

    // 4.1.1 set the headers
    let headers = Header::new(Algorithm::HS256);

    // 4.1.2 set the claim
    let expr = Utc::now()
        .checked_add_signed(Duration::seconds(60))
        .expect("Invalid expriation")
        .timestamp();

    let claims = Claims{
        sub: uid.to_owned(),
        role: role.to_string(),
        exp: expr as usize,
    };

    const JWTSECRETE: &[u8] = b"secrete";
    encode(&headers, &claims, &EncodingKey::from_secret(JWTSECRETE))
        .map_err(|_| Error::jwtcreationerror)

}
