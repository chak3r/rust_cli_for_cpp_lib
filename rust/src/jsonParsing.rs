use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Data {
    data: Vec<f64>,
}

fn parseDataFromFile(file_name: &std::path::PathBuf) -> Data {
    let json_str = std::fs::read_to_string(file_name).expect("Unable to read file");

    let data: Data = serde_json::from_str(json_str.as_str()).expect("JSON was not well-formatted");

    return data;
}
