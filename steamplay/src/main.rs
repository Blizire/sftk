use std::fs;
//use std::path;

fn main() {
    println!("steamplay - 0.0.1");

    // Iterators for each drive on a windows system.
    let drive_iterators = enumerate_drives();

    for drive in drive_iterators {
        enumerate_directories(drive);
    }
}

fn enumerate_directories(dir: std::fs::ReadDir) {
    // assumes its being given an iterator from fs::read_dir, then will use that
    // to enumerate the directories and sub-directories based on the "depth"
    let mut search_paths = Vec::new();
    // flatten is cool, silently drops errors and returns only valid values
    for entry in dir.flatten() {
        match entry.file_type() {
            Ok(e) => {
                if e.is_dir() {
                    search_paths.push(entry);
                }
            },
            Err(_) => {}
        };
    }
    // TODO: implement depth somehow so we can dive into these sub directories
    println!("{:?}", search_paths);
}

fn enumerate_drives() -> Vec<std::fs::ReadDir> {

    // searches for all valid windows drives, returns an iterator that
    // can read through the directory listing.

    let mut valid_drives = Vec::new();

    // generate a vector of strings that represents typical windows disk drives
    // a:/ b:/ c:/ etc...
    let logical_drives: Vec<String> = ('a'..='z')
        .map(|c| format!("{}:/", c))
        .collect();

    for drive in logical_drives {
        match fs::read_dir(&drive) {
            Ok(e) => {
                // pushes an iterator that can read the contents of these directories
                // into a vector for return.
                valid_drives.push(e);
            }
            Err(_) => {
                continue
            }
        }
    }

    valid_drives
}
