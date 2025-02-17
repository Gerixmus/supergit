mod git_operations;
mod standard;

fn main() {
    if let Err(err) = standard::run_standard() {
        eprintln!("❌ Error: {}", err);
        std::process::exit(1);
    }
}