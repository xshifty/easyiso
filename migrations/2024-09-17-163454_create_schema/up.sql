CREATE TABLE IF NOT EXISTS users (
id SERIAL PRIMARY KEY,
    full_name VARCHAR(150) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password VARCHAR(256) NOT NULL DEFAULT '',
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS groups (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50),
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50),
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS group_roles (
    group_id INTEGER REFERENCES groups(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    role_id INTEGER REFERENCES roles(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT group_roles_pkey PRIMARY KEY (group_id, role_id)
);

CREATE TABLE IF NOT EXISTS user_groups (
    user_id INTEGER REFERENCES users(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    group_id INTEGER REFERENCES groups(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT user_groups_pkey PRIMARY KEY (user_id, group_id)
);

CREATE TABLE IF NOT EXISTS certifications (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100),
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS certification_roles (
    certification_id INTEGER REFERENCES certifications(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    role_id INTEGER REFERENCES roles(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT certification_roles_pkey PRIMARY KEY (certification_id, role_id)
);

CREATE TABLE IF NOT EXISTS events (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description VARCHAR(500),
    type INTEGER NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS event_schedules (
    id SERIAL PRIMARY KEY,
    event_id INTEGER REFERENCES events(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    event_start TIMESTAMP NOT NULL,
    event_end TIMESTAMP,
    event_count INTEGER NOT NULL DEFAULT 1,
    event_delay_sec INTEGER NOT NULL DEFAULT 3600,
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS event_logs (
    id SERIAL PRIMARY KEY,
    event_id INTEGER REFERENCES events(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    event_start TIMESTAMP NOT NULL,
    event_end TIMESTAMP NOT NULL,
    event_result INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS checklists (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description VARCHAR(500),
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS checklist_items (
    id SERIAL PRIMARY KEY,
    checklist_id INTEGER REFERENCES checklists(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    name VARCHAR(100) NOT NULL,
    description VARCHAR(500) NOT NULL,
    checklist_order INTEGER NOT NULL DEFAULT 0,
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS certification_checklists (
    certification_id INTEGER REFERENCES certifications(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    checklist_id INTEGER REFERENCES checklists(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT certification_checklists_pkey PRIMARY KEY (certification_id, checklist_id)
);

CREATE TABLE IF NOT EXISTS certification_events (
    certification_id INTEGER REFERENCES certifications(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    event_id INTEGER REFERENCES events(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT certification_events_pkey PRIMARY KEY (certification_id, event_id)
);

-- SEED
INSERT INTO users (full_name, email, password, enabled)
VALUES ('John Doe', 'john.doe@easyiso.com.br', 'q1w2e3r4', true);

INSERT INTO groups (name)
VALUES ('Admin');

INSERT INTO roles (name)
VALUES ('full-access');

INSERT INTO group_roles(group_id, role_id)
VALUES (1, 1);

INSERT INTO user_groups(user_id, group_id)
VALUES (1, 1);

INSERT INTO certifications (name)
VALUES ('ISO-14001');

INSERT INTO certification_roles (certification_id, role_id)
VALUES (1, 1);