use std::{env, path};

fn main() {
    // コマンドライン引数を取得
    let args: Vec<String> = env::args().collect();
    // ディレクトリの初期値を設定
    let mut target_dir = ".";
    // 引数がある場合は、ディレクトリを設定
    if args.len() >= 2 {
        target_dir = &args[1];
    }
    // PathBufに変換
    let target = path::PathBuf::from(target_dir);
    println!("{}", target_dir);
    let mut finished_level: Vec<isize> = Vec::new();
    tree(&target, 0, &mut finished_level);
}

fn tree(target: &path::PathBuf, level: isize, finished_level: &mut Vec<isize>) {
    // ファイル一覧を取得
    let files: Vec<_> = target.read_dir().expect("パスが不正です").collect();
    // そのディレクトリ内のファイルの数
    let count = files.len();
    // ファイル一覧をループ
    for (i, file) in files.into_iter().enumerate() {
        let path = file.unwrap().path();
        // level の数だけプレフィックスを出力
        for j in 0..level {
            if finished_level.contains(&j) {
                print!("    ");
            } else {
                print!("│   ");
            }
        }
        // ファイル名を出力
        let file_name = path.file_name().unwrap().to_string_lossy();
        if path.is_dir() {
            if i == count - 1 {
                println!("└── {}", file_name);
                finished_level.push(level);
                tree(&path, level + 1, finished_level);
                finished_level.pop(); // 再帰呼び出し後に pop する
            } else {
                println!("├── {}", file_name);
                tree(&path, level + 1, finished_level);
            }
        } else {
            if i == count - 1 {
                println!("└── {}", file_name);
            } else {
                println!("├── {}", file_name);
            }
        }
    }
}
