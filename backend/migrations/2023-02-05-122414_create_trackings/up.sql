CREATE TABLE trackings (
    id uuid PRIMARY KEY,
    user_id uuid NOT NULL,
    project_id uuid NOT NULL,
    task_id uuid NULL,
    "start" TIMESTAMPTZ NOT NULL,
    "end" TIMESTAMPTZ NOT NULL,
    CONSTRAINT user_fk FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT project_fk FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE ON UPDATE NO ACTION,
    CONSTRAINT task_fk FOREIGN KEY (task_id) REFERENCES tasks (id) ON DELETE CASCADE ON UPDATE NO ACTION
);
