import React, { useEffect, useRef, useState } from 'react';
import { createRoot } from 'react-dom/client';
import { listen } from '@tauri-apps/api/event';
import { Camera, Eye, MessageCircle, Mic, RotateCcw, Send, Settings, Volume2, X } from 'lucide-react';
import './styles.css';

type Mode = 'avatar' | 'chat' | 'settings';
type DeviceStatus = 'unknown' | 'available' | 'active' | 'denied' | 'unavailable';
const providers = ['KENRI', 'OpenAI', 'Claude / Anthropic', 'Gemini', 'Qwen', 'DeepSeek'];

function App() {
  const [mode, setMode] = useState<Mode>('avatar');
  const [name, setName] = useState('Kelly');
  const [avatar, setAvatar] = useState('/avatars/kelly-default.webp');
  const [provider, setProvider] = useState('KENRI');
  const [apiKey, setApiKey] = useState('');
  const [providerStatus, setProviderStatus] = useState('Not tested — provider backend is not connected yet.');
  const [devices, setDevices] = useState<MediaDeviceInfo[]>([]);
  const [cameraId, setCameraId] = useState('');
  const [microphoneId, setMicrophoneId] = useState('');
  const [speakerId, setSpeakerId] = useState('');
  const [cameraStatus, setCameraStatus] = useState<DeviceStatus>('unknown');
  const [microphoneStatus, setMicrophoneStatus] = useState<DeviceStatus>('unknown');
  const [speakerStatus, setSpeakerStatus] = useState<DeviceStatus>('unknown');
  const [cameraActive, setCameraActive] = useState(false);
  const [listening, setListening] = useState(false);
  const [speaking, setSpeaking] = useState(false);
  const [micLevel, setMicLevel] = useState(0);
  const [deviceMessage, setDeviceMessage] = useState('Select Refresh devices to inspect available hardware.');
  const cameraStream = useRef<MediaStream | null>(null);
  const micStream = useRef<MediaStream | null>(null);
  const videoRef = useRef<HTMLVideoElement>(null);
  const analyserFrame = useRef<number>();

  const refreshDevices = async () => {
    if (!navigator.mediaDevices?.enumerateDevices) {
      setDeviceMessage('Media device APIs are unavailable in this runtime.');
      setCameraStatus('unavailable'); setMicrophoneStatus('unavailable'); setSpeakerStatus('unavailable');
      return;
    }
    try {
      const found = await navigator.mediaDevices.enumerateDevices();
      setDevices(found);
      const cameras = found.filter(d => d.kind === 'videoinput');
      const microphones = found.filter(d => d.kind === 'audioinput');
      const speakers = found.filter(d => d.kind === 'audiooutput');
      if (!cameraId && cameras[0]) setCameraId(cameras[0].deviceId);
      if (!microphoneId && microphones[0]) setMicrophoneId(microphones[0].deviceId);
      if (!speakerId && speakers[0]) setSpeakerId(speakers[0].deviceId);
      setCameraStatus(cameras.length ? 'available' : 'unavailable');
      setMicrophoneStatus(microphones.length ? 'available' : 'unavailable');
      setSpeakerStatus(speakers.length || 'speechSynthesis' in window ? 'available' : 'unavailable');
      setDeviceMessage(`Found ${cameras.length} camera(s), ${microphones.length} microphone(s), and ${speakers.length} selectable speaker output(s).`);
    } catch (error) { setDeviceMessage(`Could not enumerate devices: ${String(error)}`); }
  };

  const stopCamera = () => {
    cameraStream.current?.getTracks().forEach(track => track.stop());
    cameraStream.current = null;
    if (videoRef.current) videoRef.current.srcObject = null;
    setCameraActive(false);
    setCameraStatus(devices.some(d => d.kind === 'videoinput') ? 'available' : 'unknown');
  };

  const testCamera = async () => {
    stopCamera(); setDeviceMessage('Requesting camera permission…');
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ video: cameraId ? { deviceId: { exact: cameraId } } : true });
      cameraStream.current = stream; setCameraActive(true); setCameraStatus('active'); setDeviceMessage('Camera preview is active. Stop it when finished testing.');
      requestAnimationFrame(() => { if (videoRef.current) { videoRef.current.srcObject = stream; void videoRef.current.play(); } });
    } catch (error) { setCameraStatus('denied'); setDeviceMessage(`Camera permission/test failed: ${String(error)}`); }
  };

  const stopMicrophone = () => {
    if (analyserFrame.current) cancelAnimationFrame(analyserFrame.current);
    micStream.current?.getTracks().forEach(track => track.stop());
    micStream.current = null; setListening(false); setMicLevel(0);
    setMicrophoneStatus(devices.some(d => d.kind === 'audioinput') ? 'available' : 'unknown');
  };

  const testMicrophone = async () => {
    stopMicrophone(); setDeviceMessage('Requesting microphone permission…');
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: microphoneId ? { deviceId: { exact: microphoneId } } : true });
      micStream.current = stream; setListening(true); setMicrophoneStatus('active'); setDeviceMessage('Microphone test is active. Speak to verify the live input meter.');
      const context = new AudioContext(); const analyser = context.createAnalyser(); analyser.fftSize = 256;
      context.createMediaStreamSource(stream).connect(analyser); const values = new Uint8Array(analyser.frequencyBinCount);
      const sample = () => { analyser.getByteFrequencyData(values); setMicLevel(Math.min(100, Math.round(values.reduce((a, b) => a + b, 0) / values.length))); analyserFrame.current = requestAnimationFrame(sample); };
      sample();
    } catch (error) { setMicrophoneStatus('denied'); setDeviceMessage(`Microphone permission/test failed: ${String(error)}`); }
  };

  const testSpeaker = () => {
    setSpeaking(true); setSpeakerStatus('active'); setDeviceMessage('Playing Kelly speaker test…');
    const utterance = new SpeechSynthesisUtterance('Hello, I am Kelly. Your speaker test is working.');
    utterance.onend = () => { setSpeaking(false); setSpeakerStatus('available'); setDeviceMessage('Speaker test completed.'); };
    utterance.onerror = () => { setSpeaking(false); setSpeakerStatus('unavailable'); setDeviceMessage('The system speech output test failed.'); };
    speechSynthesis.cancel(); speechSynthesis.speak(utterance);
  };

  useEffect(() => {
    void refreshDevices();
    let unlisten: (() => void) | undefined;
    listen('open-settings', () => setMode('settings')).then(fn => { unlisten = fn; }).catch(() => undefined);
    return () => { unlisten?.(); cameraStream.current?.getTracks().forEach(t => t.stop()); micStream.current?.getTracks().forEach(t => t.stop()); speechSynthesis?.cancel(); };
  }, []);

  const deviceOptions = (kind: MediaDeviceKind) => devices.filter(d => d.kind === kind);
  const resetKelly = () => { setName('Kelly'); setAvatar('/avatars/kelly-default.webp'); };
  const testProvider = () => setProviderStatus(apiKey || provider === 'KENRI' ? 'Stub: credential held in memory only; no safe provider backend contract is connected.' : 'Enter a credential to test. Nothing will be saved.');
  const indicator = cameraActive ? 'Camera Active' : listening ? 'Listening' : speaking ? 'Speaking' : 'Ready';

  return <main className="stage">
    <div className={`privacy-indicator ${indicator === 'Ready' ? '' : 'active'}`}>{indicator}</div>
    {mode === 'avatar' && <section className="avatar-wrap" data-tauri-drag-region>
      <img className="kelly" src={avatar} alt="Kelly, French Bulldog desktop companion" draggable={false} data-tauri-drag-region />
      <div className="name" data-tauri-drag-region>{name}</div>
      <div className="controls"><button aria-label="Quick chat" onClick={() => setMode('chat')}><MessageCircle /></button><button aria-label={listening ? 'Stop microphone' : 'Test microphone'} className={listening ? 'sensor-on' : ''} onClick={listening ? stopMicrophone : testMicrophone}><Mic /></button><button aria-label={cameraActive ? 'Stop camera' : 'Start camera'} className={cameraActive ? 'sensor-on' : ''} onClick={cameraActive ? stopCamera : testCamera}><Camera /></button><button aria-label="Setup" onClick={() => setMode('settings')}><Settings /></button></div>
    </section>}
    {mode === 'chat' && <section className="panel chat"><header data-tauri-drag-region><div><b>{name}</b><small>● {indicator}</small></div><button aria-label="Close quick chat" onClick={() => setMode('avatar')}><X /></button></header><div className="messages"><div className="bubble">Hello. Text chat is ready for a future provider adapter.</div></div><footer><input aria-label="Message" placeholder="Type a message…" /><button aria-label="Send"><Send /></button></footer></section>}
    {mode === 'settings' && <section className="panel settings-panel"><header data-tauri-drag-region><div><b>Companion Setup</b><small>KENRI Desktop Companion v0.1</small></div><button aria-label="Close setup" onClick={() => setMode('avatar')}><X /></button></header><div className="scroll">
      <h3>Companion</h3><label>Name<input value={name} onChange={e => setName(e.target.value)} /></label><label>Avatar<input type="file" accept="image/*" onChange={e => { const file = e.target.files?.[0]; if (file) setAvatar(URL.createObjectURL(file)); }} /></label><button className="secondary" onClick={resetKelly}><RotateCcw /> Reset to Kelly</button>
      <h3>Personality</h3><label>Personality<textarea defaultValue="Warm, loyal, a little confused, brave and friendly." /></label>
      <h3>AI Provider</h3><label>Provider<select value={provider} onChange={e => { setProvider(e.target.value); setProviderStatus('Not tested — provider backend is not connected yet.'); }}>{providers.map(p => <option key={p}>{p}</option>)}</select></label><label>API Key / Credential<input type="password" autoComplete="off" value={apiKey} onChange={e => setApiKey(e.target.value)} placeholder="Runtime memory only — never persisted" /></label><p className="notice">Credentials are not saved, logged, or written to localStorage. Secure OS credential integration remains a roadmap item.</p><div className="row"><button onClick={testProvider}>Test Connection</button><span>{providerStatus}</span></div>
      <h3>Devices</h3><button className="secondary" onClick={refreshDevices}>Refresh devices</button>
      <DeviceSelect label="Camera" value={cameraId} options={deviceOptions('videoinput')} onChange={setCameraId} status={cameraStatus} /><div className="device-actions"><button onClick={cameraActive ? stopCamera : testCamera}><Camera /> {cameraActive ? 'Stop Camera' : 'Test Camera'}</button></div>{cameraActive && <div className="preview"><div className="preview-label"><Eye /> Camera Active</div><video ref={videoRef} muted playsInline /></div>}
      <DeviceSelect label="Microphone" value={microphoneId} options={deviceOptions('audioinput')} onChange={setMicrophoneId} status={microphoneStatus} /><div className="device-actions"><button onClick={listening ? stopMicrophone : testMicrophone}><Mic /> {listening ? 'Stop Microphone' : 'Test Microphone'}</button><meter min="0" max="100" value={micLevel} /></div>
      <DeviceSelect label="Speaker" value={speakerId} options={deviceOptions('audiooutput')} onChange={setSpeakerId} status={speakerStatus} note={deviceOptions('audiooutput').length ? 'Selection is exposed where the runtime supports output routing.' : 'This runtime does not expose selectable audio outputs; the system default will be used.'} /><div className="device-actions"><button onClick={testSpeaker}><Volume2 /> Test Speaker</button></div><p className="device-message">{deviceMessage}</p>
      <h3>Desktop</h3><label className="check"><input type="checkbox" defaultChecked /> Always on top</label><label className="check"><input type="checkbox" defaultChecked /> Remember position</label><label className="check"><input type="checkbox" /> Start with OS <small>(roadmap)</small></label><p className="notice">System tray show/hide is active in Tauri. Autostart, global shortcut, secure credential storage, STT, Vision-provider upload, and live AI responses require native/provider adapters.</p>
    </div></section>}
  </main>;
}

function DeviceSelect({ label, value, options, onChange, status, note }: { label: string; value: string; options: MediaDeviceInfo[]; onChange: (v: string) => void; status: DeviceStatus; note?: string }) {
  return <label>{label} <span className={`status ${status}`}>{status}</span><select value={value} onChange={e => onChange(e.target.value)} disabled={!options.length}><option value="">System default</option>{options.map((d, i) => <option key={d.deviceId || i} value={d.deviceId}>{d.label || `${label} ${i + 1}`}</option>)}</select>{note && <small className="field-note">{note}</small>}</label>;
}

createRoot(document.getElementById('root')!).render(<App />);
