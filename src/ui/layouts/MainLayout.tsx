import React from 'react';
import { Outlet } from 'react-router-dom';
import Navigation from '../components/Navigation';
import Notifications from '../components/Notifications';
import { useUI } from '../context/UIContext';

/**
 * Main layout component that includes navigation and content area
 */
const MainLayout: React.FC = () => {
  const { sidebarOpen, theme } = useUI();

  return (
    <div className={`min-h-screen ${theme === 'dark' ? 'dark' : ''}`}>
      <div className="flex h-screen bg-gray-100 dark:bg-gray-900">
        {/* Sidebar Navigation */}
        <div
          className={`${
            sidebarOpen ? 'w-64' : 'w-20'
          } transition-width duration-300 ease-in-out bg-white dark:bg-gray-800 shadow-md`}
        >
          <Navigation collapsed={!sidebarOpen} />
        </div>

        {/* Main Content */}
        <div className="flex-1 flex flex-col overflow-hidden">
          {/* Header */}
          <header className="bg-white dark:bg-gray-800 shadow-sm z-10">
            <div className="px-4 py-3 flex justify-between items-center">
              <h1 className="text-xl font-semibold text-gray-800 dark:text-white">Implexa</h1>
              <div className="flex items-center space-x-4">
                {/* User profile, theme toggle, etc. could go here */}
              </div>
            </div>
          </header>

          {/* Content Area */}
          <main className="flex-1 overflow-auto bg-gray-100 dark:bg-gray-900 p-4">
            <Outlet />
          </main>
        </div>
      </div>

      {/* Notifications */}
      <Notifications />
    </div>
  );
};

export default MainLayout;