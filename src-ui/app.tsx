import { useEffect, useState } from "preact/hooks";
import { getVaultSlots, getVaultStatus, popSlot, UiSlot, UiStatus } from "./api";

export function App() {
  const [slots, setSlots] = useState<UiSlot[]>([]);
  const [status, setStatus] = useState<UiStatus | null>(null);
  const [error, setError] = useState<string | null>(null);

  const refreshData = async () => {
    try {
      setError(null);
      const [fetchedSlots, fetchedStatus] = await Promise.all([
        getVaultSlots(),
        getVaultStatus(),
      ]);
      setSlots(fetchedSlots);
      setStatus(fetchedStatus);
    } catch (err) {
      setError(String(err));
    }
  };

  useEffect(() => {
    refreshData();
  }, []);

  const handlePopSlot = async (id: number) => {
    try {
      const challengePayload = await popSlot(id);
      console.log("Challenge received:", challengePayload);
    } catch (err) {
      setError(`Failed to pop slot: ${err}`);
    }
  };

  return (
    <div className="container">
      <header>
        <h1>PassClip Vault</h1>
        <button onClick={refreshData}>Refresh</button>
      </header>

      {status && (
        <div className="status-bar">
          <p>Slots used: {status.count} / {status.max_slots}</p>
          <p>Vault status: {status.has_secret ? "Unlocked" : "Locked"}</p>
        </div>
      )}

      {error && <div className="error-banner">{error}</div>}

      <div className="slots-list">
        {slots.map((slot) => (
          <div key={slot.id} className="slot-card">
            <span>Slot #{slot.id}</span>
            <span>Length: {slot.len}</span>
            <span>Entropy: {slot.entropy.toFixed(2)}</span>
            <button onClick={() => handlePopSlot(slot.id)}>Pop</button>
          </div>
        ))}
      </div>
    </div>
  );
}