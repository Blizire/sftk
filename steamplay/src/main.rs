use std::fs;

fn main() {
    println!("steamplay - 0.0.1");
    enumerate_steam_library();
}

fn enumerate_steam_library() {
    
    // enumerate drives on a windows system
    let valid_drives = enumerate_drives();

    // start searching through each drive
    let mut search_results = Vec::new();
    for drive in valid_drives {
        let paths = steam_library_search(&drive);
        match paths {
            Some(p) => {
                search_results.push(p)
            },
            None => continue
        }
    }
}

fn steam_library_search(root: &String) -> Option<Vec<String>>  {

    // search into sub-directories looking for the steamapps folder.
    
    let mut found_paths = Vec::new();

    let dir_entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        Err(_) => return None
    };

    for entry in dir_entries {
        let entry = match entry {
            Ok(s) => s,
            Err(_) => return None
        };

        let entrystr = match entry.file_name().into_string() {
            Ok(s) => s,
            Err(_) => return None
        };

        if entrystr == "steamapps" {
            found_paths.push(String::from(&entrystr));
        }
    }

    Some(found_paths)
}

fn enumerate_drives() -> Vec<String> {

    // generate a vector of strings that represents typical windows disk drives
    // c:/ d:/ e:/ etc...
    
    let logical_drives: Vec<String> = ('a'..='z')
        .map(|c| format!("{}:/", c))
        .collect();
    
    let mut valid_drives = Vec::new();

    for drive in logical_drives {
        let read_status = fs::read_dir(&drive);
        match read_status {
            Ok(_) => {
                valid_drives.push(drive);
            }
            Err(_) => {
                continue
            }
        }
    }

    valid_drives
}
