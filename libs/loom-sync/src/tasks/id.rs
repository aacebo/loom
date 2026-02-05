use std::sync::atomic::{AtomicU64, Ordering};

static COUNTER: AtomicU64 = AtomicU64::new(0);

///
/// ## TaskId
/// an auto incrementing atomic
/// task identifier
///
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct TaskId(u64);

impl TaskId {
    pub fn new() -> Self {
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    pub fn as_u64(&self) -> &u64 {
        &self.0
    }

    pub fn to_u64(&self) -> u64 {
        self.0
    }
}

impl std::fmt::Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl Default for TaskId {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::thread;

    #[test]
    fn unique() {
        let id1 = TaskId::new();
        let id2 = TaskId::new();
        let id3 = TaskId::new();

        assert_ne!(id1, id2);
        assert_ne!(id2, id3);
        assert_ne!(id1, id3);
    }

    #[test]
    fn monotonic() {
        let id1 = TaskId::new();
        let id2 = TaskId::new();
        let id3 = TaskId::new();

        assert!(id1.to_u64() < id2.to_u64());
        assert!(id2.to_u64() < id3.to_u64());
    }

    #[test]
    fn as_u64() {
        let id = TaskId::new();
        let _val: &u64 = id.as_u64();
    }

    #[test]
    fn to_u64() {
        let id = TaskId::new();
        let val: u64 = id.to_u64();
        assert_eq!(val, *id.as_u64());
    }

    #[test]
    fn display() {
        let id = TaskId::new();
        let display = format!("{}", id);
        let parsed: u64 = display.parse().unwrap();
        assert_eq!(parsed, id.to_u64());
    }

    #[test]
    fn debug() {
        let id = TaskId::new();
        let debug = format!("{:?}", id);
        assert!(debug.contains("TaskId"));
    }

    #[test]
    fn clone_test() {
        let id1 = TaskId::new();
        let id2 = id1.clone();
        assert_eq!(id1, id2);
    }

    #[test]
    fn copy_test() {
        let id1 = TaskId::new();
        let id2: TaskId = id1;
        assert_eq!(id1, id2);
    }

    #[test]
    fn hash() {
        let id1 = TaskId::new();
        let id2 = TaskId::new();

        let mut set = HashSet::new();
        set.insert(id1);
        set.insert(id2);

        assert_eq!(set.len(), 2);
        assert!(set.contains(&id1));
        assert!(set.contains(&id2));
    }

    #[test]
    fn equality() {
        let id1 = TaskId::new();
        let id2 = id1;
        let id3 = TaskId::new();

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn unique_across_threads() {
        let num_threads = 10;
        let ids_per_thread = 100;

        let handles: Vec<_> = (0..num_threads)
            .map(|_| {
                thread::spawn(move || {
                    (0..ids_per_thread)
                        .map(|_| TaskId::new().to_u64())
                        .collect::<Vec<_>>()
                })
            })
            .collect();

        let mut all_ids = HashSet::new();
        for handle in handles {
            let ids = handle.join().unwrap();
            for id in ids {
                all_ids.insert(id);
            }
        }

        // All IDs should be unique
        assert_eq!(all_ids.len(), num_threads * ids_per_thread);
    }

    #[test]
    fn stress_test() {
        let mut ids = Vec::new();
        for _ in 0..10000 {
            ids.push(TaskId::new().to_u64());
        }

        let unique: HashSet<_> = ids.iter().collect();
        assert_eq!(unique.len(), ids.len());
    }

    #[test]
    fn default_test() {
        let id = TaskId::default();
        // Just verify it creates a valid ID
        let _ = id.to_u64();
    }
}
