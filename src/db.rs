use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use bytes::Bytes;

#[derive(Debug)]
pub(crate) struct DbDropGuard {
    db: Db,
}

#[derive(Debug, Clone)]
pub(crate) struct Db {
    shared: Arc<Shared>,
}

#[derive(Debug)]
struct Shared {
    state: Mutex<State>,
}

#[derive(Debug)]
struct State {
    entries: HashMap<String, Entry>,
}

#[derive(Debug)]
struct Entry {
    data: Bytes,
}

impl DbDropGuard {
    pub(crate) fn new() -> DbDropGuard {
        DbDropGuard { db: Db::new() }
    }

    /// Get the shared database, Internally, this is an Arc so a clone only
    /// increase the refcount
    pub(crate) fn db(&self) -> Db {
        self.db.clone()
    }
}

impl Db {
    /// Create a new empty Db instance
    pub(crate) fn new() -> Db {
        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                entries: HashMap::new(),
            }),
        });

        Db { shared }
    }

    /// Get the value associated with a key
    pub(crate) fn get(&self, key: &str) -> Option<Bytes> {
        let state = self.shared.state.lock().unwrap();

        // get the clone of data, because the data is `Bytes`,
        // `Bytes`` itself is fat pointer, so a clone is a shallow clone
        state.entries.get(key).map(|entry| entry.data.clone())
    }

    pub(crate) fn set(&self, key: String, entry: Bytes) {
        let mut state = self.shared.state.lock().unwrap();

        // why the key need deep clone?
        let _ = state.entries.insert(key.clone(), Entry { data: entry });
    }
}
