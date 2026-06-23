use std::fs;

fn main() {
    println!("steamplay - 0.0.1");
    let drives = enumerate_drives();
    println!("valid drives : {:?}", drives);
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
