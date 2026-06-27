mod util;

fn main() {

    println!("steamplay - 0.0.1");
    
    for path in find_steamapps() {
        println!("{:?}", path);
    }
    
}

fn find_steamapps() -> Vec<std::path::PathBuf> {

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
