/** Kelly Mini Brain v0.1 — local, deterministic, no network or UI side effects. */
export type BrainMood = 'calm' | 'curious' | 'content' | 'tired';
export type BrainEvent = 'interaction' | 'chat-start' | 'chat-end' | 'rest';
export type BrainSnapshot = Readonly<{
  energy: number;
  curiosity: number;
  socialNeed: number;
  mood: BrainMood;
  updatedAt: number;
}>;
type Listener = (snapshot: BrainSnapshot) => void;
const clamp = (n: number) => Math.max(0, Math.min(100, n));
const INITIAL = { energy: 80, curiosity: 55, socialNeed: 25 };
export class KellyMiniBrain {
  private state: BrainSnapshot;
  private listeners = new Set<Listener>();
  constructor(now = Date.now()) {
    this.state = { ...INITIAL, mood: 'calm', updatedAt: now };
  }
  getSnapshot(): BrainSnapshot { return { ...this.state }; }
  subscribe(listener: Listener): () => void {
    this.listeners.add(listener);
    return () => { this.listeners.delete(listener); };
  }
  /** Advance virtual biological time. Call explicitly; no background timer is started. */
  tick(now = Date.now()): BrainSnapshot {
    const elapsedMinutes = Math.max(0, Math.min((now - this.state.updatedAt) / 60000, 1440));
    return this.apply({
      energy: clamp(this.state.energy - elapsedMinutes * 0.06),
      curiosity: clamp(this.state.curiosity + elapsedMinutes * 0.08),
      socialNeed: clamp(this.state.socialNeed + elapsedMinutes * 0.10),
    }, Math.max(now, this.state.updatedAt));
  }
  dispatch(event: BrainEvent, now = Date.now()): BrainSnapshot {
    this.tick(now);
    const { energy, curiosity, socialNeed } = this.state;
    switch (event) {
      case 'interaction': return this.apply({ energy: clamp(energy - 1), curiosity: clamp(curiosity + 4), socialNeed: clamp(socialNeed - 8) }, now);
      case 'chat-start': return this.apply({ energy: clamp(energy - 2), curiosity: clamp(curiosity + 6), socialNeed: clamp(socialNeed - 12) }, now);
      case 'chat-end': return this.apply({ energy, curiosity: clamp(curiosity - 3), socialNeed: clamp(socialNeed - 5) }, now);
      case 'rest': return this.apply({ energy: clamp(energy + 15), curiosity, socialNeed }, now);
    }
  }
  private apply(values: typeof INITIAL, now: number): BrainSnapshot {
    const mood: BrainMood = values.energy < 25 ? 'tired' : values.curiosity > 75 ? 'curious' : values.socialNeed < 15 ? 'content' : 'calm';
    this.state = { ...values, mood, updatedAt: Math.max(now, this.state.updatedAt) };
    const snapshot = this.getSnapshot();
    for (const listener of this.listeners) listener(snapshot);
    return snapshot;
  }
}
/** Singleton shared by the widget's main window; no autonomous behavior yet. */
export const kellyBrain = new KellyMiniBrain();
