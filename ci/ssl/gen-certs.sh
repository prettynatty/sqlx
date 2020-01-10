#!/usr/bin/env bash

openssl req -new -x509 -days 365 -nodes -text -out server.crt \
  -keyout server.key -subj "/CN=db.sqlx.test"

openssl req -new -nodes -text -out root.csr \
  -keyout root.key -subj "/CN=root.sqlx.test"

# chmod og-rwx root.key

openssl x509 -req -in root.csr -text -days 3650 \
  -extfile /etc/ssl/openssl.cnf -extensions v3_ca \
  -signkey root.key -out root.crt

openssl req -new -nodes -text -out server.csr \
  -keyout server.key -subj "/CN=db.sqlx.test"

# chmod og-rwx server.key

openssl x509 -req -in server.csr -text -days 365 \
  -CA root.crt -CAkey root.key -CAcreateserial \
  -out server.crt
