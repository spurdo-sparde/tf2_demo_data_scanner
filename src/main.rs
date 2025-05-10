use serde_json::Value;
use std::fs;

fn main() {
    if std::env::args().len() != 2 {
        println!("Usage: tf2_dem_parse <path>");
        return;
    }

    let path = std::env::args().nth(1).expect("Missing input file path");
    let dir = fs::read_dir(path).expect("Failed to open directory.");
    for file in dir
        .filter_map(Result::ok)
        .filter(|f| f.path().extension().and_then(|ext| ext.to_str()) == Some("json"))
    {
        let input = fs::read_to_string(file.path()).expect("Failed to read file.");
        let json = serde_json::from_str(&input).expect("Failed to parse json.");
        get_chat_log(&json);
    }
}

pub fn get_chat_log(json: &Value) {
    if let Value::Object(map) = json {
        let chat = map
            .get("chat")
            .expect("Could not locate chat in demo file.")
            .as_array()
            .unwrap();

        for message in chat.iter().filter(|x| !sent_by_server(x)) {
            if let Some(text) = message.get("text") {
                println!("{}", text.as_str().unwrap())
            }
        }
    }
}

pub fn sent_by_server(value: &Value) -> bool {
    value.get("from").unwrap() == ""
}
