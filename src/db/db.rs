// use diesel::pg::PgConnection;
// use diesel::prelude::*;

// #[derive(Clone)]
// pub struct Db {
//     conn: &mut PgConnection,
// }

// impl Db {
//     pub fn new() -> Self {
//         let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//         let conn = PgConnection::establish(&database_url)
//             .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
//         Self { conn }
//     }
// }
