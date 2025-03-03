import React, { createContext, useContext, useState, ReactNode } from 'react';

interface UIContextType {
  sidebarOpen: boolean;
  toggleSidebar: () => void;
  theme: 'light' | 'dark' | 'system';
  setTheme: (theme: 'light' | 'dark' | 'system') => void;
}

const UIContext = createContext<UIContextType | undefined>(undefined);

interface UIProviderProps {
  children: ReactNode;
}

/**
 * Provider component for UI-related state
 */
export const UIProvider: React.FC<UIProviderProps> = ({ children }) => {
  const [sidebarOpen, setSidebarOpen] = useState(true);
  const [theme, setTheme] = useState<'light' | 'dark' | 'system'>('system');

  const toggleSidebar = () => {
    setSidebarOpen(!sidebarOpen);
  };

  const value = {
    sidebarOpen,
    toggleSidebar,
    theme,
    setTheme,
  };

  return <UIContext.Provider value={value}>{children}</UIContext.Provider>;
};

/**
 * Hook for accessing UI context
 */
export const useUI = (): UIContextType => {
  const context = useContext(UIContext);
  if (context === undefined) {
    throw new Error('useUI must be used within a UIProvider');
  }
  return context;
};