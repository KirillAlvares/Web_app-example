use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse, Responder};
use crate::models::user::user::User;
use crate::json_serialization::login::Login;
use crate::schema::users;
use crate::jwt::JwToken;
use std::collections::HashMap;
use crate::database::establish_connection;





pub async fn login(credentials: web::Json<Login>) -> HttpResponse {
    let connection = &mut establish_connection();
    let password = credentials.password.clone();
    let users = users::table
                .filter(users::columns::username.eq(
                credentials.username.clone())
        ).load::<User>(connection).unwrap();
    if users.len() == 0 {
        return HttpResponse::NotFound().await.unwrap()
    } else if users.len() > 1 {
        return HttpResponse::Conflict().await.unwrap()
    }
    match users[0].verify(password) {
        true => {
            let token = JwToken::new(users[0].id);
            let raw_token = token.encode();
            let mut body = HashMap::new();
            body.insert("token", raw_token);
            HttpResponse::Ok().json(body)
        },
        false => HttpResponse::Conflict().await.unwrap()
    }
}