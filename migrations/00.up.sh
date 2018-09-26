ROOT="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null && pwd )"

psql analytics -f $ROOT/00/tables.sql
psql analytics -f $ROOT/00/types.sql
psql analytics -f $ROOT/00/roles.sql
psql analytics -f $ROOT/00/add_session.sql
psql analytics -f $ROOT/00/update_session.sql
psql analytics -f $ROOT/00/permissions.sql