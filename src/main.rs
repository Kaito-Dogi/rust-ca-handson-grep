use std::fs::read_to_string;

use structopt::StructOpt;

// #[~(...)]：アトリビュート（他の言語におけるアノテーション）
#[derive(StructOpt)]
#[structopt(name = "rsgrep")]
// 構造体
struct GrepArgs {
    #[structopt(name = "PATTERN")]
    pattern: String,
    #[structopt(name = "PATH")]
    // Vec：サイズが可変の配列
    path: Vec<String>
}

// コンストラクタ
// impl GrepArgs {
//     fn new(path: String, pattern: String) -> GrepArgs {
//         GrepArgs {
//             pattern: pattern,
//             path: path,
//         }
//     }
// }

fn grep(content: String, pattern: &String) {
    for line in content.lines() {
        if line.contains(pattern.as_str()) {
            println!("{}", line);
        }
    }
}

// fn run(path: String, pattern: String) {
//     match read_to_string(path) {
//         Ok(content) => grep(content, pattern),
//         Err(reason) => println!("{}", reason),
//     }
// }

fn run(state: GrepArgs) {
    for path in state.path.iter() {
        match read_to_string(path) {
            Ok(content) => grep(content, &state.pattern),
            Err(reason) => println!("{}", reason),
        }
    }
}

// fn main() {
//     let pattern = std::env::args().nth(1);
//     let path = std::env::args().nth(2);

//     match (pattern, path) {
//         // (Some(pattern), Some(path)) => run(path, pattern),
//         (Some(pattern), Some(path)) => run(GrepArgs::new(path, pattern)),
//         _ => println!("No path is specified!"),
//     }
// }

fn main() {
    run(GrepArgs::from_args());
}
