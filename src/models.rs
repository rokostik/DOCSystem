mod schema {
    infer_schema!("dotenv:DATABASE_URL");
}

use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use self::schema::*;

fn db() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[table_name = "users"]
#[derive(Serialize, Queryable, Insertable, FromForm, Debug, Clone, Identifiable, Associations)]
#[has_many(folders)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub surname: String,
    pub password: String,
}

impl User {

    /*pub fn new_folder(&self, folder_name: String) {
        diesel::insert(&folder).into(folders::table).belonging_to(&self).execute(&db()).is_ok()
    }*/

    pub fn get(id: i32) -> User {
        users::table.find(id).first(&db()).expect("Error getting user.")
    }

    pub fn get_folders(&self) -> Vec<Folder> {
        Folder::belonging_to(self).load(&db()).expect("Error getting folders")
    }
}

#[derive(FromForm, Debug, Clone)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

impl UserLogin {
    pub fn get(&self) -> User {
        users::table.filter(users::username.eq(&self.username))
            .filter(users::password.eq(&self.password))
            .first(&db()).expect("Error getting user")
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

#[table_name = "folders"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone, Identifiable, Associations)]
#[belongs_to(User)]
pub struct Folder {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
}