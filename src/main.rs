use std::fs;

enum CodeBlockKind {
    NotInCodeBlock,
    Backticks,
    Tildes,
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
    let mut stack: Vec<Line> = Vec::new();
    let mut in_code = CodeBlockKind::NotInCodeBlock;
    let mut line_no = 0;
    for line in contents.lines() {
        line_no = line_no + 1;
        if line.starts_with("```") {
            in_code = match in_code {
                CodeBlockKind::NotInCodeBlock => CodeBlockKind::Backticks,
                CodeBlockKind::Backticks => CodeBlockKind::NotInCodeBlock,
                _ => panic!(),
            }
        }
        if line.starts_with("~~~") {
            in_code = match in_code {
                CodeBlockKind::NotInCodeBlock => CodeBlockKind::Tildes,
                CodeBlockKind::Tildes => CodeBlockKind::NotInCodeBlock,
                _ => panic!(),
            }
        }
        if match in_code {
            CodeBlockKind::NotInCodeBlock => false,
            _ => true,
        } {
            continue
        }
        if line.starts_with("#") && line.contains(" ") {
            let item: Line = Line::split(line);
            let title = item.title.clone();
            let level = item.level;
            let item_type = 0x60 + item.level;
            if stack.len() == 0 {
                stack.insert(0, item);
            } else if stack[0].level < item.level {
                stack.insert(0, item);
            } else {
                while stack.len() > 0 && stack[0].level >= item.level {
                    stack.remove(0);
                }
                stack.insert(0, item);
            }
            #[allow(unused_mut)]
            let mut scopes: Vec<String> = Vec::new();
            for each in stack.clone() {
                scopes.insert(0, each.title.clone());
            }
            scopes.pop();
            let scopes_str = scopes.join("::");
            #[allow(unused_assignments)]
            let mut plevel = 0;
            if stack.len() < 2 {
                if level > 1 {
                    plevel = level - 1;
                } else {
                    plevel = 0;
                }
            } else {
                let parent = &stack[1];
                plevel = parent.level;
            }
            let mut scope: String = String::new();
            if !scopes_str.is_empty() {
                scope = format!("h{}:{}", plevel, scopes_str);
            }
            println!(
                "{}\t{}\t/^{}$/;\"\t{}\tline:{}\t{}",
                title,
                canonicalize(path),
                line,
                item_type as char,
                line_no,
                scope.as_str()
            );
        }
    }
}

#[derive(Clone)]
struct Line {
    title: String,
    level: u8,
}

impl Line {
    fn split(line: &str) -> Line {
        let idx = line.find(' ').unwrap();
        Line {
            // @question 如何从字符串取切片并转换为 String
            title: String::from(line)[idx + 1..].to_string(),
            level: idx as u8,
        }
    }
}

#[allow(dead_code)]
fn is_head(line: &str) -> bool {
    if line.starts_with("# ") {
        true
    } else if line.starts_with("## ") {
        true
    } else {
        false
    }
}
#[cfg(not(windows))]
fn canonicalize(path: &String) -> String {
    fs::canonicalize(path).unwrap().to_str().unwrap().to_string()
}

#[cfg(windows)]
fn canonicalize(path: &String) -> String {
    // Real fs::canonicalize on Windows produces UNC paths which cl.exe is
    // unable to handle in includes. Use a poor approximation instead.
    // https://github.com/rust-lang/rust/issues/42869
    // https://github.com/alexcrichton/cc-rs/issues/169
    fs::canonicalize(path).unwrap().to_str().unwrap()[4..].to_string()
}
