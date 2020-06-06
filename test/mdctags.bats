#!/usr/bin/env bats

load test/test_helper.bash

@test "If there are no arguments, it terminates abnormally" {
  run exec_mdctags
  [ "$status" -ne 0 ]
}

@test "If the argument is given and the file does not exist, it terminates abnormally" {
  local markdown_file
  markdown_file=$(get_markdown_file)
  rm -f "$markdown_file"
  run exec_mdctags "$markdown_file"
  [ "$status" -ne 0 ]
}

@test "If an empty file is given, only exclamation mark lines will be output" {
  local markdown_file tags_file expect_file
  markdown_file=$(get_markdown_file)
  tags_file=$(get_tags_file)
  expect_file=$(get_expect_file)

  ensure run_mdctags "$markdown_file" "$tags_file"

  cat <<'__EXPECT__' >"$expect_file"
!_TAG_FILE_FORMAT       2       /extended format; --format=1 will not append ;" to lines/'
!_TAG_FILE_SORTED       0       /0=unsorted, 1=sorted, 2=foldcase/'
!_TAG_PROGRAM_AUTHOR    wsdjeg /wsdkeg@outlook.com/'
!_TAG_PROGRAM_NAME      mdctags        //'
!_TAG_PROGRAM_URL       https://github.com/wsdjeg/mdctags /official site/'
!_TAG_PROGRAM_VERSION   0.1.0   //'
__EXPECT__

  diff -u "$tags_file" "$expect_file"
}

@test "If the level of the headline is 7 or higher, it ends normally" {
  local markdown_file tags_file expect_file
  markdown_file=$(get_markdown_file)
  tags_file=$(get_tags_file)
  expect_file=$(get_expect_file)

  cat <<'__MARKDOWN__' >"$markdown_file"
# hd1
## hd2
### hd3
#### hd4
##### hd5
###### hd6
####### hd7
######## hd8
__MARKDOWN__

  run_ok "$markdown_file" "$tags_file"

  cat <<'__EXPECT__' >"$expect_file"
hd1<tab>/a.md<tab>/^# hd1$/;"<tab>a<tab>line:1<tab>
hd2<tab>/a.md<tab>/^## hd2$/;"<tab>b<tab>line:2<tab>h1:hd1
hd3<tab>/a.md<tab>/^### hd3$/;"<tab>c<tab>line:3<tab>h2:hd1::hd2
hd4<tab>/a.md<tab>/^#### hd4$/;"<tab>d<tab>line:4<tab>h3:hd1::hd2::hd3
hd5<tab>/a.md<tab>/^##### hd5$/;"<tab>e<tab>line:5<tab>h4:hd1::hd2::hd3::hd4
hd6<tab>/a.md<tab>/^###### hd6$/;"<tab>f<tab>line:6<tab>h5:hd1::hd2::hd3::hd4::hd5
hd7<tab>/a.md<tab>/^####### hd7$/;"<tab>g<tab>line:7<tab>h6:hd1::hd2::hd3::hd4::hd5::hd6
hd8<tab>/a.md<tab>/^######## hd8$/;"<tab>h<tab>line:8<tab>h7:hd1::hd2::hd3::hd4::hd5::hd6::hd7
__EXPECT__

  diff -u "$tags_file" "$expect_file"
}

@test "If the level is insufficient, make the appropriate heading the parent heading" {
  local markdown_file tags_file expect_file
  markdown_file=$(get_markdown_file)
  tags_file=$(get_tags_file)
  expect_file=$(get_expect_file)

  cat <<'__MARKDOWN__' >"$markdown_file"
# hd1
### hd3
__MARKDOWN__

  run_ok "$markdown_file" "$tags_file"

  cat <<'__EXPECT__' >"$expect_file"
hd1<tab>/a.md<tab>/^# hd1$/;"<tab>a<tab>line:1<tab>
hd3<tab>/a.md<tab>/^### hd3$/;"<tab>c<tab>line:2<tab>h1:hd1
__EXPECT__

  diff -u "$tags_file" "$expect_file"
}

@test "Code blocks with backticks are ignored" {
  local markdown_file tags_file expect_file
  markdown_file=$(get_markdown_file)
  tags_file=$(get_tags_file)
  expect_file=$(get_expect_file)

  cat <<'__MARKDOWN__' >"$markdown_file"
# hd1a
```
# in code block
```
# hd1b
__MARKDOWN__

  run_ok "$markdown_file" "$tags_file"

  cat <<'__EXPECT__' >"$expect_file"
hd1a<tab>/a.md<tab>/^# hd1a$/;"<tab>a<tab>line:1<tab>
hd1b<tab>/a.md<tab>/^# hd1b$/;"<tab>a<tab>line:5<tab>
__EXPECT__

  diff -u "$tags_file" "$expect_file"
}

