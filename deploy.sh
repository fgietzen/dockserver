#!/bin/bash

function set_version {
	echo "Set version to $version"
	sed -i "s/^version = .*/version = \"$1\"/" Cargo.toml
}

if test "$(git diff --raw)"; then
	echo "Commit all changes first!"
	exit 1
fi

read -p "Enter the new version: " version
if test ! "$(echo $version | grep -E -o '[0-9]+\.[0-9]+\.[0-9]+')"; then
	echo "Wrong version format! Must be X.Y.Z."
	exit 1
fi

echo "Update dependencies (Cargo.lock)"
cargo update
git add Cargo.lock
git commit -m "Update dependencies"

set_version $version
git add Cargo.toml
git commit -m "Release of version $version"
git tag "$version"

readarray -d "." -t v <<< $version
next_version="${v[0]}.${v[1]}.$(expr ${v[2]} + 1)-dev"
set_version $next_version
git add Cargo.toml
git commit -m "Next dev version"
