use fs_extra::dir::get_size;
use human_bytes::human_bytes;
use std::{
    fs::{self, ReadDir},
    io::ErrorKind,
    path::Path,
};

fn main() {
    let search_path = Path::new("/home/aaq");
    let mut total_size: u64 = 0;
    let search_path = fs::read_dir("/home/aaq");
    // search_folder(search_path.to_path_buf(), &mut total_size);
    search_folder(search_path.unwrap(), &mut total_size);
    println!("{} TOTAL SIZE", human_bytes(total_size as f64));
}

// fn skip_in_permission_error(path: &PathBuf) ->  {
//     return;
// }

fn search_folder(folder: ReadDir, total_size: &mut u64) {
    for entry in folder
        .map(|x| x.expect("cant map"))
        .filter(|x| x.file_type().expect("cant file type").is_dir())
    {
        if entry.file_name().eq("node_modules") {
            let path_size = get_size(entry.path()).unwrap();
            *total_size += path_size;
            let human_size = human_bytes(path_size as f64);
            println!("{} {}", human_size, entry.path().display());
        } else {
            let match_entry = fs::read_dir(entry.path());
            match match_entry {
                Ok(folder) => search_folder(folder, total_size),
                Err(error) => match error.kind() {
                    ErrorKind::PermissionDenied => {
                        eprintln!("{} {}", error, entry.path().display())
                    }
                    other_error => eprintln!("{} {}", error, entry.path().display()),
                },
            }
        };
        // search_folder(entry.path(), total_size);
    }
}
