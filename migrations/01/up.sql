ALTER TABLE session ADD COLUMN site VARCHAR(255) NULL;
ALTER TABLE session ADD COLUMN user_agent VARCHAR(255) NULL;

CREATE OR REPLACE FUNCTION public.add_session(
	token_arg uuid,
	ip_arg text,
	referrer_arg text,
	page_arg text,
	start_arg timestamp with time zone,
	prev_arg uuid,
    site_arg text,
    agent_arg text
    )
    RETURNS initial_response
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE 
AS $BODY$
DECLARE ret initial_response;
DECLARE new_cookie tp_cookie;
DECLARE visit_token UUID;
BEGIN
    CASE WHEN token_arg IS NULL THEN
        ret := (SELECT add_session_no_cookie(ip_arg, referrer_arg, page_arg, start_arg, prev_arg, site_arg, agent_arg));
    ELSE
        SELECT id, cookie
            INTO new_cookie.id, new_cookie.token
        FROM cookie
        WHERE cookie.cookie = token_arg;

        CASE WHEN new_cookie.id IS NULL THEN
            new_cookie := (SELECT get_cookie_for_ip(ip_arg));
        ELSE

        END CASE;
        ret.visit = (SELECT new_session(new_cookie.id, referrer_arg, page_arg, start_arg, prev_arg, site_arg, agent_arg));
        ret.token = new_cookie.token;
        PERFORM ensure_ip_stored(ip_arg, new_cookie.id);
    END CASE;

    RETURN ret;
END;
$BODY$;

ALTER FUNCTION public.add_session(uuid, text, text, text, timestamp with time zone, uuid, text)
    OWNER TO rfm;


CREATE OR REPLACE FUNCTION public.add_session_no_cookie(
	ip_arg text,
	referrer_arg text,
	page_arg text,
	start_arg timestamp with time zone,
	prev_arg uuid,
    site_arg text,
    agent_arg text
    )
    RETURNS initial_response
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE 
AS $BODY$
DECLARE ret initial_response;
DECLARE new_cookie tp_cookie;
BEGIN
    new_cookie = (SELECT get_cookie_for_ip(ip_arg));
    ret.visit = new_session(new_cookie.id, referrer_arg, page_arg, start_arg, prev_arg, site_arg, agent_arg);
    ret.token = new_cookie.token;
    RETURN ret;
END;
$BODY$;

ALTER FUNCTION public.add_session_no_cookie(text, text, text, timestamp with time zone, uuid, text)
    OWNER TO rfm;


CREATE OR REPLACE FUNCTION public.new_session(
	cookie_id_arg integer,
	referrer_arg text,
	page_arg text,
	start_arg timestamp with time zone,
	prev_arg uuid,
    site_arg text,
    agent_arg text)
    RETURNS uuid
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE 
AS $BODY$
DECLARE ret UUID;
BEGIN
    INSERT INTO session (cookie_id, referrer, page, start, prev_visit_token, site)
    VALUES (cookie_id_arg, referrer_arg, page_arg, start_arg, prev_arg, site_arg, agent_arg)
    RETURNING visit_token INTO ret;
    RETURN ret;
END;
$BODY$;

ALTER FUNCTION public.new_session(integer, text, text, timestamp with time zone, uuid, text)
    OWNER TO rfm;