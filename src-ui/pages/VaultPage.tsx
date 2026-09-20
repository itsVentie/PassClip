import { useState } from "preact/hooks";
import { useVault } from "../hooks/useVault";
import { SlotCard } from "../components/SlotCard";
import { LogViewer } from "../components/LogViewer";
import { SettingsPage } from "./Settings";
import styles from "../styles/Vault.module.css";

type Tab = "slots" | "history" | "logs" | "settings";

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
        <div className={styles.tabGroup}>
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
        </div>

        <button
          className={`${styles.settingsTabBtn} ${activeTab === "settings" ? styles.tabActive : ""}`}
          onClick={() => setActiveTab("settings")}
          title="Settings"
          aria-label="Settings"
        >
          <svg
            className={styles.gearIcon}
            viewBox="0 0 24 24"
            width="18"
            height="18"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
          >
            <path d="M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" />
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
          </svg>
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
        {activeTab === "settings" && <SettingsPage />}
      </div>
    </div>
  );
}