-- DBNAME = url_shorter
DROP TABLE IF EXISTS url_info;
CREATE TABLE url_info  (
  id serial NOT NULL,
  long_url varchar(511) NOT NULL,
  mur_hash_code varchar(255) NOT NULL,
  insert_at varchar(255) NOT NULL,
  latest_visit_at varchar(255) NOT NULL,
  visit_count INTEGER NOT NULL,
  is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
	UNIQUE(long_url)
)
