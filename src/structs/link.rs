use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use crate::structs::node::Node;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Link {
    id : i64,
    #[serde(skip)]
    from : Arc<Mutex<Node>>,
    #[serde(skip)]
    to : Arc<Mutex<Node>>,
    from_id : i64,
    to_id : i64,
    owner: String,
}