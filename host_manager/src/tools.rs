use debug_print::debug_print;
use debug_print::debug_println;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs::{self, DirEntry};
use std::io;
use std::io::{Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
pub fn hexdump(label: &str, data: &[u8]) {
    debug_println!("{}", label);
    for (i, byte) in data.iter().enumerate() {
        debug_print!("{:02x} ", byte);
        if (i + 1) % 16 == 0 {
            debug_println!();
        }
    }
    debug_println!("\n");
}

pub fn write_to_file(path: &str, data: &[u8]) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

pub fn read_from_file(path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}

pub fn save_vec64_to_file(path: String, vector: Vec<u64>) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)
        .unwrap();
    for address in vector {
        writeln!(file, "0x{:X}", address).unwrap();
    }
}
pub fn delete_file_if_exists(file_path: &str) -> std::io::Result<()> {
    let path = Path::new(file_path);

    if path.exists() {
        fs::remove_file(path)?;
        debug_println!("File '{}' has been deleted.", file_path);
    } else {
        debug_println!("File '{}' does not exist.", file_path);
    }

    Ok(())
}

pub fn remove_files_in_folder<P: AsRef<Path>>(folder_path: P) -> io::Result<()> {
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}
pub fn reset_folder(id: u32) -> std::io::Result<()> {
    // Construct the path using the user ID
    let path = format!("/samba/users/user{}", id);

    // Delete the folder if it exists
    if fs::metadata(&path).is_ok() {
        fs::remove_dir_all(&path)?;
    }

    // Create the folder
    fs::create_dir_all(&path)?;

    // Change permissions to 777
    let permissions = fs::Permissions::from_mode(0o777);
    fs::set_permissions(&path, permissions)?;

    Ok(())
}
