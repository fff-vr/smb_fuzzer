use rayon::prelude::*;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::process::Command;
use std::sync::Mutex;
use std::collections::HashMap;
use std::path::Path;
fn main(){
    coverage_to_lines().unwrap();
    lines_to_html().unwrap();
}
fn lines_to_html()->io::Result<()>{
    let data_file_path = "source_lines.txt";
   let mut line_numbers = HashMap::new();

    let file = File::open(data_file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            let file_path = parts[0];
            let line_number: usize = parts[1].parse().unwrap_or(0);
            line_numbers.insert(file_path.to_string(), line_number);
        }
    }

    for (file_path, line_number) in line_numbers {
        println!("process {}",file_path);

        match File::open(&file_path) {
            Ok(file) => {
                let new_path = &file_path.replace("/home/jjy/target/", "./");
                let path = Path::new(&new_path);
                if let Some(dir) = path.parent() {
                    // 디렉토리가 존재하지 않으면 생성
                    fs::create_dir_all(dir)?;
                }
                let html_file_path = path.with_extension("html");

                let mut html_file = File::create(&html_file_path)?;
                writeln!(html_file, "<html><body><pre>")?;

                let reader = BufReader::new(file);
                for (index, line) in reader.lines().enumerate() {
                    let line = line?;
                    if index + 1 == line_number {
                        writeln!(html_file, "<span style='background-color: red;'>{}</span>", line)?;
                    } else {
                        writeln!(html_file, "{}", line)?;
                    }
                }

                writeln!(html_file, "</pre></body></html>")?;
            },
            Err(_) => {
                println!("Failed to open file: {}", file_path);
            }
        }
    }

    Ok(())
}
fn coverage_to_lines() -> io::Result<()> {
    let chunk_size = 3000; // 청크 크기를 정의합니다.
    let addresses: Vec<_> = BufReader::new(File::open("../coverage.txt")?)
        .lines()
        .filter_map(Result::ok)
        .collect();

    // 주소들을 청크로 나눕니다.
    let chunks: Vec<_> = addresses.chunks(chunk_size).enumerate().collect();
    let num = chunks.len();
    // 각 청크를 병렬로 처리합니다.
    chunks.into_par_iter().for_each(|(index, chunk)| {
        process_chunk(chunk, index).unwrap();
    });

    // 임시 파일들의 결과를 읽고 중복 제거 후 최종 파일에 저장
    remove_duplicates_and_save(num)?;

    Ok(())
}

fn process_chunk(chunk: &[String], chunk_index: usize) -> io::Result<()> {
    let output = Command::new("addr2line")
        .arg("-e")
        .arg("/home/jjy/target/linux/vmlinux") // 실행 파일 경로
        .args(chunk)
        .output()?;

    let temp_file_path = format!("tmp/temp_result_{}.txt", chunk_index);
    fs::write(&temp_file_path, output.stdout)?;

    Ok(())
}

fn remove_duplicates_and_save(chunk_count: usize) -> io::Result<()> {
    let mut unique_results = HashSet::new();

    for i in 0..chunk_count {
        let temp_file_path = format!("tmp/temp_result_{}.txt", i);
        let file = File::open(&temp_file_path)?;
        let reader = BufReader::new(file);
        unique_results.extend(reader.lines().filter_map(Result::ok));
        fs::remove_file(temp_file_path)?;
    }

    let mut final_file = File::create("source_lines.txt")?;
    for result in unique_results {
        writeln!(final_file, "{}", result)?;
    }

    Ok(())
}

