import { UiSlot } from '../api/api';

interface SlotCardProps {
  slot: UiSlot;
  onPop: (id: number) => void;
}

export function SlotCard({ slot, onPop }: SlotCardProps) {
  return (
    <div className="slot-card">
      <div className="slot-info">
        <span className="slot-id">Slot #{slot.id}</span>
        <span className="slot-detail">Length: {slot.len}</span>
        <span className="slot-detail">Entropy: {slot.entropy.toFixed(2)}</span>
      </div>
      <button className="pop-btn" onClick={() => onPop(slot.id)}>
        Pop
      </button>
    </div>
  );
}