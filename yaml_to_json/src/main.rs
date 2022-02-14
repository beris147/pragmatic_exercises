use std::env;
mod utils;

fn main() {
    for arg in env::args().skip(1) {
        let file_name = arg;
        utils::json::create_json_file_from_yaml_file(&file_name);
    }
}
