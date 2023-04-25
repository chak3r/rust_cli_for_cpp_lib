use crate::types;

pub fn get_a_collection_from_file(file_name: &std::path::PathBuf) -> types::ACollection {
    let json_str = std::fs::read_to_string(file_name).expect("Unable to read file");

    let data: types::ACollection =
        serde_json::from_str(json_str.as_str()).expect("JSON was not well-formatted");

    return data;
}

pub fn get_b_collection_from_file(file_name: &std::path::PathBuf) -> types::BCollection {
    let json_str = std::fs::read_to_string(file_name).expect("Unable to read file");

    let data: types::BCollection =
        serde_json::from_str(json_str.as_str()).expect("JSON was not well-formatted");

    return data;
}
