use std::path::PathBuf;
use std::fs;

fn main() {

    println!("steamplay - 0.0.1");
    
    let mut found_paths = Vec::new();

    // search through all windows drives, and sub-directories and return
    // all paths enumerated N levels deep.
    for root in enumerate_drives() {
        found_paths.extend(
            enumerate_directory_depth(root, 2)
        );
    }

    println!("{:?}", found_paths);
}

fn enumerate_drives() -> Vec<PathBuf> {

    // return a vector of valid paths that represent logical windows drives
    let mut valid_paths = Vec::new();

    // generate strings like typical windows disk drives
    // a:/ b:/ c:/ etc...
    let drive_strings: Vec<String> = ('a'..='z')
        .map(|c| format!("{}:/", c))
        .collect();

    // turn strings into path, use 'exists' to check if valid
    for s in drive_strings {
        let path = PathBuf::from(&s);
        if path.exists() {
            valid_paths.push(path);
        }
    }

    valid_paths
}

fn enumerate_directory(p: PathBuf) -> Vec<PathBuf> {

    // reads a directory and returns the path of the content as a PathBuf vector
    let mut directory_contents = Vec::new();
    if let Ok(reader) = fs::read_dir(p) {
        for entry in reader.flatten() {
            directory_contents.push(entry.path());
        }
    }

    directory_contents
}

fn path_filter_directories<'a>(path: &'a Vec<PathBuf>) -> Vec<PathBuf> {

    // filter a PathBuf vector for directories only and return as vector
    let mut found_directories = Vec::new();
    for p in path {
        if p.is_dir() {
            found_directories.push(p.clone());
        }
    }
    found_directories
}

fn enumerate_directory_depth(p: PathBuf, max_depth: u32) -> Vec<PathBuf> {
    
    // reads the path at "p", then based on "max_depth" will dive into the sub
    // directories. after enumerating all paths down to the depth, return
    // all paths found in a vector

    let mut found_paths = enumerate_directory(p);
    
    // 0 depth is just returning the contents of the directory given
    if 0 == max_depth {
        return found_paths;
    }

    // check for directories in contents and add to search_queue
    let mut search_queue = path_filter_directories(&found_paths);

    for current_depth in 0..max_depth {

        let mut new_paths = Vec::new();

        if current_depth == max_depth { 
            break; 
        }
        
        loop {
            // search queue till empty
            if let Some(current_dir) = search_queue.pop() {
                    new_paths.extend(
                        enumerate_directory(current_dir)
                    );
            } else {
                break;
            }
        }
        // save the paths we found after emptying a queue
        found_paths.extend(
            new_paths.clone()
        );

        // filter the new paths for directories to add back into
        // search queue, which is diving into the sub directory.
        search_queue.extend(
            path_filter_directories(&new_paths)
        );
    }

    found_paths
}
