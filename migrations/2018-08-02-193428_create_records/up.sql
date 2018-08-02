-- Your SQL goes here

CREATE TABLE records (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255) NOT NULL,
  description TEXT NOT NULL,
  update_date DATE NOT NULL,
  crawl_time DATETIME NOT NULL
)
