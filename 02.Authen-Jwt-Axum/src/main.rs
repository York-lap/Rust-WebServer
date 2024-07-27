use std::time::SystemTime;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use axum::{
    async_trait, extract::{FromRequestParts,Request}, http::{header, request::Parts, StatusCode}, response::{Html, IntoResponse}, routing::{get,post}, Json, RequestPartsExt, Router
};
use axum_extra::{headers::{authorization::Bearer, Authorization}, TypedHeader};
/// 1.Define Data Structure
    // User|LoginRequest|LoginResponse|Claims     

/// 2.Error handling
    // AuthError:[InvalidToken, WrongCredentials, TokenCredential, MissingCredentials]

/// 3.Set the route
    // 3.1 Set app
    // 3.2 Set Listener
    // 3.3 Server

/// 4.Set the handler
    // 
/// 5.Create JWT
    // 1. secret
const SECRETE: &[u8] = b"yorklap";

// 1. Define Data Structure
#[derive(Serialize,Deserialize,Debug)]
struct User{
    id: String,
    email: String,
    password:String,
}
#[derive(Serialize,Deserialize,Debug)]
struct LoginRequest{
    email: String,
    password: String,
}
#[derive(Serialize,Deserialize,Debug)]
struct LoginResponse{
    token: String,
}

#[derive(Serialize,Deserialize,Debug)]
struct Claims{
    sub: u32,
    email: String,
    exp: usize,
}

// 2.Error Handling

    // 2.1 Define Error
pub enum AuthError {
    InvalidToken,
    WrongCredentials,
    TokenCredential,
    MissingCredentials,
}
    // 2.2 impl IntoResponse
impl IntoResponse for AuthError{
    fn into_response(self) -> axum::response::Response {
        let (statuscode, err_msg) = match self {
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_GATEWAY, "Missing credentials"),
            AuthError::TokenCredential => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
        };
        (statuscode, err_msg).into_response()
    }
}

#[tokio::main]
async fn main() {
    
    // 3. Set the route
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/auth", post(login_handler))
        .route("/get", post(getinfo_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

}

/// 4.Set the handler

// 4.2 Index
async fn index_handler() -> Html<&'static str>{
    Html("<h3>Hello World!</h3>")
}

// 4.2 Get JWT
async fn login_handler( Json(login): Json<LoginRequest> ) -> Result<Json<LoginResponse>, AuthError> {
    // header
    let header = Header::default();

    // claim
    let expr = std::time::SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
        +  24*60*60;
    let claims = Claims{
        sub: 1,
        email: login.email.clone(),
        exp: expr,
    };

    let token = encode(&header, &claims, &EncodingKey::from_secret(SECRETE))
        .map_err(|_| AuthError::TokenCredential)?;

    Ok(Json(LoginResponse{token}))
}

// 4.3 Check JWT
#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {

        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>().await
            .map_err(|_| AuthError::InvalidToken)?;

        let token = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(SECRETE),
            &Validation::default()
        ).map_err(|_| AuthError::InvalidToken)?;

        Ok(token.claims)

    }
}
async fn getinfo_handler(claims:Claims)->StatusCode {
    println!("{:?}",claims);
    StatusCode::CREATED
}