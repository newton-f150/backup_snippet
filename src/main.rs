use backup::backup;

fn main() {
    println!("Starting backup server...");

    backup::run();
}