CREATE FUNCTION update_session(
    visit_arg UUID,
    time_arg BIGINT,
    link_arg TEXT
) RETURNS INTEGER
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
