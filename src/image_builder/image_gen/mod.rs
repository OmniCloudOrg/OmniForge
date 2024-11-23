pub mod scanner;
use std::collections::HashMap;

use crate::image_builder::{DevContainer, FeatureData};
use std::fs::File;
use std::io::Write;
pub fn gen_devcontainer(path: &str) {
    println!("Generating devcontainer.json...");

    let features = scanner::scan(path);

    let featuredata = FeatureData {
        version: Some("latest".to_string()),
    };

    let devcontainer = DevContainer {
        name: "My Dev Container".to_string(),
        image: "ubuntu:latest".to_string(),
        features: {
            let mut map = HashMap::new();
            for url in features.unwrap() {
                map.insert(url.clone(), Some(featuredata.clone()));
                println!("Feature URL: {}", url);
            }
            map
        },
    };

    let devcontainer_json = serde_json::to_string_pretty(&devcontainer).unwrap();
    let final_path = format!("{}/.devcontainer/devcontainer.json", path);
    std::fs::create_dir_all(format!("{}/.devcontainer", path)).unwrap();
    println!("Final path: {}", final_path);
    let mut file = File::create(final_path).unwrap();
    file.write_all(devcontainer_json.as_bytes()).unwrap();

    println!("devcontainer.json has been generated.");
}
