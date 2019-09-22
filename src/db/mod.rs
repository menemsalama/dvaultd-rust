use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2;

pub fn connect(database_url: String) -> ConnectionManager<PgConnection> {
    ConnectionManager::<PgConnection>::new(database_url)
}

pub fn pool(
    manager: ConnectionManager<PgConnection>,
) -> r2d2::Pool<ConnectionManager<diesel::PgConnection>> {
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
