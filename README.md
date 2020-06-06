## mdctags

> tags for markdown file

### Screenshot

![2017-02-01_1359x723](https://cloud.githubusercontent.com/assets/13142418/22514376/12f8a792-e8da-11e6-9897-fb0136732a31.png)

### Config mdctags for tagbar

```viml
let g:tagbar_type_markdown = {
            \ 'ctagsbin'  : 'mdctags',
            \ 'ctagsargs' : '',
            \ 'kinds'     : [
            \     'a:h1:0:0',
            \     'b:h2:0:0',
            \     'c:h3:0:0',
            \     'd:h4:0:0',
            \     'e:h5:0:0',
            \     'f:h6:0:0',
            \ ],
            \ 'sro'        : '::',
            \ 'kind2scope' : {
            \     'a' : 'h1',
            \     'b' : 'h2',
            \     'c' : 'h3',
            \     'd' : 'h4',
            \     'e' : 'h5',
            \     'f' : 'h6',
            \ },
            \ 'scope2kind' : {
            \     'h1' : 'a',
            \     'h2' : 'b',
            \     'h3' : 'c',
            \     'h4' : 'd',
            \     'h5' : 'e',
            \     'h6' : 'f',
            \}
            \}
```

### Installation

```sh
git clone https://github.com/wsdjeg/mdctags.rs.git
cd ./mdctags.rs
cargo build --release
cp ./target/release/mdctags /path/to/your/bin
```

If you have cross, you can cross-compile.

```sh
git clone https://github.com/wsdjeg/mdctags.rs.git
cd ./mdctags.rs
cargo install cross
cross build --release --target x86_64-pc-windows-gnu
  # --> target/x86_64-pc-windows-gnu/release/mdctags.exe
```

### Thanks

- [tagbar-markdown](https://github.com/lvht/tagbar-markdown)

### License

MIT

### Author

Wang Shidong
