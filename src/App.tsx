import { useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import './App.css';

function App() {
  const [regKeys, setRegKeys] = useState<string[]>([]);

  const hammer = async () => {
    const regKeys = await invoke<string[]>('get_usb_keys');
    setRegKeys(regKeys);
  };

  return (
    <div className="container">
      <button onClick={hammer}>Hammer In Your USB History</button>

      <div>
        {regKeys.map((regKey) => (
          <div key={regKey}>{regKey}</div>
        ))}
      </div>
    </div>
  );
}

export default App;
