import { invoke } from "@tauri-apps/api/core";

export interface UiSlot {
  id: number;
  len: number;
  entropy: number;
  timestamp: number;
}

export interface UiStatus {
  has_secret: boolean;
  count: number;
  max_slots: number;
}

export const getVaultSlots = async (): Promise<UiSlot[]> => {
  return await invoke<UiSlot[]>("get_vault_slots");
};

export const getVaultStatus = async (): Promise<UiStatus> => {
  return await invoke<UiStatus>("get_vault_status");
};

export const popSlot = async (id: number): Promise<string> => {
  return await invoke<string>("pop_slot", { id });
};

export const verifyAssertion = async (assertionJson: string): Promise<string> => {
  return await invoke<string>("verify_assertion", { assertionJson });
};