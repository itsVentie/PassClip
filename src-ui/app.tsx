import { useEffect, useState } from "preact/hooks";
import { getVaultSlots, getVaultStatus, popSlot, UiSlot, UiStatus } from "./api/api";

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
    const challengeStr = await popSlot(id);
    const options = JSON.parse(challengeStr);

    options.publicKey.challenge = Uint8Array.from(
      atob(options.publicKey.challenge.replace(/-/g, "+").replace(/_/g, "/")),
      (c) => c.charCodeAt(0)
    );

    if (options.publicKey.allowCredentials) {
      options.publicKey.allowCredentials = options.publicKey.allowCredentials.map((cred: any) => ({
        ...cred,
        id: Uint8Array.from(atob(cred.id.replace(/-/g, "+").replace(/_/g, "/")), (c) => c.charCodeAt(0)),
      }));
    }

    const credential = (await navigator.credentials.get({
      publicKey: options.publicKey,
    })) as PublicKeyCredential;

    if (!credential) {
      setError("Passkey authentication was cancelled.");
      return;
    }

    const responseAuth = credential.response as AuthenticatorAssertionResponse;
    const assertionPayload = JSON.stringify({
      id: credential.id,
      rawId: Array.from(new Uint8Array(credential.rawId)),
      type: credential.type,
      response: {
        authenticatorData: Array.from(new Uint8Array(responseAuth.authenticatorData)),
        clientDataJSON: Array.from(new Uint8Array(responseAuth.clientDataJSON)),
        signature: Array.from(new Uint8Array(responseAuth.signature)),
        userHandle: responseAuth.userHandle ? Array.from(new Uint8Array(responseAuth.userHandle)) : null,
      },
    });

    const secret = await verifyAssertion(assertionPayload);
    console.log("Secret decrypted:", secret);

    await refreshData();
  } catch (err) {
    setError(`Passkey error: ${err}`);
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