CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;

CREATE ROLE carl WITH
    NOSUPERUSER
    INHERIT
    NOCREATEROLE
    NOCREATEDB
    LOGIN
    NOREPLICATION
    NOBYPASSRLS;
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

GRANT ALL ON TABLE session TO carl;
GRANT ALL ON SEQUENCE session_id TO carl;
GRANT ALL ON TABLE cookie TO carl;
GRANT ALL ON SEQUENCE cookie_id TO carl;
GRANT ALL ON TABLE ip_address TO carl;
GRANT ALL ON SEQUENCE ip_address_id TO carl;

ALTER DEFAULT PRIVILEGES FOR ROLE postgres GRANT ALL ON TABLES  TO carl;

CREATE TYPE tp_cookie as (
    id INTEGER,
    token UUID
);

CREATE OR REPLACE FUNCTION new_cookie() RETURNS tp_cookie
    LANGUAGE plpgsql
    AS $_$
DECLARE ret tp_cookie;
BEGIN

    ret.token = (uuid_generate_v4());
    INSERT INTO cookie (cookie)
		VALUES (ret.token) 
		RETURNING id INTO ret.id;
    RETURN ret;
END;
$_$;

CREATE TYPE initial_response as (
    token UUID,
    visit UUID
);

CREATE OR REPLACE FUNCTION get_cookie_for_ip(ip TEXT) RETURNS tp_cookie
    LANGUAGE plpgsql
    AS $_$
DECLARE ret tp_cookie;
BEGIN
    SELECT c.id, c.cookie
        INTO ret.id, ret.token
    FROM cookie c
        LEFT JOIN ip_address i
        ON i.cookie_id = c.id
    WHERE i.ip_address = ip;

    CASE WHEN ret.token IS NULL THEN ret := (SELECT new_cookie());
    END CASE;

    RETURN ret;
END;
$_$;

CREATE FUNCTION add_session(token UUID, ip TEXT, referrer TEXT,  page TEXT, start TIMESTAMP WITH TIME ZONE) RETURNS initial_response
    LANGUAGE plpgsql
    AS $_$
DECLARE ret initial_response;
DECLARE new_cookie tp_cookie;
DECLARE new_visit_token UUID;
BEGIN
    new_visit_token := uuid_generate_v4();
    CASE WHEN token IS NULL THEN
            new_cookie = (SELECT get_cookie_for_ip(ip));
            INSERT INTO session (cookie_id, referrer, page, start, visit_token)
            VALUES (new_cookie.id, referrer, page, start, new_visit_token);
            ret.token = new_cookie.token;
            ret.visit = new_visit_token;
    ELSE
            SELECT id, cookie
                INTO new_cookie.id, new_cookie.token
            FROM cookie
            WHERE cookie.cookie = token;

            INSERT INTO session (cookie_id, referrer, page, start, visit_token)
            VALUES (new_cookie.id, referrer, page, start, new_visit_token);
            ret.token = new_cookie.token;
            ret.visit = new_visit_token;
    END CASE;

    RETURN ret;
END;
$_$;

CREATE FUNCTION add_exit_info(visit_arg UUID, time_arg BIGINT, link_arg TEXT) RETURNS INTEGER
    LANGUAGE plpgsql
    AS $_$
DECLARE ret INTEGER := 0;
BEGIN
    UPDATE session
        SET time_on_page = time_arg,
        internal_link = link_arg
    WHERE visit_token = visit_arg
    RETURNING id INTO ret;
    RETURN ret;
END;
$_$;
