use std::fs;

fn main() {
    let paths = fs::read_dir(".").unwrap();

    for path in paths {

        println!("Name is {}", path.unwrap().path().display())

    }
}
