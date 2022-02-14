pub fn verify_yaml_file(file_name: &str) {
    crate::utils::file::verify_file_exists(file_name);
    assert!(file_is_yaml(file_name));
}

fn file_is_yaml(file_name: &str) -> bool {
    return file_name.ends_with(".yaml");
}
