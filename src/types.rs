use std::sync::Arc;
use std::sync::RwLock;

pub type DoublePointerSafe<T> = Arc<RwLock<Arc<RwLock<T>>>>;
