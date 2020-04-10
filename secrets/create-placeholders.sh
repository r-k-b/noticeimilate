#!/usr/bin/env bash

set -e

# The directory this script is in
# https://stackoverflow.com/q/59895/2014893
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

touch "$DIR"/postgres_password
touch "$DIR"/gatekeeper_db_user_pw
touch "$DIR"/fetcher.toml
