#!/usr/bin/env bash

openssl req -new -x509 -days 365 -nodes -text -out $PGDATA/server.crt \
  -keyout $PGDATA/server.key -subj "/CN=sqlx.test"

chmod og-rwx $PGDATA/server.key

/usr/bin/pg_ctl reload
