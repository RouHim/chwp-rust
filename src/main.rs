extern crate image;
extern crate rand;

use std::env;

mod config;
mod display;
mod pixabay;
mod image_edit;
mod kde;
mod cli;
mod display_manager;
mod gnome;
mod xfce;
mod file_receiver;


fn main() {
    // get args with app path
    let args: Vec<String> = env::args().skip(1).collect();

    // parse the args to get a configuration
    let config = config::parse_cli_args(args);

    // read the display info
    let display_info = display::get_info();

    // retrieve the image data from pixabay
    let mut image_data = if config::is_url(&config.keyword) {
        file_receiver::download_data(&config.keyword)
    } else if config::is_local_path(&config.keyword) {
        file_receiver::read_file(&config.keyword)
    } else {
        pixabay::get_image_data(&config, &display_info)
    };

    // scale the image to fit the display
    image_data = image_edit::scale_image(&image_data, config.span.clone(), &display_info);

    // change the wallpaper to the scaled image
    display_manager::change_wallpaper(&image_data, &config);
}
