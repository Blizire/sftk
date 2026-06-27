mod util;

use std::path::PathBuf;

fn main() {

    println!("steamplay - 0.0.1");
    let mut games = Vec::new();
    for path in find_steamapps() {
        for _games in find_games(path) {
            games.push(_games);
        }
    }

    for g in games {
        println!("{:?}", g);
    }
}

fn find_steamapps() -> Vec<PathBuf> {

    // Find all steamapps folder on the system
    let mut found_paths = Vec::new();
    let mut steamapp_paths = Vec::new();

    // TODO cache the steamapp library locations
    // TODO make depth configurable at runtime through cli params or config file

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

fn find_games(steamapps_path: PathBuf) -> Vec<PathBuf> {

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
