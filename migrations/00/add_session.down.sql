DROP FUNCTION IF EXISTS get_cookie_for_ip(TEXT);
DROP FUNCTION IF EXISTS add_session(UUID, TEXT, TEXT, TEXT, TIMESTAMP WITH TIME ZONE);
DROP FUNCTION IF EXISTS add_session_no_cookie(TEXT, TEXT, TEXT, TIMESTAMP WITH TIME ZONE);
DROP FUNCTION IF EXISTS new_session(INTEGER, TEXT, TEXT, TIMESTAMP WITH TIME ZONE);
DROP FUNCTION IF EXISTS get_cookie(UUID);