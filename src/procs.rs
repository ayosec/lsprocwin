use std::error::Error;
use std::path::Path;
use std::{fs, io, str};

#[derive(Debug)]
pub struct Process {
    pub window: u32,
    pub cmdline: String,
}

/// Find all processes with a WINDOWID variable.
pub fn find() -> Result<Vec<Process>, Box<dyn Error>> {
    let mut procs = Vec::new();

    for entry in fs::read_dir("/proc")?.flat_map(|e| e.ok()) {
        let base = entry.path();
        let environ = base.join("environ");
        if let Ok(environ) = fs::read(environ) {
            let mut last = 0;
            for index in memchr::memchr_iter(0, &environ) {
                if let Some(window) = extract_windowid(&environ[last..index]) {
                    if let Ok(cmdline) = get_cmdline(&base) {
                        procs.push(Process { window, cmdline });
                        break;
                    }
                }

                last = index + 1;
            }
        }
    }

    Ok(procs)
}

/// Extract the value of the WINDOWID variable.
fn extract_windowid(line: &[u8]) -> Option<u32> {
    if let Some(value) = line.strip_prefix(b"WINDOWID=") {
        match str::from_utf8(value).map(str::parse) {
            Ok(Ok(window)) => Some(window),
            _ => None,
        }
    } else {
        None
    }
}

/// Read the command line of a process.
fn get_cmdline<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let cmdline = path.as_ref().join("cmdline");
    let mut content = fs::read(cmdline)?;

    // Replace NUL with spaces.
    for byte in &mut content[..] {
        if *byte == 0 {
            *byte = b' ';
        }
    }

    Ok(String::from_utf8_lossy(&content).trim_end().into())
}
