import { useEffect, useRef } from "preact/hooks";
import { useLogs } from "../hooks/useLogs";
import styles from "../styles/Vault.module.css";

export function LogViewer() {
  const { logs, loading, clearLogs } = useLogs();
  const logContainerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (logContainerRef.current) {
      logContainerRef.current.scrollTop = logContainerRef.current.scrollHeight;
    }
  }, [logs]);

  return (
    <div className={styles.placeholderPanel}>
      <div className={styles.panelHeader}>
        <span>Daemon Telemetry & Process Logs</span>
        <button className={styles.refreshBtn} onClick={clearLogs}>
          Clear Logs
        </button>
      </div>

      <div className={styles.logBox} ref={logContainerRef}>
        {loading ? (
          <div>Loading telemetry stream...</div>
        ) : logs.length === 0 ? (
          <div>No log entries recorded.</div>
        ) : (
          logs.map((log) => (
            <div key={log.id} className={styles.placeholderRow}>
              <span className={styles.timestamp}>[{log.timestamp}]</span>
              <span className={styles.meta}>[{log.level}]</span>
              <span className={styles.maskedText}>{log.message}</span>
            </div>
          ))
        )}
      </div>
    </div>
  );
}