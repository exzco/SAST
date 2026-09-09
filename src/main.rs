use std::fs;
use std::path::Path;
use std::time::Instant;

mod lexer;
mod parser;
pub mod semantic;
use lexer::{Lexer, Token, TokenKind};
use parser::Parser;

use std::collections::HashMap;

fn main() {
    // let target_dir = "../testdata";
    let target_dir = "./src/example";
    println!("🚀 开始批量扫描目录: {}\n", target_dir);

    let mut total = 0;
    let mut passed = 0;
    let mut failed = 0;
    let mut error_map: HashMap<String, (usize, Vec<String>)> = HashMap::new();
    let start_time = Instant::now();

    scan_directory(Path::new(target_dir), &mut total, &mut passed, &mut failed, &mut error_map);

    println!("\n==========================================");
    println!("📊 扫描结果统计:");
    println!("总文件数: {}", total);
    println!("✅ 成功: {}", passed);
    println!("❌ 失败: {}", failed);
    println!("🎯 通过率: {:.2}%", (passed as f64 / total.max(1) as f64) * 100.0);
    println!("⏱️ 总耗时: {:?}", start_time.elapsed());
    println!("\n🔥 Top 失败原因分布:");
    let mut err_vec: Vec<_> = error_map.into_iter().collect();
    err_vec.sort_by(|a, b| b.1.0.cmp(&a.1.0));
    for (err, (count, samples)) in err_vec.iter().take(15) {
        println!("{:>5} 次: {}", count, err);
        for s in samples.iter().take(2) {
            println!("       样例: {}", s);
        }
    }
    println!("==========================================");
}

fn scan_directory(dir: &Path, total: &mut usize, passed: &mut usize, failed: &mut usize, error_map: &mut HashMap<String, (usize, Vec<String>)>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                scan_directory(&path, total, passed, failed, error_map);
            } else if path.extension().is_some_and(|ext| ext == "java") {
                *total += 1;
                let path_str = path.to_string_lossy();
                match test_single_file(&path_str) {
                    Ok(_) => {
                        *passed += 1;
                    }
                    Err(err) => {
                        *failed += 1;
                        let simplified_err = if let Some(idx) = err.find("当前 Token") {
                            err[..idx].trim().to_string()
                        } else {
                            err.clone()
                        };
                        let entry = error_map.entry(simplified_err).or_insert((0, Vec::new()));
                        entry.0 += 1;
                        if entry.1.len() < 3 {
                            entry.1.push(format!("{}: {}", path_str, err));
                        }
                    }
                }
            }
        }
    }
}

fn test_single_file(file_path: &str) -> Result<(), String> {
    let code = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
    let mut lexer = Lexer::new(&code);
    let mut tokens: Vec<Token> = Vec::new();
    loop {
        let token = lexer.next_token();
        let is_eof = token.kind == TokenKind::Eof;
        if token.kind != TokenKind::Comment {
            tokens.push(token);
        }
        if is_eof {
            break;
        }
    }
    let mut parser = Parser::new(tokens);
    // parser.parse_compile_unit().map(|_| ())
    match parser.parse_compile_unit() {
        Ok(_ast) => {
            let cu = &_ast.classes;
            for class in cu {
                println!("解析到类名：{:#?}",class.name)
            }

            
            let content = format!("{:#?}",_ast);
            if let Err(e) = fs::write("src/test", content) {
                eprintln!("[-] 写入 src/test 失败: {}", e);
            } else {
                println!("[+] 已成功将 AST 输出到 src/test 文件");
            }
            
            Ok(())          
        }
        Err(_) => todo!()
    }       
}