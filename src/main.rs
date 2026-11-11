use directories::ProjectDirs;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn init_dirs_and_files() -> std::io::Result<PathBuf> {
    match ProjectDirs::from("", "", "rememberthis") {
        Some(dirs) => {
            // Ensure data dir exists
            let data_dir = dirs.data_dir();
            std::fs::create_dir_all(data_dir)?;

            // Ensure text file exists
            let txtfile_path = data_dir.join("rememberthis.txt");
            let _txtfile = OpenOptions::new()
                .append(true)
                .create(true)
                .open(&txtfile_path);
            Ok(txtfile_path)
        }
        None => {
            eprintln!("Failed to determine valid home directory path");
            std::process::exit(1);
        }
    }
}

enum Mode {
    Create,
    Edit,
    List,
}

fn parse_args() -> Mode {
    let args: Vec<String> = std::env::args().collect();
    let mut mode = Mode::Create;
    if args.len() == 1 {
    } else if args.len() == 2 && args[1] == "edit" {
        mode = Mode::Edit;
    } else if args.len() == 2 && args[1] == "list" {
        mode = Mode::List;
    } else {
        eprintln!("Usage: rt [edit | list]");
        std::process::exit(2);
    }

    mode
}

const SEPARATOR: &str = "====#";
const ANOTHER_ENTRY: &str = "====!";

fn input_entry() -> (String, bool) {
    let mut entry = String::new();

    loop {
        let mut line = String::new();
        let num_bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        if num_bytes == 0 {
            return (line, true)
        }
        if line.trim_end() == SEPARATOR {
            break;
        } else if line.trim_end() == ANOTHER_ENTRY {
            return (entry, true);
        }
        entry.push_str(&line);
    }

    (entry, false)
}

fn append_entry(txtfile_path: &PathBuf, entry: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(txtfile_path)?;

    writeln!(file, "{}", SEPARATOR)?;
    write!(file, "{}", entry)?;

    Ok(())
}

fn create(txtfile_path: PathBuf) -> std::io::Result<()> {
    loop {
        let (entry, keep_going) = input_entry();
        if entry.len() == 0 {
            break;
        }
        append_entry(&txtfile_path, entry)?;
        if !keep_going {
            break;
        }
    }

    Ok(())
}

fn edit(txtfile_path: PathBuf) -> std::io::Result<()> {
    #[cfg(not(target_os = "windows"))]
    let editor = "/usr/bin/vi";
    #[cfg(target_os = "windows")]
    let editor = r"C:\Windows\System32\notepad.exe";

    match Command::new(editor).arg(txtfile_path).status() {
        Ok(status) if status.success() => {}
        Ok(status) => {
            eprintln!("Editor exited with status: {status}");
            std::process::exit(1);
        }
        Err(error) => {
            eprintln!("Failed to start editor: {error}");
            std::process::exit(1);
        }
    }

    Ok(())
}

fn list(txtfile_path: PathBuf) -> std::io::Result<()> {
    let contents = fs::read_to_string(txtfile_path)?;
    print!("{contents}");

    Ok(())
}

fn main() -> std::io::Result<()> {
    let txtfile_path = init_dirs_and_files()?;

    match parse_args() {
        Mode::Create => create(txtfile_path),
        Mode::Edit => edit(txtfile_path),
        Mode::List => list(txtfile_path),
    }
}
