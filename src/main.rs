use std::fs;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

enum CodeBlockKind {
    NotInCodeBlock,
    Backticks,
    Tildes,
}

#[derive(Clone)]
struct HeadingItem {
    title: String,
    level: u8,
}

impl HeadingItem {
    fn split(line: &str) -> HeadingItem {
        let v: Vec<&str> = line.splitn(2, ' ').collect();
        HeadingItem {
            title: v[1].to_string(),
            level: v[0].len() as u8,
        }
    }
}

#[test]
fn test_heading_item_split_parses_simple_heading() {
    let item = HeadingItem::split("# h1");
    assert_eq!(item.level, 1);
    assert_eq!(item.title, "h1");
}

#[test]
fn test_heading_item_split_parses_heading_contains_spaces() {
    let item = HeadingItem::split("# h 1");
    assert_eq!(item.level, 1);
    assert_eq!(item.title, "h 1");
}

#[cfg(not(windows))]
fn canonicalize(path: &String) -> String {
    fs::canonicalize(path)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

#[cfg(windows)]
fn canonicalize(path: &String) -> String {
    // Real fs::canonicalize on Windows produces UNC paths which cl.exe is
    // unable to handle in includes. Use a poor approximation instead.
    // https://github.com/rust-lang/rust/issues/42869
    // https://github.com/alexcrichton/cc-rs/issues/169
    fs::canonicalize(path).unwrap().to_str().unwrap()[4..].to_string()
}

fn update_in_code(line: &str, in_code: &mut CodeBlockKind) {
    if line.starts_with("```") {
        *in_code = match in_code {
            CodeBlockKind::NotInCodeBlock => CodeBlockKind::Backticks,
            CodeBlockKind::Backticks => CodeBlockKind::NotInCodeBlock,
            _ => panic!(),
        }
    }
    if line.starts_with("~~~") {
        *in_code = match in_code {
            CodeBlockKind::NotInCodeBlock => CodeBlockKind::Tildes,
            CodeBlockKind::Tildes => CodeBlockKind::NotInCodeBlock,
            _ => panic!(),
        }
    }
}

fn is_heading(line: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new("^#+ .*").unwrap();
    }
    RE.is_match(line)
}

#[test]
fn test_is_heading_p() {
    assert_eq!(is_heading("text"), false);
}

#[test]
fn test_is_heading_h1() {
    assert_eq!(is_heading("# h1"), true);
}

#[test]
fn test_is_heading_h2() {
    assert_eq!(is_heading("## h2"), true);
}

#[test]
fn test_is_heading_hashtag() {
    assert_eq!(is_heading("#tag"), false);
}

#[test]
fn test_is_heading_hashtag_with_space() {
    assert_eq!(is_heading("#tag with space"), false);
}

fn process_heading(line: &str, path: &String, stack: &mut Vec<HeadingItem>, line_no: u8) {
    let item: HeadingItem = HeadingItem::split(line);

    while stack.len() > 0 && stack[0].level >= item.level {
        stack.remove(0);
    }

    let plevel = if stack.len() > 0 { stack[0].level } else { 0 };
    let scopes_str = stack
        .iter()
        .map(|x| x.title.clone())
        .rev()
        .collect::<Vec<String>>()
        .join("::");
    let scope = if stack.len() > 0 {
        format!("h{}:{}", plevel, scopes_str)
    } else {
        String::new()
    };

    let item_type = 0x60 + item.level;

    println!(
        "{}\t{}\t/^{}$/;\"\t{}\tline:{}\t{}",
        item.title.clone(),
        canonicalize(path),
        line,
        item_type as char,
        line_no,
        scope
    );

    stack.insert(0, item);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Not enough arguments");
    }
    print!("!_TAG_FILE_FORMAT       2       /extended format; --format=1 will not append ;\" to lines/'
!_TAG_FILE_SORTED       0       /0=unsorted, 1=sorted, 2=foldcase/'
!_TAG_PROGRAM_AUTHOR    wsdjeg /wsdkeg@outlook.com/'
!_TAG_PROGRAM_NAME      mdctags        //'
!_TAG_PROGRAM_URL       https://github.com/wsdjeg/mdctags /official site/'
!_TAG_PROGRAM_VERSION   0.1.0   //'
");
    // 转换 path 至绝对路径
    let path = &args[1];

    let contents = fs::read_to_string(path).expect("");
    let mut stack: Vec<HeadingItem> = Vec::new();
    let mut in_code = CodeBlockKind::NotInCodeBlock;
    let mut line_no = 0;
    for line in contents.lines() {
        line_no = line_no + 1;

        update_in_code(line, &mut in_code);
        if match in_code {
            CodeBlockKind::NotInCodeBlock => false,
            _ => true,
        } {
            continue;
        }

        if is_heading(line) {
            process_heading(line, path, &mut stack, line_no);
        }
    }
}
