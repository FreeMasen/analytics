# psql analytics -f ./migrations/00.down.sql
ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"
psql analytics -f "$ROOT/00/add_session.down.sql"
psql analytics -f "$ROOT/00/update_session.down.sql"
psql analytics -f "$ROOT/00/types.down.sql"
psql analytics -f "$ROOT/00/tables.down.sql"
psql analytics -f "$ROOT/00/permissions.down.sql"
psql analytics -f "$ROOT/00/roles.down.sql"
