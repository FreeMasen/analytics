CREATE TYPE tp_cookie as (
    id INTEGER,
    token UUID
);

CREATE TYPE initial_response as (
    token UUID,
    visit UUID
);

