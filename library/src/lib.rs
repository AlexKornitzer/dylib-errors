use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use tokio::runtime::Runtime;

pub struct Test {
    test: Arc<Mutex<HashMap<String, String>>>,
    runtime: Runtime,
}

impl Test {
    pub fn new() -> Self {
        let _ = async {
            reqwest::get("").await.unwrap();
        };
        Self {
            test: Arc::new(Mutex::new(HashMap::new())),
            runtime: Runtime::new().unwrap(),
        }
    }
}