@test "Code blocks with tildes are ignored" {
  local markdown_file tags_file expect_file
  markdown_file=$(get_markdown_file)
  tags_file=$(get_tags_file)
  expect_file=$(get_expect_file)

  cat <<'__MARKDOWN__' >"$markdown_file"
# hd1a
~~~
# in code block
~~~
# hd1b
__MARKDOWN__

  run_ok "$markdown_file" "$tags_file"

  cat <<'__EXPECT__' >"$expect_file"
hd1a<tab>/a.md<tab>/^# hd1a$/;"<tab>a<tab>line:1<tab>
hd1b<tab>/a.md<tab>/^# hd1b$/;"<tab>a<tab>line:5<tab>
__EXPECT__

  diff -u "$tags_file" "$expect_file"
}

@test "Normal exit" {
  local markdown_file tags_file expect_file
  markdown_file=$(get_markdown_file)
  tags_file=$(get_tags_file)
  expect_file=$(get_expect_file)

  cat <<'__MARKDOWN__' >"$markdown_file"
# hd1
## hd2
### hd3
#### hd4
##### hd5
###### hd6
__MARKDOWN__

  run_ok "$markdown_file" "$tags_file"

  cat <<'__EXPECT__' >"$expect_file"
hd1<tab>/a.md<tab>/^# hd1$/;"<tab>a<tab>line:1<tab>
hd2<tab>/a.md<tab>/^## hd2$/;"<tab>b<tab>line:2<tab>h1:hd1
hd3<tab>/a.md<tab>/^### hd3$/;"<tab>c<tab>line:3<tab>h2:hd1::hd2
hd4<tab>/a.md<tab>/^#### hd4$/;"<tab>d<tab>line:4<tab>h3:hd1::hd2::hd3
hd5<tab>/a.md<tab>/^##### hd5$/;"<tab>e<tab>line:5<tab>h4:hd1::hd2::hd3::hd4
hd6<tab>/a.md<tab>/^###### hd6$/;"<tab>f<tab>line:6<tab>h5:hd1::hd2::hd3::hd4::hd5
__EXPECT__

  diff -u "$tags_file" "$expect_file"
}


@test "If the previous level is larger, it ends normally" {
  local markdown_file tags_file expect_file
  markdown_file=$(get_markdown_file)
  tags_file=$(get_tags_file)
  expect_file=$(get_expect_file)

  cat <<'__MARKDOWN__' >"$markdown_file"
## hd2
# hd1
__MARKDOWN__

  run_ok "$markdown_file" "$tags_file"

  cat <<'__EXPECT__' >"$expect_file"
hd2<tab>/a.md<tab>/^## hd2$/;"<tab>b<tab>line:1<tab>
hd1<tab>/a.md<tab>/^# hd1$/;"<tab>a<tab>line:2<tab>
__EXPECT__

  diff -u "$tags_file" "$expect_file"
}

@test "If the previous level is more larger, it ends normally" {
  local markdown_file tags_file expect_file
  markdown_file=$(get_markdown_file)
  tags_file=$(get_tags_file)
  expect_file=$(get_expect_file)

  cat <<'__MARKDOWN__' >"$markdown_file"
### hd3
# hd1
__MARKDOWN__

  run_ok "$markdown_file" "$tags_file"

  cat <<'__EXPECT__' >"$expect_file"
hd3<tab>/a.md<tab>/^### hd3$/;"<tab>c<tab>line:1<tab>
hd1<tab>/a.md<tab>/^# hd1$/;"<tab>a<tab>line:2<tab>
__EXPECT__

  diff -u "$tags_file" "$expect_file"
}

@test "If the previous level is same, it ends normally" {
  local markdown_file tags_file expect_file
  markdown_file=$(get_markdown_file)
  tags_file=$(get_tags_file)
  expect_file=$(get_expect_file)

  cat <<'__MARKDOWN__' >"$markdown_file"
# hd1
## hd2a
## hd2b
__MARKDOWN__

  run_ok "$markdown_file" "$tags_file"

  cat <<'__EXPECT__' >"$expect_file"
hd1<tab>/a.md<tab>/^# hd1$/;"<tab>a<tab>line:1<tab>
hd2a<tab>/a.md<tab>/^## hd2a$/;"<tab>b<tab>line:2<tab>h1:hd1
hd2b<tab>/a.md<tab>/^## hd2b$/;"<tab>b<tab>line:3<tab>h1:hd1
__EXPECT__

  diff -u "$tags_file" "$expect_file"
}
