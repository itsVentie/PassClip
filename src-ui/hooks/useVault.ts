import { useState, useEffect } from 'preact/hooks';
import { invoke } from '@tauri-apps/api/core';

export interface UiSlot {
  id: number;
  len: number;
  entropy: number;
  timestamp: number;
}

export function useVault() {
  const [slots, setSlots] = useState<UiSlot[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const fetchSlots = async () => {
    try {
      setLoading(true);
      const data = await invoke<UiSlot[]>('get_vault_slots');
      setSlots(data);
      setError(null);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchSlots();
    const timer = setInterval(fetchSlots, 2000);
    return () => clearInterval(timer);
  }, []);

  return { slots, error, loading, refresh: fetchSlots };
}