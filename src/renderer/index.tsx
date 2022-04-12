import { createRoot } from 'react-dom/client';
import App from './App';

const container = document.getElementById('root')!;
const root = createRoot(container);
root.render(<App />);

// window.electron.ipcRenderer.invoke('get-user-path').then((path) => console.log('path:', path))
