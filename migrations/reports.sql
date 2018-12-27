CREATE TYPE ReferrerCount AS (
    referrer TEXT,
    ct BIGINT
);

ALTER TYPE ReferrerCount
    OWENER TO carl;

--REFERRERS THIS WEEK
CREATE OR REPLACE FUNCTION referrers_this_week()
RETURNS SETOF ReferrerCount AS
$$
    SELECT DISTINCT referrer, count(page) as ct
    FROM session
    WHERE referrer IS NOT NULL
    AND referrer NOT LIKE 'https://wiredforge%'
    AND referrer NOT LIKE 'https://www.wiredforge%'
    AND start > CURRENT_DATE - 7
    GROUP BY referrer
$$
LANGUAGE sql;

ALTER FUNCTION referrers_this_week()
    OWNER TO carl;

CREATE OR REPLACE FUNCTION unique_visits_this_week()
RETURNS SETOF BIGINT AS
$$
    SELECT count(cookie_id) as visit_count
    FROM (SELECT DISTINCT cookie_id
        FROM session
    WHERE start > CURRENT_DATE - 7) a;
$$
LANGUAGE sql;

ALTER FUNCTION unique_visits_this_week()
    OWNER TO carl;

CREATE TYPE PageView AS (
    view_count BIGINT,
    page TEXT
);

ALTER TYPE PageView
    OWNER TO carl;

CREATE OR REPLACE FUNCTION unique_page_view_this_week()
RETURNS SETOF PageView AS
$$
    SELECT count(cookie_id) as view_count, page
    FROM (SELECT DISTINCT cookie_id, page
            FROM session
    WHERE start > CURRENT_DATE - 7) a
    GROUP BY page;
$$
LANGUAGE sql;

ALTER FUNCTION unique_page_view_this_week()
    OWNER TO carl;