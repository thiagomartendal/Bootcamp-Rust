#!/bin/bash

# Credenciais do MySQL
DB_USER="root"
DB_PASS="Node@1234"
DB_HOST="localhost"
DB_NAME="clientes_rust"

# Arquivo SQL
SQL_FILE="restore.sql"

# Conectando-se ao MySQL
mysql -u"$DB_USER" -p"$DB_PASS" -h "$DB_HOST" < "$SQL_FILE"
