# SafeBackup CLI – CYB235 Secure File Utility

## Overview

**SafeBackup CLI** is a secure, memory-safe file management utility written in **Rust** for the **CYB235 Cloud Security** course.  

It supports:
- **Backup** a file to a specified directory  
- **Retrieve** (restore) a file from a backup  
- **Delete** a file (with optional confirmation)  
- **Logging** of all actions and errors to `logfile.txt` for audit purposes  

**Security Highlights:**
- **Memory safety** – ensured by Rust’s ownership model  
- **Secure file operations** – prevents path traversal (`..`) and invalid filenames  
- **Robust error handling** – uses `Result` and `Option` patterns to handle failures  
- **Audit logging** – logs successes and failures with RFC3339 timestamps  

This approach aligns with Australian cybersecurity best practices, including the [ACSC Essential Eight](https://www.cyber.gov.au/acsc/essential-eight) and OAIC’s Notifiable Data Breaches Scheme.

---

## Features

- Rejects malicious filename input (e.g., `../../etc/passwd`)
- Logs every operation for compliance and incident response
- Works in both local and cloud-hosted environments
- Cross-platform support (Windows, macOS, Linux with Rust installed)

---

## Directory Structure

Mati_SafeFolder/
├── sample.txt
├── data.txt
├── backup_dir/ # Backup storage location
├── logfile.txt # Generated audit log
└── safebackup_cli/ # Rust project folder
├── Cargo.toml
└── src/
└── main.rs

yaml
Copy
Edit

---

## Installation

Clone the repository:
```bash
git clone https://github.com/<yourusername>/safebackup_cli.git
cd safebackup_cli
Add dependency:

bash
Copy
Edit
cargo add chrono
Build the release binary:

bash
Copy
Edit
cargo build --release
The executable will be located at:

arduino
Copy
Edit
target/release/safebackup_cli(.exe)
Usage
From the folder containing your test files:

Backup a file:

bash
Copy
Edit
./safebackup_cli backup sample.txt backup_dir
Retrieve a backed-up file:

bash
Copy
Edit
./safebackup_cli retrieve backup_dir/sample.txt
Delete a file:

bash
Copy
Edit
./safebackup_cli delete data.txt
Syntax:

php-template
Copy
Edit
safebackup_cli <command> <file> [backup_directory]
Commands:

backup <source_file> <backup_directory>

retrieve <backup_file>

delete <file_path>

Test Cases
#	Scenario	Command	Expected Result
1	Backup valid file	backup sample.txt backup_dir	File copied, Success logged
2	Retrieve valid file	retrieve backup_dir/sample.txt	Content printed, Success logged
3	Delete valid file	delete data.txt	Deleted, Success logged
4	Delete cancelled (if enabled)	delete data.txt → no	Cancelled, log shows cancellation
5	Path traversal attempt	backup ../../etc/passwd backup_dir	Rejected, log: path traversal detected
6	Invalid filename	delete inva|id?.txt	Rejected, invalid filename logged
7	Retrieve non-existent file	retrieve nofile.txt	Error, failure logged
8	Delete non-existent file	delete nofile.txt	Error, failure logged

Logging
All actions and errors are appended to logfile.txt:

ruby
Copy
Edit
YYYY-MM-DDTHH:MM:SS+TZ, <command>, <status>
Example:

pgsql
Copy
Edit
2025-08-13T16:25:00+10:00, backup, Success
2025-08-13T16:26:15+10:00, delete, Invalid input - path traversal detected
Security Considerations
Input validation – prevents directory traversal or unsafe filenames

Least privilege – executes only the requested operation

Complete auditing – supports ASD Essential Eight monitoring and response

Data privacy – no unintended data exposure
