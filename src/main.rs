use std::{fs, path::Path, path::PathBuf};

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

fn main() {
    // let stack: Vec<PathBuf> = vec![];
    // let path: &'static str = "/home/aaq";
    // let dirs = fs::read_dir(path)
    //     .expect("dir not fount")
    //     .map(|x| x.unwrap().path())
    //     .filter(|x| x.is_dir())
    //     .collect::<Vec<PathBuf>>();
    // for i in dirs.iter() {
    //     println!("{:?}", i);
    // }
    // print_type_of(&dirs);
    // println!(
    //     "{:?}",
    //     fs::read_dir(&dirs[0])
    //         .expect("dir not fount")
    //         .map(|x| x.unwrap().path())
    //         .filter(|x| x.is_dir())
    //         .collect::<Vec<PathBuf>>()
    // );
    let search_path = Path::new("/home/aaq");
    search_folder(search_path.to_path_buf());
}

fn search_folder(folder: PathBuf) {
    for entry in fs::read_dir(folder)
        .expect("cant read")
        .map(|x| x.expect("cant map"))
        .filter(|x| x.file_type().expect("cant file type").is_dir())
    {
        if entry.file_name().eq("node_modules") {
            println!("node_modules found {}", entry.path().display());
        } else {
            search_folder(entry.path());
        }
    }
}
// for entry in fs::read_dir("/home/aaq/code/").unwrap() {
//     let entry = entry.unwrap();
//     if entry.path().is_dir() {
//         println!("{}", entry.path().display());
//     }
// }
