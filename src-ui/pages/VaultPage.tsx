import { useState } from "preact/hooks";
import { useVault } from "../hooks/useVault";
import { SlotCard } from "../components/SlotCard";
import { LogViewer } from "../components/LogViewer";
import styles from "../styles/Vault.module.css";

type Tab = "slots" | "history" | "logs";

export function VaultPage() {
  const { slots, status, error, loading, refresh, handlePopSlot } = useVault();
  const [activeTab, setActiveTab] = useState<Tab>("slots");

  return (
    <div className={styles.vaultPage}>
      <header className={styles.vaultHeader}>
        <h1>PassClip</h1>
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

      <nav className={styles.tabNav}>
        <button
          className={`${styles.tabBtn} ${activeTab === "slots" ? styles.tabActive : ""}`}
          onClick={() => setActiveTab("slots")}
        >
          Encrypted Slots ({slots.length})
        </button>
        <button
          className={`${styles.tabBtn} ${activeTab === "history" ? styles.tabActive : ""}`}
          onClick={() => setActiveTab("history")}
        >
          Clipboard History
        </button>
        <button
          className={`${styles.tabBtn} ${activeTab === "logs" ? styles.tabActive : ""}`}
          onClick={() => setActiveTab("logs")}
        >
          Entropy & Logs
        </button>
      </nav>

      <div className={styles.tabContent}>
        {activeTab === "slots" && (
          <div className={styles.slotsList}>
            {slots.length === 0 ? (
              <div className={styles.emptyState}>No encrypted slots in buffer</div>
            ) : (
              slots.map((slot) => (
                <SlotCard key={slot.id} slot={slot} onPop={handlePopSlot} />
              ))
            )}
          </div>
        )}

        {activeTab === "history" && (
          <div className={styles.placeholderPanel}>
            <div className={styles.panelHeader}>
              <span>Clipboard Buffer History</span>
              <span className={styles.panelBadge}>NOT IMPLEMENTED</span>
            </div>
            <div className={styles.unimplementedBanner}>
              [STUB] Clipboard history buffer is currently disabled. Subsystem integration pending daemon RAM-store IPC handler.
            </div>
            <div className={styles.placeholderList}>
              <div className={styles.placeholderRow}>
                <span className={styles.timestamp}>[--:--:--]</span>
                <span className={styles.maskedText}>••••••••••••••••</span>
                <span className={styles.meta}>[Entropy: -.- bits/char]</span>
              </div>
            </div>
          </div>
        )}

        {activeTab === "logs" && <LogViewer />}
      </div>
    </div>
  );
}