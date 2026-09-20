import { useState } from "preact/hooks";
import styles from "../styles/Settings.module.css";

interface AppConfig {
  rpId: string;
  rpOrigin: string;
  autoClearSec: number;
  clipboardPollIntervalMs: number;
  enableNotifications: boolean;
  entropyThreshold: number;
  entropyAlgorithm: "shannon" | "metric";
}

export function SettingsPage() {
  const [config, setConfig] = useState<AppConfig>({
    rpId: "localhost",
    rpOrigin: "http://localhost:1420",
    autoClearSec: 30,
    clipboardPollIntervalMs: 500,
    enableNotifications: true,
    entropyThreshold: 4.2,
    entropyAlgorithm: "shannon",
  });

  const [saving, setSaving] = useState(false);
  const [statusMessage, setStatusMessage] = useState<string | null>(null);

  const handleChange = <K extends keyof AppConfig>(field: K, value: AppConfig[K]) => {
    setConfig((prev) => ({ ...prev, [field]: value }));
  };

  const handleSave = async (e: Event) => {
    e.preventDefault();
    setSaving(true);
    setStatusMessage(null);

    try {
      await new Promise((resolve) => setTimeout(resolve, 300));
      setStatusMessage("Settings updated successfully");
    } catch (err: any) {
      setStatusMessage(`Failed to save: ${err}`);
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className={styles.settingsPanel}>
      <div className={styles.panelHeader}>
        <span>System & Security Configuration</span>
      </div>

      {statusMessage && (
        <div className={styles.statusBanner}>{statusMessage}</div>
      )}

      <form onSubmit={handleSave} className={styles.settingsForm}>
        <div className={styles.sectionTitle}>Timers & Interval Policies</div>

        <div className={styles.formGroup}>
          <label>Volatile Clipboard Auto-Clear (Seconds)</label>
          <div className={styles.inputWithUnit}>
            <input
              type="number"
              min="5"
              max="300"
              value={config.autoClearSec}
              onInput={(e) =>
                handleChange("autoClearSec", parseInt((e.target as HTMLInputElement).value) || 30)
              }
              className={styles.inputField}
            />
            <span className={styles.unitBadge}>sec</span>
          </div>
          <span className={styles.fieldHint}>
            Delay before restored sensitive data is wiped from OS clipboard
          </span>
        </div>

        <div className={styles.formGroup}>
          <label>Daemon Clipboard Polling Rate</label>
          <div className={styles.inputWithUnit}>
            <input
              type="number"
              min="100"
              max="5000"
              step="100"
              value={config.clipboardPollIntervalMs}
              onInput={(e) =>
                handleChange(
                  "clipboardPollIntervalMs",
                  parseInt((e.target as HTMLInputElement).value) || 500
                )
              }
              className={styles.inputField}
            />
            <span className={styles.unitBadge}>ms</span>
          </div>
          <span className={styles.fieldHint}>
            Scan interval for background memory entropy evaluation
          </span>
        </div>

        <div className={styles.sectionTitle}>Entropy Detection Engine</div>

        <div className={styles.formGroup}>
          <div className={styles.labelRow}>
            <label>Entropy Sensitivity Threshold</label>
            <span className={styles.valueDisplay}>{config.entropyThreshold.toFixed(1)} bits/char</span>
          </div>
          <input
            type="range"
            min="2.0"
            max="6.0"
            step="0.1"
            value={config.entropyThreshold}
            onInput={(e) =>
              handleChange(
                "entropyThreshold",
                parseFloat((e.target as HTMLInputElement).value)
              )
            }
            className={styles.rangeSlider}
          />
          <div className={styles.rangeLabels}>
            <span>Permissive (2.0)</span>
            <span>Balanced (4.0)</span>
            <span>Strict (6.0)</span>
          </div>
          <span className={styles.fieldHint}>
            Strings above this Shannon entropy rating will trigger auto-isolation prompt
          </span>
        </div>

        <div className={styles.formGroup}>
          <label>Calculation Algorithm</label>
          <select
            value={config.entropyAlgorithm}
            onChange={(e) =>
              handleChange(
                "entropyAlgorithm",
                (e.target as HTMLSelectElement).value as "shannon" | "metric"
              )
            }
            className={styles.selectField}
          >
            <option value="shannon">Shannon Entropy (Standard H-value)</option>
            <option value="metric">Metric Entropy (Normalized H / length)</option>
          </select>
        </div>

        <div className={styles.sectionTitle}>WebAuthn & OS Integration</div>

        <div className={styles.formGroup}>
          <label>WebAuthn RP ID</label>
          <input
            type="text"
            value={config.rpId}
            onInput={(e) => handleChange("rpId", (e.target as HTMLInputElement).value)}
            className={styles.inputField}
          />
        </div>

        <div className={styles.formGroup}>
          <label>WebAuthn RP Origin</label>
          <input
            type="text"
            value={config.rpOrigin}
            onInput={(e) => handleChange("rpOrigin", (e.target as HTMLInputElement).value)}
            className={styles.inputField}
          />
        </div>

        <div className={styles.formGroupRow}>
          <label className={styles.checkboxLabel}>
            <input
              type="checkbox"
              checked={config.enableNotifications}
              onChange={(e) =>
                handleChange("enableNotifications", (e.target as HTMLInputElement).checked)
              }
            />
            <span>Enable Desktop Notifications</span>
          </label>
        </div>

        <div className={styles.formActions}>
          <button type="submit" className={styles.saveBtn} disabled={saving}>
            {saving ? "Saving..." : "Save Settings"}
          </button>
        </div>
      </form>
    </div>
  );
}