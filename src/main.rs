use std::process::exit;

mod procs;

fn main() {
    let procs = match procs::find() {
        Ok(p) => p,
        Err(err) => {
            eprintln!("Failed to get process list: {}", err);
            exit(1);
        }
    };

    dbg!(procs);
}
