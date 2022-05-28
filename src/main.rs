use std::fs;

fn main() {
    if let Ok(entries) = fs::read_dir(".") {
        for entry in entries {
            if let Ok(entry) = entry {
                // Here, `entry` is a `DirEntry`.
                if let Ok(metadata) = entry.metadata() {
                    // Now let's show our entry's permissions!
                    println!("{:?} {:?} bytes", entry.path(), metadata.len());
                } else {
                    println!("Couldn't get metadata for {:?}", entry.path());
                }
            }
        }
    }
    

}
