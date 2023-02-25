use cargo_get_docs::*;

fn main() {
    color_eyre::install().unwrap();
    run(Config::default()).unwrap();
}
