use std::env;
use std::fs;
use std::process;

fn main() {
    // 收集命令列參數到 args 向量

    let args: Vec<String> = env::args().collect();

    // 嘗試從參數建立 Config 結構，失敗則顯示錯誤並退出
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // --snip-- 這裡可以放後續邏輯

    // 顯示搜尋關鍵字與檔案路徑
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // 讀取指定檔案內容
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    // 輸出檔案內容
    println!("With text:\n{contents}");
}

// 儲存搜尋設定的結構
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // 從命令列參數建立 Config，參數不足時回傳錯誤
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}