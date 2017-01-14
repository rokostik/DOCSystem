mod schema {
    infer_schema!("dotenv:DATABASE_URL");
}

use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use self::schema::users;

fn db() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[table_name = "users"]
#[derive(Serialize, Queryable, Insertable, FromForm, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub surname: String,
    pub password: String,
}

#[derive(FromForm, Debug, Clone)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

impl UserLogin {
    pub fn get(&self) -> Vec<User> {
        users::table.load(&db()).expect("Error getting user")
    }
}

#[table_name = "users"]
#[derive(Serialize, Insertable, FromForm, Debug, Clone)]
pub struct UserNew {
    pub username: String,
    pub name: String,
    pub surname: String,
    pub password: String,
}

impl UserNew {
    pub fn insert(&self) -> bool {
        diesel::insert(self).into(users::table).execute(&db()).is_ok()
    }
}



