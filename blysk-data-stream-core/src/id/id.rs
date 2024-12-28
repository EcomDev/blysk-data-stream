use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

pub struct Id {
    id: AtomicU64,
    has_value: AtomicBool,
    resolvable: AtomicBool,
}

impl Default for Id {
    fn default() -> Self {
        Self {
            id: AtomicU64::new(0),
            has_value: AtomicBool::new(false),
            resolvable: AtomicBool::new(true),
        }
    }
}

impl Id {
    pub(super) fn resolved(id: u64) -> Self {
        Self {
            id: AtomicU64::new(id),
            has_value: AtomicBool::new(true),
            resolvable: AtomicBool::new(false),
        }
    }

    pub(super) fn load(&self) -> Option<u64> {
        if self.has_value.load(Ordering::Relaxed) {
            return Some(self.id.load(Ordering::Relaxed))
        }

        None
    }

    pub(super) fn store(&self, value: Option<u64>) {
        self.resolvable.store(false, Ordering::Relaxed);
        self.has_value.store(value.is_some(), Ordering::Relaxed);
        if let Some(value) = value {
            self.id.store(value, Ordering::Relaxed)
        }
    }

    pub(super) fn is_resolvable(&self) -> bool {
        self.resolvable.load(Ordering::Relaxed)
    }
}