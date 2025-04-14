
use serde::{Deserialize,Serialize};

#[derive(Deserialize)]
pub struct Users{
    pub username:String,
    pub password:String

}

#[derive(Serialize)]
pub struct UserResponse{
    id: i32,
    username:String
}

// impl Users {
//     fn create_user(&self, username:String, password:String) -> Users{
//         Users{
//             username, password
//         }

//     }
// }

