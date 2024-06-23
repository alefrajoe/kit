use std::fs;
use std::path::Path;
use image::GenericImageView;

/// Count the images in a directory or file and print their size
/// 
/// # Arguments
/// 
/// * `path` - The path to the image or directory
pub fn count_images_with_size(path: &str) {
    
    // Create a path object
    let path = Path::new(path);
    // Check if the path is a file
    if path.is_file() {
        // Open the image
        if let Ok(img) = image::open(path) {
            // Get the image dimensions
            let (img_width, img_height) = img.dimensions();
            // Check if the image has the correct size
            println!("Image {} has size {}x{}", path.to_string_lossy(), img_height, img_width);
        } else {
            // Print an error message if the file is not a valid image
            println!("The file is not a valid image.");
        }
    } else if path.is_dir() {

        // Iterate over the files in the directory
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            // Check if the file is a valid image
            if path.is_file() {
                if let Ok(img) = image::open(&path) {
                    let (img_width, img_height) = img.dimensions();
                    // Print the image size
                    println!("Image {} has size {}x{}", path.to_string_lossy(), img_height, img_width);
                }
            }
        }
    } else {
        println!("The path is neither a file nor a directory.");
    }
} 