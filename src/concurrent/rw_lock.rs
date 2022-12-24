#![allow(dead_code)]

use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::{borrow::Borrow, collections::HashMap, hash::Hash, sync::Arc};

pub static SYNC_MAP: Lazy<SyncMap<u32, String>> = Lazy::new(|| {
    let map = SyncMap::new();
    map.insert(1, "one".to_owned());
    map.insert(2, "two".to_owned());
    map.insert(3, "three".to_owned());
    map
});

pub static LAZY_SYNC_MAP: Lazy<RwLock<HashMap<u32, String>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

// sync struct
// Auto impl `Send` and `Sync` traits.
pub struct SyncMap<K, V> {
    inner: RwLock<HashMap<K, Arc<V>>>,
}

impl<K, V> SyncMap<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> SyncMap<K, V> {
        SyncMap {
            inner: RwLock::new(HashMap::new()),
        }
    }

    pub fn insert(&self, k: K, v: V) -> Option<Arc<V>> {
        let mut map = self.inner.write();
        map.insert(k, Arc::new(v))
    }

    pub fn map_value<F, U, Q: ?Sized>(&self, k: &Q, f: F) -> Option<U>
    where
        K: Borrow<Q>,
        Q: Eq + Hash,
        F: Fn(Option<&Arc<V>>) -> Option<U>,
    {
        let map = self.inner.read();
        f(map.get(k))
    }

    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<Arc<V>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash,
    {
        let map = self.inner.read();
        map.get(k).map(|v| v.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::{ops::Deref, sync::Arc, thread, time::Duration};

    use crate::concurrent::rw_lock::SYNC_MAP;
    use parking_lot::RwLock;

    use super::{SyncMap, LAZY_SYNC_MAP};

    #[test]
    fn test_static_sync_map() {
        let _ = thread::spawn(move || {
            let value = SYNC_MAP.get(&1);
            println!("{:?}", value)
        })
        .join();
    }

    #[test]
    fn test_lazy_sync_map() {
        let _ = thread::spawn(move || {
            let mut map = LAZY_SYNC_MAP.write();
            map.insert(1, "one".to_owned());
            map.insert(2, "two".to_owned());
            map.insert(3, "three".to_owned());
            map.insert(4, "four".to_owned());
        })
        .join();

        let map = LAZY_SYNC_MAP.read();
        println!("{:?}", map.deref())
    }

    #[test]
    fn test_new_sync_map() {
        let sync_map = Arc::new(SyncMap::new());
        let cloned1 = sync_map.clone();
        thread::spawn(move || {
            cloned1.insert(1, "one".to_owned());
        });

        let cloned2 = sync_map.clone();
        thread::spawn(move || {
            cloned2.insert(2, "two".to_owned());
        });

        let cloned3 = sync_map.clone();
        thread::spawn(move || {
            cloned3.insert(3, "three".to_owned());
        });

        thread::sleep(Duration::from_secs(2));

        let cloned4 = sync_map.clone();
        let _ = thread::spawn(move || {
            println!("{:?}", cloned4.get(&1));
            println!("{:?}", cloned4.get(&3));
        })
        .join();

        let clone5 = sync_map.clone();
        let _ = thread::spawn(move || {
            let result = clone5.map_value(&1, |opt| opt.map(|rc| rc.as_ref().clone()));
            println!("{:?}", result);
        })
        .join();
    }

    #[test]
    fn test_arc() {
        let number = Arc::new(RwLock::new(1));

        let cloned = number.clone();
        let _ = thread::spawn(move || {
            let mut num = cloned.write();
            *num += 1
        })
        .join();

        println!("{:?}", number);
    }

    #[test]
    fn test_deref() {
        #[derive(Clone, Copy)]
        struct Te;
        #[derive(Clone, Copy)]
        struct Se;

        impl Se {
            fn test_self(self) {
                println!("Se");
            }

            fn test_ref(&self) {
                println!("&Se");
            }
        }

        impl Deref for Te {
            type Target = Se;

            fn deref(&self) -> &Self::Target {
                &Se
            }
        }

        Te.test_ref();
        Te.test_self();
    }
}
