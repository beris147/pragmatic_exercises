fn printFileContent(fileToRead: &str) {
    let fileContent = fs::read_to_string(fileToRead)
        .expect("Something went wrong reading the file");
    println!("With text:\n{}", fileContent);
}