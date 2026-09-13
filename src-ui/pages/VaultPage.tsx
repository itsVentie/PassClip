import { useVault } from "../hooks/useVault";
import { SlotCard } from "../components/SlotCard";

export function VaultPage() {
  const { slots, status, error, loading, refresh, handlePopSlot } = useVault();

  return (
    <div className="vault-page">
      <header className="vault-header">
        <h1>PassClip Vault</h1>
        <button className="refresh-btn" onClick={refresh} disabled={loading}>
          {loading ? "Refreshing..." : "Refresh"}
        </button>
      </header>

      {status && (
        <div className="status-bar">
          <div className="status-item">
            <span className="status-label">Slots:</span>
            <span className="status-value">{status.count} / {status.max_slots}</span>
          </div>
          <div className="status-item">
            <span className="status-label">State:</span>
            <span className={`status-badge ${status.has_secret ? "unlocked" : "locked"}`}>
              {status.has_secret ? "Unlocked" : "Locked"}
            </span>
          </div>
        </div>
      )}

      {error && <div className="error-banner">{error}</div>}

      <div className="slots-list">
        {slots.length === 0 ? (
          <div className="empty-state">No encrypted slots in buffer</div>
        ) : (
          slots.map((slot) => (
            <SlotCard key={slot.id} slot={slot} onPop={handlePopSlot} />
          ))
        )}
      </div>
    </div>
  );
}