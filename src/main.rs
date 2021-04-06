use std::process::exit;

mod procs;
mod x11;

fn main() {
    // Get processes.
    let procs = match procs::find() {
        Ok(p) => p,
        Err(err) => {
            eprintln!("Failed to get process list: {}", err);
            exit(1);
        }
    };

    // Get desktop where windows are placed.
    let desktops = match x11::window_desktops(procs.iter().map(|p| p.window)) {
        Ok(d) => d,
        Err(err) => {
            eprintln!("Failed to read X11 properties: {}", err);
            exit(1);
        }
    };

    // Print processes with in a single desktop.
    for proc in procs {
        match desktops.get(&proc.window) {
            Some(item) => {
                if let Some(desktop) = item {
                    println!("{} [{}] {}", proc.window, desktop, proc.cmdline);
                }
            }

            None => {
                eprintln!("Failed to get data for {:?}", proc)
            }
        }
    }
}
