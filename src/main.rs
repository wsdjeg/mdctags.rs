use std::fs;
use std::process;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    // process::exit(0);
    println!("!_TAG_FILE_FORMAT       2       /extended format; --format=1 will not append ;\" to lines/'
!_TAG_FILE_SORTED       0       /0=unsorted, 1=sorted, 2=foldcase/'
!_TAG_PROGRAM_AUTHOR    wsdjeg /wsdkeg@outlook.com/'
!_TAG_PROGRAM_NAME      mdctags        //'
!_TAG_PROGRAM_URL       https://github.com/wsdjeg/mdctags /official site/'
!_TAG_PROGRAM_VERSION   0.1.0   //'
");
    if args.len() == 0 {
        process::exit(0);
    }
    // 转换 path 至绝对路径
    let path = "README.md";

    let contents = fs::read_to_string(path).expect("");
    let mut stack: Vec<Line> = Vec::new();
    let mut in_code = false;
    let mut line_no = 0;
    for line in contents.lines() {
        line_no = line_no + 1;
        if line.starts_with("```") {
            in_code = !in_code;
        }
        if !in_code && line.starts_with("#") && line.contains(" ") {
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
            //$scopes = array_map(function ($line) { return $line['title']; }, array_reverse($stack));
            #[allow(unused_mut)]
            let mut scopes: Vec<String> = Vec::new();
            // for each in stack {
                // scopes.insert(0, each.title.clone());
            // }
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
            let scope: String;
            if !scopes_str.is_empty() {
                scope = format!("h{}:{}", plevel, scopes_str);
            }
            println!(
                "{}\t{}\t/^{}$/;\"\t{}\tline:{}\t{}",
                title, path, line, item_type as char, line_no, scope.as_str()
            );
        }
    }
}

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

fn is_head(line: &str) -> bool {
    if line.starts_with("# ") {
        true
    }else if line.starts_with("## ") {
        true
    }else{
        false
    }
}
