CREATE SEQUENCE migration_id
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

CREATE TABLE migration (
    id INTEGER DEFAULT nextval('migration_id') NOT NULL,
    name varchar(255) NOT NULL,
    applied TIMESTAMP DEFAULT current_timestamp
);

CREATE SEQUENCE session_id
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

CREATE TABLE session (
    id INTEGER DEFAULT nextval('session_id'::regclass) NOT NULL,
    cookie_id INTEGER NOT NULL,
    referrer CHARACTER VARYING(255),
    page CHARACTER VARYING(255) NOT NULL,
    start TIMESTAMP WITH TIME ZONE,
    visit_token UUID DEFAULT uuid_generate_v4() NOT NULL,
    time_on_page BIGINT,
    internal_link CHARACTER VARYING(255)
);

CREATE SEQUENCE cookie_id
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

CREATE TABLE cookie (
    id INTEGER DEFAULT nextval('cookie_id'::regclass) NOT NULL,
    cookie UUID DEFAULT uuid_generate_v4() NOT NULL
);

CREATE SEQUENCE ip_address_id
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;

CREATE TABLE ip_address (
    id INTEGER DEFAULT nextval('ip_address_id') NOT NULL,
    ip_address CHARACTER VARYING(15) NOT NULL,
    cookie_id INTEGER NOT NULL
);

ALTER TABLE ONLY cookie
    ADD CONSTRAINT cookie_key
    PRIMARY KEY (id);

ALTER TABLE ONLY session
    ADD CONSTRAINT session_key
    PRIMARY KEY (id);

ALTER TABLE ONLY session
    ADD CONSTRAINT cookie_fkey
    FOREIGN KEY (cookie_id)
    REFERENCES cookie(id);

ALTER TABLE ONLY ip_address
    ADD CONSTRAINT ip_address_key
    PRIMARY KEY (id);

ALTER TABLE ONLY ip_address
    ADD CONSTRAINT ip_cookie_fkey
    FOREIGN KEY (cookie_id)
    REFERENCES cookie(id);