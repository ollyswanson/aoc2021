#!/usr/bin/env bash
if ! [[ $1 =~ ^[0-9]{1,2}$ ]] ; then
  echo "Not a valid arg" >&2; exit
fi

day=day$(printf "%02d" "$1")

cargo run -r --bin "$day"
