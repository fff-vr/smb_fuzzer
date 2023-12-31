use rayon::prelude::*;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::process::Command;
use std::collections::HashMap;
use std::path::Path;
fn main(){
    let args: Vec<String> = std::env::args().collect();
    if let Some(coverage_path) = args.get(1) {
        coverage_to_lines(coverage_path.to_string()).unwrap();
        lines_to_html().unwrap();
    } else {
        println!("no coverage path");
        std::process::exit(1);
    }
}
fn lines_to_html()->io::Result<()>{
    let data_file_path = "source_lines.txt";
    let mut line_numbers = HashMap::new();

    let file = File::open(data_file_path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 2 {
            let file_path = parts[0].to_string();
            let line_number_part = parts[1].split_whitespace().next().unwrap_or("0");
            let line_number: usize = line_number_part.parse().unwrap_or(0);
            line_numbers.entry(file_path).or_insert_with(Vec::new).push(line_number);
        }
    }

    for (file_path, lines) in line_numbers {

        match File::open(&file_path) {
            Ok(file) => {

                let new_path = &file_path.replace("~/target/", "./");
                let new_path = format!("{}.html",new_path);
                let path = Path::new(&new_path);
                let html_file_path = path.clone();
                if let Some(dir) = path.parent() {
                    fs::create_dir_all(dir)?;
                }
                println!("save at {}",html_file_path.display());
                let mut html_file = File::create(&html_file_path)?;
                writeln!(html_file, "<html><body><pre><code>")?;

                let reader = BufReader::new(file);
                for (index, line) in reader.lines().enumerate() {
                    let line = line?;
                    if lines.contains(&(index + 1)) {
                        writeln!(html_file, "<span style='background-color: red;'>{}</span>", html_escape(&line))?;
                    } else {
                        writeln!(html_file, "{}", html_escape(&line))?;
                    }
                }

                writeln!(html_file, "</code></pre></body></html>")?;
            },
            Err(_) => {
                println!("Failed to open file: {}", file_path);
            }
        }
    }

    Ok(())

}
fn html_escape(input: &str) -> String {
    input.replace("&", "&amp;")
         .replace("<", "&lt;")
         .replace(">", "&gt;")
         .replace("\"", "&quot;")
         .replace("'", "&#39;")
}
fn coverage_to_lines(coverage_path : String) -> io::Result<()> {
    let chunk_size = 3000; 
    let addresses: Vec<_> = BufReader::new(File::open(coverage_path)?)
        .lines()
        .filter_map(Result::ok)
        .collect();

    let chunks: Vec<_> = addresses.chunks(chunk_size).enumerate().collect();
    let num = chunks.len();
    chunks.into_par_iter().for_each(|(index, chunk)| {
        process_chunk(chunk, index).unwrap();
    });

    remove_duplicates_and_save(num)?;

    Ok(())
}

fn process_chunk(chunk: &[String], chunk_index: usize) -> io::Result<()> {
    let output = Command::new("addr2line")
        .arg("-e")
        .arg("~/target/linux/vmlinux")
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

