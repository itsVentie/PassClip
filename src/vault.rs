use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Clone, Zeroize, ZeroizeOnDrop)]
pub struct VaultSlot {
    pub id: u32,
    pub secret: String,
    pub entropy: f32,
    #[zeroize(skip)]
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlotMetadata {
    pub id: u32,
    pub entropy: f32,
    pub timestamp: SystemTime,
    pub len: usize,
}

pub struct MultiSlotVault {
    slots: Vec<VaultSlot>,
    max_slots: usize,
    next_id: u32,
}

impl MultiSlotVault {
    pub fn new(max_slots: usize) -> Self {
        Self {
            slots: Vec::with_capacity(max_slots),
            max_slots,
            next_id: 1,
        }
    }

    pub fn push(&mut self, secret: String, entropy: f32) -> u32 {
        if self.slots.len() >= self.max_slots && !self.slots.is_empty() {
            self.slots.remove(0);
        }

        let id = self.next_id;
        self.next_id = self.next_id.wrapping_add(1);

        self.slots.push(VaultSlot {
            id,
            secret,
            entropy,
            timestamp: SystemTime::now(),
        });

        id
    }

    pub fn pop_by_id(&mut self, id: u32) -> Option<VaultSlot> {
        if let Some(index) = self.slots.iter().position(|s| s.id == id) {
            Some(self.slots.remove(index))
        } else {
            None
        }
    }

    pub fn pop_latest(&mut self) -> Option<VaultSlot> {
        self.slots.pop()
    }

    pub fn list_metadata(&self) -> Vec<SlotMetadata> {
        self.slots
            .iter()
            .map(|s| SlotMetadata {
                id: s.id,
                entropy: s.entropy,
                timestamp: s.timestamp,
                len: s.secret.len(),
            })
            .collect()
    }

    pub fn cleanup_expired(&mut self, max_age: Duration) {
        let now = SystemTime::now();
        self.slots.retain(|slot| {
            now.duration_since(slot.timestamp)
                .map(|elapsed| elapsed < max_age)
                .unwrap_or(false)
        });
    }

    pub fn clear(&mut self) {
        self.slots.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    pub fn len(&self) -> usize {
        self.slots.len()
    }

    pub fn max_slots(&self) -> usize {
        self.max_slots
    }
}