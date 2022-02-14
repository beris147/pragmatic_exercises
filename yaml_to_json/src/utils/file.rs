pub fn verify_file_exists(file_name: &str) {
    match file_exists(file_name) {
        Ok(_) => return,
        Err(error) => std::panic!(
            "Cannot verify file {file} Error: {error}",
            file = file_name,
            error = error
        ),
    }
}

pub fn file_exists(file_name: &str) -> std::io::Result<()> {
    std::fs::metadata(file_name)?;
    Ok(())
}

pub fn get_file_content(file_name: &str) -> std::string::String {
    verify_file_exists(file_name);
    match std::fs::read_to_string(file_name) {
        Ok(content) => return content,
        Err(error) => std::panic!(
            "Cannot read file {file_name} Error: {error}",
            file_name = file_name,
            error = error
        ),
    }
}

pub fn write_to_file(file_name: &str, content: &str) {
    match std::fs::write(&file_name, &content) {
        Ok(_) => println!("Output generated at {}", file_name),
        Err(error) => std::panic!(
            "Error while writing {file_name}, Error {error}",
            file_name = file_name,
            error = error
        ),
    }
}
