use piface::PrimaryInterface;

fn main() {
    std::env::set_var("RUST_LOG", "trace");
    env_logger::init();

    let piface = PrimaryInterface::load().unwrap();
    println!("{:#?}", piface);
}
