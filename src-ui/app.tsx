import { useVault } from './hooks/useVault';

export function App() {
  const { slots, error, loading, refresh } = useVault();

  return (
    <div style={{ padding: '24px', background: '#0f0f11', color: '#e2e8f0', minHeight: '100vh', fontFamily: 'system-ui' }}>
      <header style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '20px' }}>
        <h1 style={{ fontSize: '1.2rem', fontWeight: 'bold' }}>PassClip Vault</h1>
        <button onClick={refresh} disabled={loading} style={{ background: '#27272a', border: '1px solid #3f3f46', color: '#fff', padding: '6px 12px', borderRadius: '6px', cursor: 'pointer' }}>
          {loading ? 'Refreshing...' : 'Refresh'}
        </button>
      </header>

      {error ? (
        <div style={{ background: '#3f1212', border: '1px solid #7f1d1d', color: '#fca5a5', padding: '12px', borderRadius: '8px' }}>
          {error}
        </div>
      ) : (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
          {slots.length === 0 ? (
            <p style={{ color: '#71717a' }}>No secrets currently held in volatile RAM.</p>
          ) : (
            slots.map((slot) => (
              <div key={slot.id} style={{ background: '#18181b', border: '1px solid #27272a', padding: '12px 16px', borderRadius: '8px', display: 'flex', justifyContent: 'space-between' }}>
                <div>
                  <span style={{ fontWeight: 'bold' }}>Slot #{slot.id}</span>
                  <span style={{ marginLeft: '10px', color: '#a1a1aa', fontSize: '0.9rem' }}>Length: {slot.len} chars</span>
                </div>
                <div style={{ color: '#10b981', fontSize: '0.9rem', fontFamily: 'monospace' }}>
                  Entropy: {slot.entropy.toFixed(2)}
                </div>
              </div>
            ))
          )}
        </div>
      )}
    </div>
  );
}