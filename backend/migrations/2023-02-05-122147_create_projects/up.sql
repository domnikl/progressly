CREATE TABLE projects (
    id uuid PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    user_id uuid NOT NULL,
    color VARCHAR(10) NOT NULL,
    CONSTRAINT user_fk FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE ON UPDATE NO ACTION
);

INSERT INTO projects (id, name, user_id, color)
VALUES ('56cd1086-a8f0-41a5-9131-c0b4a42f6e5c', 'Sample project', 'c79f9e52-86eb-4de6-be78-a7a097b8f516', '#89AC76');
