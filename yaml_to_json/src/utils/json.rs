pub fn get_pretty_json(json: &serde_json::Value) -> std::string::String {
    return serde_json::to_string_pretty(json).unwrap();
}

pub fn get_json_from_yaml_file(file_name: &str) -> serde_json::Value {
    crate::utils::yaml::verify_yaml_file(file_name);
    let content = crate::utils::file::get_file_content(file_name);
    return serde_yaml::from_str(&content).expect("Invalid yaml file");
}

pub fn create_json_file_from_yaml_file(file_name: &str) {
    let json = get_json_from_yaml_file(&file_name);
    let pretty_json = get_pretty_json(&json);
    let json_file = file_name.replace(".yaml", ".json");
    crate::utils::file::write_to_file(&json_file, &pretty_json);
}
