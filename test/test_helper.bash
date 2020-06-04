#!bash

get_markdown_file() {
  mktemp /tmp/tmp.XXXXXXXXXX.md
}

get_tags_file() {
  mktemp /tmp/tmp.XXXXXXXXXX.tags
}

get_expect_file() {
  mktemp /tmp/tmp.XXXXXXXXXX.expect
}

_filter_remove_exmark_lines() {
  sed -e "/^!/d"
}

_filter_replace_to_fixed_fname() {
  local markdown_file
  markdown_file="$1"; shift
  sed -e "s|${markdown_file}|/a.md|"
}

_filter_replace_tab_to_string() {
  sed -e "s|\t|<tab>|g"
}

filter_tags_for_test() {
  local markdown_file
  markdown_file="$1"; shift
  _filter_remove_exmark_lines \
    | _filter_replace_to_fixed_fname "$markdown_file" \
    | _filter_replace_tab_to_string
}

exec_mdctags() {
  ./target/debug/mdctags "$@"
}

run_mdctags() {
  local markdown_file tags_file2
  markdown_file="$1"; shift
  tags_file2="$1"; shift
  exec_mdctags "$markdown_file" >"$tags_file2"
}

say() {
    printf 'mcdtags-bats: %s\n' "$1"
}

err() {
    say "$1" >&2
    exit 1
}

ensure() {
    if ! "$@"; then err "command failed: $*"; fi
}

run_ok() {
  local markdown_file tags_file tags_file2
  markdown_file="$1"; shift
  tags_file="$1"; shift
  tags_file2=$(get_tags_file)
  ensure run_mdctags "$markdown_file" "$tags_file2"
  cat "$tags_file2" | filter_tags_for_test "$markdown_file" >"$tags_file"
}
