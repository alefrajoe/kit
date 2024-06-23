pub fn count_files(path: &String, pattern: Option<String>) {
    
    // List all files in the directory
    let files: Vec<_> = std::fs::read_dir(path).unwrap().collect();

    // Count the files
    let count = files.len();

    // If pattern is None, print the total number of files
    if pattern.is_none() {

        // Print the result
        println!("There are {count} files in {path}");

    // If pattern is Some, match the files to the pattern
    } else {

        // Get the pattern
        let pattern = pattern.unwrap();

        // Match the files to the pattern
        let matched_files = files.iter().filter(|file| file.as_ref().unwrap().file_name().to_str().unwrap().contains(&pattern));
        
        // Count the matched files
        let count = matched_files.count();

        // Print the result
        println!("There are {count} files in {path} that match {pattern}");
    }
}