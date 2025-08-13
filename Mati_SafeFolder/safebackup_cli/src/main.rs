use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use chrono::Local;
use std::env;
use std::process;

/// Log actions with timestamp into logfile.txt
fn log_action(action: &str, status: &str) {
    let timestamp = Local::now().to_rfc3339();
    let entry = format!("{}, {}, {}\n", timestamp, action, status);
    let mut logfile = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logfile.txt")
        .expect("Could not open logfile.txt");
    logfile.write_all(entry.as_bytes()).unwrap();
}

/// Checks that a path is safe (blocks path traversal)
fn is_valid_path(input: &str) -> bool {
    if input.contains("..")
        || Path::new(input)
            .components()
            .any(|c| c == std::path::Component::ParentDir)
    {
        return false;
    }
    true
}

/// Backup command - copies file to backup folder
fn backup(file_path: &str, backup_dir: &str) -> Result<(), String> {
    if !is_valid_path(file_path) {
        log_action("backup", "Invalid input - path traversal detected");
        return Err("Invalid file path".to_string());
    }

    let path = Path::new(file_path);
    if !path.is_file() {
        log_action("backup", "Invalid input - file does not exist");
        return Err("Source file does not exist or is not a file".to_string());
    }

    let mut src_file = File::open(path).map_err(|e| {
        log_action("backup", &format!("Failed to open source file: {}", e));
        e.to_string()
    })?;

    let mut content = Vec::new();
    src_file.read_to_end(&mut content).map_err(|e| {
        log_action("backup", &format!("Failed to read source file: {}", e));
        e.to_string()
    })?;

    let backup_path = Path::new(backup_dir).join(path.file_name().unwrap());
    let mut backup_file = File::create(&backup_path).map_err(|e| {
        log_action("backup", &format!("Failed to create backup file: {}", e));
        e.to_string()
    })?;

    backup_file.write_all(&content).map_err(|e| {
        log_action("backup", &format!("Failed to write backup file: {}", e));
        e.to_string()
    })?;

    log_action("backup", "Success");
    println!("Backup successful: {}", backup_path.display());
    Ok(())
}

/// Retrieve command - reads and prints backup file content
fn retrieve(backup_file: &str) -> Result<(), String> {
    if !is_valid_path(backup_file) {
        log_action("retrieve", "Invalid input - path traversal detected");
        return Err("Invalid file path".to_string());
    }

    let path = Path::new(backup_file);
    let mut file = File::open(path).map_err(|e| {
        log_action("retrieve", &format!("Failed to open backup file: {}", e));
        e.to_string()
    })?;

    let mut content = Vec::new();
    file.read_to_end(&mut content).map_err(|e| {
        log_action("retrieve", &format!("Failed to read backup file: {}", e));
        e.to_string()
    })?;

    log_action("retrieve", "Success");
    println!("Retrieved content ({} bytes):", content.len());
    println!("{}", String::from_utf8_lossy(&content));
    Ok(())
}

/// Delete command - deletes given file
fn delete(file_path: &str) -> Result<(), String> {
    if !is_valid_path(file_path) {
        log_action("delete", "Invalid input - path traversal detected");
        return Err("Invalid file path".to_string());
    }

    println!("Are you sure you want to delete '{}'? (yes/no):", file_path);

    let mut confirm = String::new();
    std::io::stdin().read_line(&mut confirm).map_err(|e| {
        format!("Failed to read input: {}", e)
    })?;

    let confirm = confirm.trim().to_lowercase();
    if confirm != "yes" {
        println!("Delete operation cancelled.");
        log_action("delete", "Cancelled by user");
        return Ok(());
    }

    std::fs::remove_file(file_path).map_err(|e| {
        log_action("delete", &format!("Failed to delete file: {}", e));
        e.to_string()
    })?;

    log_action("delete", "Success");
    println!("File deleted successfully");
    Ok(())
}


/// Show usage info
fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  utility backup <source_file> <backup_directory>");
    eprintln!("  utility retrieve <backup_file>");
    eprintln!("  utility delete <file_path>");
}

/// Entry point
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let command = &args[1];

    let result = match command.as_str() {
        "backup" => {
            if args.len() != 4 {
                eprintln!("backup command requires source file and backup directory");
                print_usage();
                process::exit(1);
            }
            backup(&args[2], &args[3])
        }
        "retrieve" => {
            if args.len() != 3 {
                eprintln!("retrieve command requires backup file path");
                print_usage();
                process::exit(1);
            }
            retrieve(&args[2])
        }
        "delete" => {
            if args.len() != 3 {
                eprintln!("delete command requires file path");
                print_usage();
                process::exit(1);
            }
            delete(&args[2])
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            print_usage();
            process::exit(1);
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
