use serde::{Serialize, Deserialize};
use core::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, Mutex};

pub static NODE_COUNTER : AtomicI64 = AtomicI64::new(0);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    id : i64,
    content : String,
    owner: String,
    #[serde(skip)]
    links: Vec<Arc<Mutex<Node>>>,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            id: -1,
            content: "null".to_string(),
            owner: "null".to_string(),
            links: Vec::new(),
        }
    }
}

impl Node {

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub(crate) fn create_and_register(content: String, owner: String) -> Node {
        let id = NODE_COUNTER.fetch_add(1, Ordering::SeqCst);
        let node = Node {
            id,
            content,
            owner,
            links: Vec::new(),
        };
        node
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }

}
