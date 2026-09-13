import { useState, useEffect } from 'preact/hooks';
import { 
  getVaultSlots, 
  getVaultStatus, 
  popSlot, 
  verifyAssertion, 
  UiSlot, 
  UiStatus 
} from '../api/api';

export function useVault() {
  const [slots, setSlots] = useState<UiSlot[]>([]);
  const [status, setStatus] = useState<UiStatus | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const refreshData = async () => {
    try {
      setLoading(true);
      setError(null);
      const [fetchedSlots, fetchedStatus] = await Promise.all([
        getVaultSlots(),
        getVaultStatus(),
      ]);
      setSlots(fetchedSlots);
      setStatus(fetchedStatus);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    refreshData();
    const timer = setInterval(refreshData, 3000);
    return () => clearInterval(timer);
  }, []);

  const handlePopSlot = async (id: number) => {
    try {
      setError(null);
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

  return { 
    slots, 
    status, 
    error, 
    loading, 
    refresh: refreshData, 
    handlePopSlot 
  };
}