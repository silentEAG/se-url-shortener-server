create table url_info (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	long_url VARCHAR(255) NOT NULL,
    mur_hash_code VARCHAR(255) NOT NULL,
    insert_at VARCHAR(255) NOT NULL,
    latest_visit_at VARCHAR(255) NOT NULL,
    visit_count VARCHAR(255) NOT NULL,
    is_deleted BOOLEAN NOT NULL
);

insert into url_info values('test1','https://silente.top', 'xxx', '114', '514', '0', false);