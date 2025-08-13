
# SafeBackup CLI – CYB225 Secure File Utility

## 📌 Overview

SafeBackup CLI is a secure, memory-safe file management utility written in **Rust** for **CYB225 Secure Coding**.

It supports:

* **Backup** a file to a specified directory
* **Retrieve** (restore) a file from a backup
* **Delete** a file (with optional confirmation)
* **Logging** of all actions and errors to `logfile.txt` for audit purposes

The tool is built with **modern secure coding practices**:

* **Memory safety** – guaranteed by Rust’s ownership model
* **Secure file operations** – prevents path traversal (`..`) and invalid filenames
* **Robust error handling** – uses `Result` and `Option` patterns so no errors are ignored
* **Audit logging** – all successes and failures logged with RFC3339 timestamps

This aligns with **Australian cybersecurity guidance** including the [ACSC Essential Eight](https://www.cyber.gov.au/acsc/essential-eight) and the Notifiable Data Breaches Scheme under the OAIC.

---

## ✨ Features

* Rejects malicious filename input (e.g., `../../etc/passwd`)
* Logs every operation for compliance and incident response
* Works in both local and cloud-hosted file systems
* Cross-platform (Windows, macOS, Linux with Rust installed)

---

## 📂 Directory Structure

```
Mati_SafeFolder/
├── sample.txt
├── data.txt
├── backup_dir/       # Backup storage location
├── logfile.txt       # Generated audit log
└── safebackup_cli/   # Rust project folder
    ├── Cargo.toml
    └── src/
        └── main.rs
```

---

## ⚙️ Installation

**Clone the repository**

```bash
git clone https://github.com/<yourusername>/safebackup_cli.git
cd safebackup_cli
```

**Add dependency**

```bash
cargo add chrono
```

**Build the release binary**

```bash
cargo build --release
```

**Executable Location**

```
target/release/safebackup_cli       # Linux / macOS
target/release/safebackup_cli.exe   # Windows
```

---

## 💻 Usage

**Backup a file**

```bash
./safebackup_cli backup sample.txt backup_dir
```

**Retrieve a backed-up file**

```bash
./safebackup_cli retrieve backup_dir/sample.txt
```

**Delete a file**

```bash
./safebackup_cli delete data.txt
```

**Syntax**

```
safebackup_cli <command> <file> [backup_directory]
```

**Commands**

| Command    | Parameters                         | Description                          |
| ---------- | ---------------------------------- | ------------------------------------ |
| `backup`   | `<source_file> <backup_directory>` | Copies file to backup location       |
| `retrieve` | `<backup_file>`                    | Restores file from backup            |
| `delete`   | `<file_path>`                      | Deletes file (optional confirmation) |

---

## 🧪 Test Cases

| # | Scenario                      | Command                              | Expected Result                   |
| - | ----------------------------- | ------------------------------------ | --------------------------------- |
| 1 | Backup valid file             | `backup sample.txt backup_dir`       | File copied, success logged       |
| 2 | Retrieve valid file           | `retrieve backup_dir/sample.txt`     | Content printed, success logged   |
| 3 | Delete valid file             | `delete data.txt`                    | File deleted, success logged      |
| 4 | Delete cancelled (if enabled) | `delete data.txt` → `no`             | Cancelled, log shows cancellation |
| 5 | Path traversal attempt        | `backup ../../etc/passwd backup_dir` | Rejected, path traversal logged   |
| 6 | Invalid filename              | `delete inva\|id?.txt`               | Rejected, invalid filename logged |
| 7 | Retrieve non-existent file    | `retrieve nofile.txt`                | Error, failure logged             |
| 8 | Delete non-existent file      | `delete nofile.txt`                  | Error, failure logged             |

---

## 📝 Logging

All actions and errors are appended to `logfile.txt` in the format:

```
YYYY-MM-DDTHH:MM:SS+TZ, <command>, <status>
```

**Example**

```
2025-08-13T16:25:00+10:00, backup, Success
2025-08-13T16:26:15+10:00, delete, Invalid input - path traversal detected
```

---

## 🔒 Security Considerations

| Security Measure      | Purpose                                           |
| --------------------- | ------------------------------------------------- |
| **Input validation**  | Prevents directory traversal and unsafe filenames |
| **Least privilege**   | Executes only the requested operation             |
| **Complete auditing** | Meets ASD Essential Eight monitoring requirements |
| **Data privacy**      | Prevents unintended data disclosure               |

---

## 📜 License

MIT License – For academic and educational use.

---


