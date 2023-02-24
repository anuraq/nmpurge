use fs_extra::dir::get_size;
use human_bytes::human_bytes;
use std::{fs, ops::Add, path::Path, path::PathBuf};

fn main() {
    let search_path = Path::new("/home/aaq");
    let mut total_size: u64 = 0;
    search_folder(search_path.to_path_buf(), &mut total_size);
    println!("TOTAL SIZE {}", total_size);
}

fn search_folder(folder: PathBuf, total_size: &mut u64) {
    for entry in fs::read_dir(folder)
        .expect(&format!("read {}", human_bytes(*total_size as f64)))
        .map(|x| x.expect("cant map"))
        .filter(|x| x.file_type().expect("cant file type").is_dir())
    {
        if entry.file_name().eq("node_modules") {
            let path_size = get_size(entry.path()).unwrap();
            *total_size += path_size;
            let human_size = human_bytes(path_size as f64);
            println!("{} {}", human_size, entry.path().display());
        } else {
            search_folder(entry.path(), total_size);
        }
    }
}
