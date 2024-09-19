CREATE TABLE IF NOT EXISTS users (
id SERIAL PRIMARY KEY,
    full_name VARCHAR(150) NOT NULL,
    email VARCHAR(100) NOT NULL,
    password VARCHAR(256) NOT NULL DEFAULT '',
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS groups (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS group_roles (
    group_id INTEGER NOT NULL REFERENCES groups(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    role_id INTEGER NOT NULL REFERENCES roles(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    CONSTRAINT group_roles_pkey PRIMARY KEY (group_id, role_id)
);

CREATE TABLE IF NOT EXISTS user_groups (
    user_id INTEGER NOT NULL REFERENCES users(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    group_id INTEGER NOT NULL REFERENCES groups(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    CONSTRAINT user_groups_pkey PRIMARY KEY (user_id, group_id)
);

CREATE TABLE IF NOT EXISTS certifications (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS certification_roles (
    certification_id INTEGER NOT NULL REFERENCES certifications(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    role_id INTEGER NOT NULL REFERENCES roles(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    CONSTRAINT certification_roles_pkey PRIMARY KEY (certification_id, role_id)
);

CREATE TABLE IF NOT EXISTS events (
    id SERIAL PRIMARY KEY,
    certification_id INTEGER NOT NULL REFERENCES certifications(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    name VARCHAR(100) NOT NULL,
    description VARCHAR(500) NOT NULL DEFAULT '',
    type INTEGER NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS event_schedules (
    id SERIAL PRIMARY KEY,
    event_id INTEGER NOT NULL REFERENCES events(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    event_start TIMESTAMP NOT NULL,
    event_end TIMESTAMP NOT NULL,
    event_count INTEGER NOT NULL DEFAULT 1,
    event_delay_sec INTEGER NOT NULL DEFAULT 3600,
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS event_logs (
    id SERIAL PRIMARY KEY,
    event_id INTEGER NOT NULL REFERENCES events(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    event_start TIMESTAMP NOT NULL,
    event_end TIMESTAMP NOT NULL,
    event_result INTEGER NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS checklists (
    id SERIAL PRIMARY KEY,
    certification_id INTEGER NOT NULL REFERENCES certifications(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    name VARCHAR(100) NOT NULL,
    description VARCHAR(500) NOT NULL DEFAULT '',
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS checklist_items (
    id SERIAL PRIMARY KEY,
    checklist_id INTEGER REFERENCES checklists(id) ON UPDATE NO ACTION ON DELETE NO ACTION,
    name VARCHAR(100) NOT NULL,
    description VARCHAR(500) NOT NULL DEFAULT '',
    checklist_order INTEGER NOT NULL DEFAULT 0,
    enabled BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- SEED
INSERT INTO users (full_name, email, password, enabled)
VALUES ('John Doe', 'john.doe@easyiso.com.br', 'q1w2e3r4', true);

INSERT INTO groups (name, enabled)
VALUES ('Admin', true);

INSERT INTO roles (name, enabled)
VALUES ('full-access', true);

INSERT INTO group_roles(group_id, role_id)
VALUES (1, 1);

INSERT INTO user_groups(user_id, group_id)
VALUES (1, 1);

INSERT INTO certifications (name, enabled)
VALUES ('ISO-14001', true);

INSERT INTO certification_roles (certification_id, role_id)
VALUES (1, 1);

INSERT INTO checklists (certification_id, name, description, enabled)
VALUES (1, 'Coleta de lixo', 'Checklist para averiguar se todos as etapas de uma coleta seletiva de lixo foram concluidas corretamente', true);

INSERT INTO checklist_items (checklist_id, name, description, checklist_order, enabled)
VALUES
    (1, 'O acesso ao lixo estava fechado corretamente quando coleta iniciou?', '', 0, true),
    (1, 'O ambiente estava organizado?', '', 1, true)
;