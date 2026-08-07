use fd_lock::RwLock;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;

pub struct SingleInstanceGuard {
    _file: File,
}

impl SingleInstanceGuard {
    pub fn acquire() -> Result<Self, String> {
        let lock_path = Self::get_lock_path()?;

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&lock_path)
            .map_err(|e| format!("Failed to open lockfile ({:?}): {}", lock_path, e))?;

        let mut lock = RwLock::new(file);

        if lock.try_write().is_ok() {
            let file = lock.into_inner();
            Ok(Self { _file: file })
        } else {
            Err("Another instance of PassClip daemon is already running.".to_string())
        }
    }

    fn get_lock_path() -> Result<PathBuf, String> {
        let mut path = std::env::temp_dir();
        path.push("passclip_daemon.lock");
        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_instance_lock() {
        let first_guard = SingleInstanceGuard::acquire();
        assert!(first_guard.is_ok(), "First instance should acquire lock");

        let second_guard = SingleInstanceGuard::acquire();
        assert!(
            second_guard.is_err(),
            "Second instance must fail to acquire lock"
        );

        drop(first_guard);

        let third_guard = SingleInstanceGuard::acquire();
        assert!(
            third_guard.is_ok(),
            "Lock should be re-acquireable after first guard is dropped"
        );
    }
}