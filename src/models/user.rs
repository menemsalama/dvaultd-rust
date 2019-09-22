// use crate::schema::users::columns::email;
use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::PgConnection;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Clone)]
#[table_name = "users"]
pub struct User {
    pub email: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub password: String,
    pub created_at: NaiveDateTime,
}

impl User {
    pub fn clear_pwd(&mut self) {
        self.password = "".to_string();
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRequest {
    pub email: String,
    pub password: String,
}

pub type UsersResult = Result<Vec<User>, diesel::result::Error>;

pub fn create(conn: &PgConnection, payload: &UserRequest) -> UsersResult {
    // use crate::schema::users::dsl::*;
    use crate::schema::users::dsl::{email,password, created_at, users};
    use diesel::insert_into;
    use diesel::prelude::*;

    insert_into(users)
        .values((
            email.eq(payload.email.clone()),
            password.eq(payload.password.clone()),
            created_at.eq(chrono::Local::now().naive_local()),
        ))
        .get_results::<User>(conn)
}

pub fn get(conn: &PgConnection, user_id: Option<String>) -> UsersResult {
    use diesel::prelude::*;
    use crate::schema::users::dsl::{email, users};

    if let Some(id) = user_id {
        return users.filter(email.eq(id)).load::<User>(conn);
    }

    users.load::<User>(conn)
    // TODO: return ServiceError::InternalServerError in production for unkown errors
    // #[cfg(feature = "release")]
    // .map_err(|_| ServiceError::InternalServerError)?;
}
