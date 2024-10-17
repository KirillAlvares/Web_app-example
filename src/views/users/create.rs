use crate::diesel;
use diesel::prelude::*;
use actix_web::{web, HttpResponse, Responder};
use crate::database::establish_connection;
use crate::json_serialization::new_user::NewUserSchema;
use crate::models::user::new_user::NewUser;
use crate::schema::users;

pub async fn create(new_user: web::Json<NewUserSchema>) -> impl Responder {
    let mut db = establish_connection();
    let new_user = NewUser::new(
        new_user.name.clone(),
        new_user.email.clone(),
        new_user.password.clone()
    );
    let insert_result = diesel::insert_into(users::table)
                                .values(&new_user)
                                .execute(&mut db);
    match insert_result {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::Conflict()
    }
}