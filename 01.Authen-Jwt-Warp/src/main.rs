/// 1. Set the data model   ✅
    //  User | LoginRequest | LoginResponse | Error | Role | ✅
/// 2. Init the data    ✅
    // 2.1 Users    ✅
/// 3. Set the route    ✅      
    // 3.1 Login_route  ✅
    // 3.2 Func: Login_handle   ✅
/// 4. Create the jwt   ✅
    // set the headers| claims | signature  ✅
    // create_jwt(uid, role)    ✅
/// 5. Run the server   ✅
mod auth;
mod error;
use serde::{Deserialize, Serialize};
use auth::{create_jwt, Role};
use error::Error;

use std::collections::HashMap;
use std::sync::Arc;
type Users = Arc<HashMap<String, User>>;

use warp::{self, reject::{self, Rejection}, reply, Filter, Reply};

// 1.1 Set the data model(User, LoginRequest, LoginResponse)
#[derive(Clone, PartialEq)]
pub struct User{
    uid: String,
    email: String,
    pwd: String,
    role: String,
}

#[derive(Deserialize)]
pub struct LoginRequest{
    email: String,
    pwd: String,
}

#[derive(Serialize)]
pub struct LoginResponse{
    token: String,
}

// 2.1  Set the Func-----init_users
fn init_users() -> HashMap<String, User>{
    let mut map = HashMap::new();

    map.insert(
        String::from("1"),
        User{
            uid: String::from("1"),
            email: String::from("123@163.com"),
            pwd: String::from("1234"),
            role: String::from("User"),
        }
    );
    map.insert(
        String::from("2"),
        User{
            uid: String::from("2"),
            email: String::from("123@gmail.com"),
            pwd: String::from("1234"),
            role: String::from("Admin"),
        }
    );
    map

}

// 3.2 Set the Func------login_handle
async fn login_handle(users: Users, body: LoginRequest) -> Result<impl Reply, Rejection>{
    
    match users
        .iter()
        .find(|(_uid, user)| user.pwd == body.pwd && user.email == body.email)
        {
            Some((uid, user)) => 
            {
                let token = create_jwt(uid, &Role::from_str(&user.role))
                    .map_err(|e| reject::custom(e))?;
                Ok(reply::json(& LoginResponse {token}))
            }
            None => Err(reject::custom(Error::discontent))
        }
}

#[tokio::main]
async fn main(){
    // 2.2  Init Users
    let users = Users::new(init_users());

    // 3.1 Set the route(Login)
    let login_route = warp::path("login")
        .and(warp::post())
        .and(warp::any().map(move || users.clone() ))
        .and(warp::body::json())
        .and_then(login_handle);

    warp::serve(login_route).run(([127,0,0,1],8000)).await;
}
