pub mod side_panel;
pub mod dummy;
pub mod top_panel;
pub mod generic_node_container;

use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use speedy2d::Graphics2D;
use crate::modules::dummy::DummyModule;
use crate::modules::generic_node_container::GenericNodeContainer;
use crate::structs::module::Module;