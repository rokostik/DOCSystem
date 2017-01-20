use models::*;

#[derive(Debug, Serialize)]
pub struct Context{ user: User,
                    folders: Vec<Folder>,
                    folder_name: Option<String>,
                    documents: Option<Vec<Document>>,
                    document: Option<Document>, }

impl Context {
    pub fn folder_view(user_id: i32, folder_name: Option<String>) -> Context {
        //TODO: user without folders
        let user: User = User::get(user_id);
        let folders: Vec<Folder> = (&user).get_folders();
        match folder_name {
            Some(folder_name) => {
                let folder = folders.clone().into_iter().filter(|folder| folder.name == folder_name).next().unwrap();
                let documents: Vec<Document> = folder.get_documents();
                Context { user: user,
                          folders: folders,
                          folder_name: Some(folder_name),
                          documents: Some(documents),
                          document: None }
            }
            None => {
                Context { user: user,
                          folders: folders,
                          folder_name: None,
                          documents: None,
                          document: None }
            }
        }
    }

    pub fn document_view(user_id: i32, folder_name: String, document_name: String) -> Context {
        let user: User = User::get(user_id);
        let folders: Vec<Folder> = (&user).get_folders();
        let folder = folders.clone().into_iter().filter(|folder| folder.name == folder_name).next().unwrap();
        let document = folder.get_document_by_name(document_name);
        Context { user: user,
                  folders: folders,
                  folder_name: Some(folder_name),
                  documents: None,
                  document: Some(document) }
    }

    pub fn profile_view(user_id: i32) -> Context {
        let user: User = User::get(user_id);
        let folders: Vec<Folder> = (&user).get_folders();
        Context { user: user,
                  folders: folders,
                  folder_name: None,
                  documents: None,
                  document: None }
    }
}