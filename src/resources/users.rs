use crate::models;
use actix_web::{web, FromRequest, HttpResponse, Responder, Scope};
use models::user::UserRequest;

pub fn resource() -> Scope {
    web::scope("/users")
        .service(
            web::resource("")
                .route(web::get().to(get_users))
                .data(web::Json::<UserRequest>::configure(|cfg| {
                    // accepte 64 bytes JSON only, or return size exceeded
                    // long email and password should exceeded but I'm playing with options
                    // you can use http://bytesizematters.com/ to get the size of a JSON sample
                    cfg.limit(64)
                }))
                .route(web::post().to(create_user)),
        )
        .service(web::resource("/{id}").route(web::get().to(get_user)))
}

fn create_user(user: web::Json<UserRequest>, pool: models::PgConn) -> impl Responder {
    let db = &pool.get().unwrap();
    let result = models::user::create(db, &user);

    match result {
        Ok(user) => HttpResponse::Ok().json(user.get(0)),
        Err(e) => HttpResponse::Ok().json(e.to_string()),
    }
}

fn get_users(pool: models::PgConn) -> impl Responder {
    let db = &pool.get().unwrap();
    let result = models::user::get(db, None);

    match result {
        Ok(mut users) => {
            for user in users.iter_mut() {
                user.clear_pwd();
            }

            HttpResponse::Ok().json(users)
        }
        Err(e) => HttpResponse::Ok().json(e.to_string()),
    }
}

fn get_user(id: web::Path<(String)>, pool: models::PgConn) -> impl Responder {
    let db = &pool.get().unwrap();

    let result = models::user::get(db, Some(id.to_string()));

    match result {
        Ok(users) => {
            let item = users.get(0);
            match item {
                Some(user) => {
                    let mut user = user.clone();
                    user.clear_pwd();
                    HttpResponse::Ok().json(user)
                }
                None => HttpResponse::Ok().json(item),
            }
        }
        Err(e) => HttpResponse::Ok().json(e.to_string()),
    }
}
