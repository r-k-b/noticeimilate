#!/usr/bin/env bash

# A convenience for creating files that should only exist locally, and
# therefore aren't tracked in git.

set -e

# The directory this script is in
# https://stackoverflow.com/q/59895/2014893
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

# Files that should contain zero linefeeds
touch "$DIR"/postgres_password
touch "$DIR"/gatekeeper_db_user_pw

# Files that have structured contents (json / yml / toml / ini etc)
if [ ! -f "$DIR"/fetcher.toml ]; then cat >"$DIR"/fetcher.toml <<EOF
[db]
password = ""
EOF
fi
