use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn process_file(file_path: &Path) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut write_after_cut = false;
    let mut output = String::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains("------------[ cut here ]------------") {
            write_after_cut = true;
            continue;
        }

        if write_after_cut {
            output.push_str(&line);
            output.push('\n');
        }
    }

    // 원본 파일 이름에서 경로를 제거
    let file_name = file_path.file_name().unwrap().to_str().unwrap();
    let output_file_name = format!("./{}_processed.txt", file_name);
    fs::write(output_file_name, output)?;

    Ok(())
}

fn main() -> io::Result<()> {
    for entry in fs::read_dir("../workdir/save")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            process_file(&path)?;
        }
    }

    Ok(())
}

