CREATE FUNCTION add_session(
    token_arg UUID,
    ip_arg TEXT,
    referrer_arg TEXT,
    page_arg TEXT,
    start_arg TIMESTAMP WITH TIME ZONE
) RETURNS initial_response
    LANGUAGE plpgsql
    AS $_$
DECLARE ret initial_response;
DECLARE new_cookie tp_cookie;
DECLARE visit_token UUID;
BEGIN
    CASE WHEN token_arg IS NULL THEN
        ret := (SELECT add_session_no_cookie(ip_arg, referrer_arg, page_arg, start_arg));
    ELSE
        SELECT id, cookie
            INTO new_cookie.id, new_cookie.token
        FROM cookie
        WHERE cookie.cookie = token_arg;

        CASE WHEN new_cookie.id IS NULL THEN
            new_cookie := (SELECT get_cookie_for_ip(ip_arg));
        ELSE

        END CASE;
        ret.visit = (SELECT new_session(new_cookie.id, referrer_arg, page_arg, start_arg));
        ret.token = new_cookie.token;
    END CASE;

    RETURN ret;
END;
$_$;

CREATE FUNCTION add_session_no_cookie(
    ip_arg TEXT,
    referrer_arg TEXT,
    page_arg TEXT,
    start_arg TIMESTAMP WITH TIME ZONE
) RETURNS initial_response
    LANGUAGE plpgsql
    AS $_$
DECLARE ret initial_response;
DECLARE new_cookie tp_cookie;
BEGIN
    new_cookie = (SELECT get_cookie_for_ip(ip));
    ret.visit = new_session(new_cookie.id, referrer_arg, page_arg, start_arg);
    ret.token = new_cookie.token;
    RETURN ret;
END;
$_$;

CREATE FUNCTION new_session(
    cookie_id_arg INTEGER,
    referrer_arg TEXT,
    page_arg TEXT,
    start_arg TIMESTAMP WITH TIME ZONE
) RETURNS UUID
    LANGUAGE plpgsql
    AS $_$
DECLARE ret UUID;
BEGIN
    INSERT INTO session (cookie_id, referrer, page, start)
    VALUES (cookie_id_arg, referrer_arg, page_arg, start_arg)
    RETURNING visit_token INTO ret;
    RETURN ret;
END;
$_$;

CREATE FUNCTION get_cookie(
    token_arg UUID
) RETURNS tp_cookie
    LANGUAGE plpgsql
    AS $_$
DECLARE ret tp_cookie;
BEGIN
    SELECT id, cookie
        INTO ret.id, ret.token
    FROM cookie c
    WHERE c.cookie = token_arg;
    CASE WHEN ret.id IS NULL OR ret.token IS NULL THEN
        ret := NULL;
    ELSE

    END CASE;
    RETURN ret;
END;
$_$;

CREATE FUNCTION get_cookie_for_ip(
    ip_arg TEXT
) RETURNS tp_cookie
    LANGUAGE plpgsql
    AS $_$
DECLARE ret tp_cookie;
BEGIN
    SELECT c.id, cookie
        INTO ret.id, ret.token
    FROM cookie c
    LEFT JOIN ip_address i
        ON c.id = i.cookie_id
    WHERE i.ip_address = ip_arg
    LIMIT 1;
    CASE WHEN ret.id IS NULL OR ret.token IS NULL THEN

        INSERT INTO cookie (cookie) VALUES (uuid_generate_v4())
        RETURNING id, cookie INTO ret.id, ret.token;
        INSERT INTO ip_address (ip_address, cookie_id)
        VALUES (ip_arg, ret.id);
    ELSE
    
    END CASE;
    RETURN ret;
END;
$_$;

