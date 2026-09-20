import { useState, useEffect } from "preact/hooks";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export interface LogEntry {
  id: string;
  timestamp: string;
  level: "INFO" | "WARN" | "ERROR" | "DEBUG";
  message: string;
}

export function useLogs() {
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let unlisten: (() => void) | undefined;

    async function initLogs() {
      try {
        const initialLogs = await invoke<LogEntry[]>("get_logs");
        setLogs(initialLogs);
      } catch (err) {
        console.error("Failed to fetch initial logs:", err);
      } finally {
        setLoading(false);
      }

      unlisten = await listen<LogEntry>("daemon-log", (event) => {
        setLogs((prev) => [...prev.slice(-199), event.payload]);
      });
    }

    initLogs();

    return () => {
      if (unlisten) unlisten();
    };
  }, []);

  const clearLogs = async () => {
    try {
      await invoke("clear_logs");
      setLogs([]);
    } catch (err) {
      console.error("Failed to clear logs:", err);
    }
  };

  return { logs, loading, clearLogs };
}