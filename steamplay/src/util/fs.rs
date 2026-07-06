use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::io::{Read, Write};
use std::ffi::os_str::OsStr;
use std::process::{Command, Child};
use std::os::windows::process::CommandExt;

pub fn enumerate_drives() -> Vec<PathBuf> {

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

pub fn enumerate_directory(p: PathBuf) -> Vec<PathBuf> {

    // reads a directory and returns the path of the content as a PathBuf vector
    let mut directory_contents = Vec::new();
    if let Ok(reader) = fs::read_dir(p) {
        for entry in reader.flatten() {
            directory_contents.push(entry.path());
        }
    }

    directory_contents
}

pub fn path_filter_directories<'a>(path: &'a Vec<PathBuf>) -> Vec<PathBuf> {

    // filter a PathBuf vector for directories only and return as vector
    let mut found_directories = Vec::new();
    for p in path {
        if p.is_dir() {
            found_directories.push(p.clone());
        }
    }
    found_directories
}

pub fn enumerate_directory_depth(p: PathBuf, max_depth: u32) -> Vec<PathBuf> {
    
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

pub fn cache_path_save(paths: &[PathBuf], path: &str) -> Result<(), std::io::Error> {

    // save the paths found into a file
    let mut file = File::create(path)?;
    for p in paths {
        writeln!(file, "{}", p.display())?;
    }
    Ok(())
}

fn cache_path_load(path: &str) -> Result<Vec<PathBuf>, std::io::Error> {

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents.lines().map(|l| PathBuf::from(l.trim())).collect::<Vec<_>>())
}

pub fn select_game(game_name: &str) -> Option<PathBuf> {

    let cache_path = "./steampapps.cache";

    // check if cache exists, if so lets load it to save time
    let steamapps_path_vec = if let Ok(loaded_paths) = cache_path_load(cache_path) {
        println!("Loaded {} paths from cache", loaded_paths.len());
        loaded_paths
    } else {
        // cache not found so we are going to manually search and create cache
        println!("Cache not found, performing manual search...");
        let paths = find_steamapps();
        cache_path_save(&paths, cache_path).unwrap();
        paths
    };


    // walk the steamapps paths and search for executables
    let mut games = Vec::new();
    for path in steamapps_path_vec {
        for _games in find_games(path) {
            games.push(_games);
        }
    }

    // provide a game_name string that matches exactly with the executable
    for game in games {
        if let Some(fname) = game.file_name() {
            if fname == OsStr::new(game_name) {
                return Some(game);
            }
        }
    }
    None
} 

pub fn start_game(path: PathBuf) -> Child {

    // process creation flags
    const DETACHED_PROCESS: u32 = 0x00000008;
    const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;

    let child = Command::new(&path)
        .current_dir(path.parent().unwrap())
        .creation_flags(DETACHED_PROCESS | CREATE_NEW_PROCESS_GROUP)
        .spawn()
        .expect("failed to start process");

    child
}

pub fn find_steamapps() -> Vec<PathBuf> {

    // Find all steamapps folder on the system
    let mut found_paths = Vec::new();
    let mut steamapp_paths = Vec::new();

    // TODO make depth configurable at runtime through cli params or config file

    // search through all the windows drivers to enumerate paths on fs
    for root in util::fs::enumerate_drives() {
        let path = util::fs::enumerate_directory_depth(root, 2);
        found_paths.extend(path);
    }

    for p in found_paths {
        if let Some(fname) = p.file_name() {
            if fname == "steamapps" {
                steamapp_paths.push(p)
            }
        }
    }
    
    steamapp_paths
}

pub fn find_games(steamapps_path: PathBuf) -> Vec<PathBuf> {

    // returns the paths to games found in steamapps.
    //
    // there is no standard packaging method, different games, different engines
    // have different patterns on how they pack it. so we walk the directories,
    // search for any executables and for now filter the trash exes from a blacklist

    let mut path_executables = Vec::new();
    let path_collection = util::fs::enumerate_directory_depth(steamapps_path, 5);

    // filter for files that are executables based on file extension
    for p in path_collection {
        if let Some(ext) = p.extension() {
            if ext == "exe" {
                path_executables.push(p);
            }
        }
    }
    
    path_executables
}
