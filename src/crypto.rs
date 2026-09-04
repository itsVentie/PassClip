use crate::vault::MultiSlotVault;
use std::collections::HashMap;
use webauthn_rs::prelude::PasskeyAuthentication;

pub struct SecureVault {
    pub multi_vault: MultiSlotVault,
    pub target_slot_id: Option<u32>,
    pub current_auth: Option<PasskeyAuthentication>,
}

impl SecureVault {
    pub fn new() -> Self {
        Self {
            multi_vault: MultiSlotVault::new(5),
            target_slot_id: None,
            current_auth: None,
        }
    }

    pub fn has_secret(&self) -> bool {
        !self.multi_vault.is_empty()
    }
}

pub fn calculate_entropy(s: &str) -> f64 {
    if s.is_empty() {
        return 0.0;
    }
    let mut frequencies = HashMap::new();
    for c in s.chars() {
        *frequencies.entry(c).or_insert(0) += 1;
    }
    let len = s.len() as f64;
    frequencies
        .values()
        .map(|&count| {
            let p = count as f64 / len;
            -p * p.log2()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_calculation() {
        let low_entropy = calculate_entropy("aaaaaaa");
        let high_entropy = calculate_entropy("g8#K!v9$XzP2");

        assert!(
            high_entropy > low_entropy,
            "Random string must have higher entropy than repeated char"
        );
        assert_eq!(low_entropy, 0.0);
    }
}