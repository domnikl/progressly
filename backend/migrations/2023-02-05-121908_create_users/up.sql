CREATE TABLE users (
	id uuid PRIMARY KEY,
	email VARCHAR(200) NOT NULL,
	UNIQUE (email)
);

-- insert sample user
INSERT INTO users (id, email) VALUES ('c79f9e52-86eb-4de6-be78-a7a097b8f516', 'test@example.com');
