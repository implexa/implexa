import React from 'react';
import ReactDOM from 'react-dom/client';
import { invoke } from '@tauri-apps/api/tauri';
import App from './ui/App';
import './styles.css';

// Initialize Tauri commands
async function initTauri() {
  try {
    // Initialize the Git backend
    await invoke('init_git_backend');
    console.log('Git backend initialized');
  } catch (error) {
    console.error('Failed to initialize Git backend:', error);
  }
}

// Initialize Tauri
initTauri().catch(console.error);

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);