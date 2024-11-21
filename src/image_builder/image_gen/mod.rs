pub mod scanner;
use scanner::*;
use std::collections::HashMap;

use crate::image_builder::{DevContainer, FeatureData};
use std::fs::File;
use std::io::Write;
pub fn gen_devcontainer() {
    println!("Generating devcontainer.json...");
   
    let features = scanner::scan("./App");
    let feature_urls = match features {
        Ok(_) => "".to_string(),
        Err(e) => e,
    };
    
    let featuredata = FeatureData {
        version: Some("1.0".to_string())
    };
    
    let devcontainer = DevContainer {
        name: "My Dev Container".to_string(),
        image: "ubuntu:latest".to_string(),
        features: {
            let mut map = HashMap::new();
            for url in feature_urls.split(',') {
                map.insert(url.to_string(), Some(featuredata.clone()));
            }
            map
        },
    };

    let devcontainer_json = serde_json::to_string_pretty(&devcontainer).unwrap();
    let mut file = File::create(".devcontainer/devcontainer2.json").unwrap();
    file.write_all(devcontainer_json.as_bytes()).unwrap();

    println!("devcontainer.json has been generated.");
}