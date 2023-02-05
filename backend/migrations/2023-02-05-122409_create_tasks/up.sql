CREATE TABLE tasks (
    id uuid PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    user_id uuid NOT NULL,
    project_id uuid NOT NULL,
    color VARCHAR(10) NOT NULL,
    CONSTRAINT user_fk FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT project_fk FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE ON UPDATE NO ACTION
);

INSERT INTO tasks (id, name, user_id, project_id, color)
VALUES ('14306523-6de0-439d-a26c-06ee5dac7573', 'Sample task', 'c79f9e52-86eb-4de6-be78-a7a097b8f516', '56cd1086-a8f0-41a5-9131-c0b4a42f6e5c', '#332F2C');
