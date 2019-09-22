pub mod user;

use actix_web::web;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type PgConn = web::Data<Pool<ConnectionManager<PgConnection>>>;
