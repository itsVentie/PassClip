import { useVault } from "../hooks/useVault";
import { SlotCard } from "../components/SlotCard";
import styles from "../styles/Vault.module.css";

export function VaultPage() {
  const { slots, status, error, loading, refresh, handlePopSlot } = useVault();

  return (
    <div className={styles.vaultPage}>
      <header className={styles.vaultHeader}>
        <h1>PassClip Vault</h1>
        <button className={styles.refreshBtn} onClick={refresh} disabled={loading}>
          {loading ? "Refreshing..." : "Refresh"}
        </button>
      </header>

      {status && (
        <div className={styles.statusBar}>
          <div className={styles.statusItem}>
            <span className={styles.statusLabel}>Slots:</span>
            <span className={styles.statusValue}>{status.count} / {status.max_slots}</span>
          </div>
          <div className={styles.statusItem}>
            <span className={styles.statusLabel}>State:</span>
            <span className={`${styles.statusBadge} ${status.has_secret ? styles.unlocked : styles.locked}`}>
              {status.has_secret ? "Unlocked" : "Locked"}
            </span>
          </div>
        </div>
      )}

      {error && <div className={styles.errorBanner}>{error}</div>}

      <div className={styles.slotsList}>
        {slots.length === 0 ? (
          <div className={styles.emptyState}>No encrypted slots in buffer</div>
        ) : (
          slots.map((slot) => (
            <SlotCard key={slot.id} slot={slot} onPop={handlePopSlot} />
          ))
        )}
      </div>
    </div>
  );
}