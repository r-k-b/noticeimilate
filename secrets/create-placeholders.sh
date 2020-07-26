#!/usr/bin/env bash

# A convenience for creating files that should only exist locally, and
# therefore aren't tracked in git.

set -e

# The directory this script is in
# https://stackoverflow.com/q/59895/2014893
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

# Files that should contain zero linefeeds
mkEmpty () {
	touch "$DIR"/$1
	echo Created single-line empty file: "$DIR"/$1
}
mkEmpty postgres_password
mkEmpty gatekeeper_db_user_pw

#debugging
rm -f "$DIR"/fetcher.toml
rm -f "$DIR"/replenisher.toml


# Files that have structured contents (json / yml / toml / ini etc)

mkWithHeredoc () {
	if [ ! -f "$DIR"/"$1" ]; then
		cat >"$DIR"/"$1";
		echo Created template file: "$DIR"/"$1";
	else
		/dev/null;
	fi
}

mkWithHeredoc fetcher.toml <<EOF
[db]
password = ""
EOF

mkWithHeredoc replenisher.toml <<EOF
[db]
password = ""
EOF
