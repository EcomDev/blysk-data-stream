#!/bin/bash

set -eo pipefail

env MYSQL_DATABASE=$DB_NAME \
  MYSQL_PASSWORD=$DB_PASS \
  MYSQL_ROOT_PASSWORD=$DB_PASS \
  MYSQL_USER=$DB_USER \
  "$@"
