extern crate reqwest;

use std::fs;
use std::fs::metadata;
use std::path::PathBuf;

use rand::Rng;

pub fn download_data(request_url: &String) -> Vec<u8> {
    println!("downloading: {}", request_url);
    return reqwest::blocking::get(request_url).unwrap()
        .bytes().unwrap()
        .to_vec();
}

pub fn read_file(file_path: &String) -> Vec<u8> {
    return if metadata(file_path).unwrap().is_file() {
        fs::read(file_path).expect("Unable to read file")
    } else {
        read_random_file_from_directory(file_path)
    };
}

fn read_random_file_from_directory(directory_path: &String) -> Vec<u8> {
    let paths = fs::read_dir(directory_path).unwrap();

    let mut images = vec![];

    for path in paths {
        let dir_entry = path.unwrap();
        if dir_entry.metadata().unwrap().is_file() && is_picture(dir_entry.path()) {
            images.push(dir_entry.path().to_str().unwrap().to_string())
        }
    }

    let random_index = rand::thread_rng().gen_range(0..images.len());
    return read_file(images.get(random_index).unwrap());
}

fn is_picture(file_path: PathBuf) -> bool {
    let file_extension = file_path.extension().unwrap().to_str().unwrap().to_lowercase();
    return file_extension == "png"
        || file_extension == "jpg"
        || file_extension == "bmp"
        || file_extension == "jpeg";
}