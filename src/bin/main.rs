fn main() {
    if let Err(e) = journey::start() {
        eprintln!("{}", e);
    }
}
