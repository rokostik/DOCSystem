mod schema {
    infer_schema!("dotenv:DATABASE_URL");
}

use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use bcrypt::{hash, verify};
use base64::decode;
use std::io::prelude::*;
use std::io::Write;
use std::fs::{File, OpenOptions};
use chrono::*;

use self::schema::*;

fn db() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[table_name = "users"]
#[derive(Serialize, Queryable, Insertable, FromForm, Debug, Clone, Identifiable, Associations, AsChangeset)]
#[has_many(folders)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub surname: String,
    pub password: String,
}

impl User {
    pub fn get(id: i32) -> User {
        users::table.find(id).first(&db()).expect("Error getting user.")
    }

    pub fn get_folders(&self) -> Vec<Folder> {
        Folder::belonging_to(self).load(&db()).expect("Error getting folders.")
    }

    pub fn new_folder(&self, folder_name: &str) -> bool {
        let folder_new = FolderNew { user_id: self.id, name: String::from(folder_name) };
        self.log_access(format!("added folder"));
        diesel::insert(&folder_new).into(folders::table).execute(&db()).is_ok()
    }

    pub fn new_document(&self, folder_name: String, document: DocumentForm) -> bool {
        let folder = Folder::get_folder_by_name(folder_name);
        let file_bytes = &decode(&document.file_b64).unwrap();

        let mut buffer = File::create(["static/documents/", &document.file].join("")).unwrap()  ;
        buffer.write(&file_bytes);

        let document_new = DocumentNew { user_id: self.id, folder_id: folder.id,
                                         file_path: document.file, file_name: document.file_name };
        self.log_access(format!("added document"));
        diesel::insert(&document_new).into(documents::table).execute(&db()).is_ok()
    }

    pub fn update_profile(&mut self, updated_profile: UserNew) {
        self.name = updated_profile.name;
        self.surname = updated_profile.surname;
        self.username = updated_profile.username;
        let hashed_password = match hash(updated_profile.password.as_str(), 10) {
            Ok(h) => h,
            Err(_) => panic!()
        };
        self.password = hashed_password;

        let _: User = self.save_changes(&db()).expect("Error updating user");
        self.log_access(format!("updated profile"));
    }

    fn log_access(&self, action: String) {
        let mut file = OpenOptions::new()
                        .write(true)
                        .append(true)
                        .create(true)
                        .open("log/access_log.txt")
                        .unwrap();

        if let Err(e) = writeln!(file, "{} {} {}", UTC::now().format("%Y-%m-%d %H:%M:%S").to_string(), self.username, action) {
            println!("{}", e);
        }
    }
}

#[derive(FromForm, Debug, Clone)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

impl UserLogin {
    pub fn get(&self) -> User {
        let user: User = users::table.filter(users::username.eq(&self.username))
                    .first(&db()).expect("Error getting user.");

        match verify(self.password.as_str(), &user.password) {
            Ok(_) => {
                user.log_access(format!("logged in"));
                return user;
            }
            Err(_) => panic!("Incorrect password")
        };
    }

    pub fn validate(&self) -> bool {
        !self.username.is_empty() && self.username.len() < 20 &&
        !self.password.is_empty() && self.password.len() < 60
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
    pub fn insert(&mut self) -> bool {
        let hashed_password = match hash(self.password.as_str(), 10) {
            Ok(h) => h,
            Err(_) => panic!()
        };

        self.password = hashed_password;
        diesel::insert(self).into(users::table).execute(&db()).is_ok()
    }

    pub fn validate(&self) -> bool {
        !self.username.is_empty() && self.username.len() < 20 &&
        !self.name.is_empty() && self.name.len() < 20 &&
        !self.surname.is_empty() && self.surname.len() < 20 &&
        !self.password.is_empty() && self.password.len() < 60
    }
}

#[table_name = "folders"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone, Identifiable, Associations)]
#[belongs_to(User)]
#[has_many(documents)]
pub struct Folder {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
}

impl Folder {
    pub fn get_documents(&self) -> Vec<Document> {
        Document::belonging_to(self).load(&db()).expect("Error getting documents.")
    }

    pub fn get_document_by_name(&self, document_name: String) -> Document {
        Document::belonging_to(self).filter(documents::file_name.eq(document_name))
                                    .first(&db()).expect("Error getting document.")
    }

    pub fn get_folder_by_name(name: String) -> Self{
        folders::table.filter(folders::name.eq(name))
                    .first(&db()).expect("Error getting folder.")
    }
}

#[derive(Serialize, FromForm, Debug, Clone)]
pub struct FolderForm {
    pub folder_name: String,
}

impl FolderForm {
    pub fn validate(&self) -> bool {
        !self.folder_name.is_empty() && self.folder_name.len() < 15 && self.folder_name.find(" ") == None
    }
}

#[table_name = "folders"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone, Associations)]
pub struct FolderNew {
    pub user_id: i32,
    pub name: String,
}


#[table_name = "documents"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone, Identifiable, Associations)]
#[belongs_to(Folder)]
pub struct Document {
    pub id: i32,
    pub user_id: i32,
    pub folder_id: i32,
    pub file_path: String,
    pub file_name: String,
}

#[table_name = "documents"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone, Associations)]
pub struct DocumentNew {
    pub user_id: i32,
    pub folder_id: i32,
    pub file_path: String,
    pub file_name: String,
}

#[derive(Serialize, FromForm, Debug, Clone)]
pub struct DocumentForm {
    pub file: String,
    pub file_name: String,
    pub file_b64: String,
}

impl DocumentForm {
    pub fn validate(&self) -> bool {
        !self.file.is_empty() && self.file.len() < 50 &&
        !self.file_name.is_empty() && self.file_name.len() < 20 && self.file_name.find(" ") == None &&
        !self.file_b64.is_empty()
    }
}