CREATE TABLE documents (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id),
  folder_id INTEGER NOT NULL REFERENCES folders(id),
  file_path VARCHAR NOT NULL,
  date_created TIMESTAMP,
  date_modified TIMESTAMP
)