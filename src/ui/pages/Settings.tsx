import React, { useState } from 'react';
import { useUI } from '../context/UIContext';
import { useAuth } from '../context/AuthContext';
import { useNotifications } from '../context/NotificationContext';

/**
 * Settings page component
 * Allows users to configure application settings
 */
const Settings: React.FC = () => {
  const { theme, setTheme } = useUI();
  const { user } = useAuth();
  const { addNotification } = useNotifications();
  const [gitPath, setGitPath] = useState('git');
  const [gitLfsPath, setGitLfsPath] = useState('git-lfs');
  const [defaultWorkspacePath, setDefaultWorkspacePath] = useState('');
  const [autoSaveInterval, setAutoSaveInterval] = useState(5);
  const [enableNotifications, setEnableNotifications] = useState(true);

  // Handle theme change
  const handleThemeChange = (newTheme: 'light' | 'dark' | 'system') => {
    setTheme(newTheme);
    addNotification({
      type: 'success',
      message: `Theme changed to ${newTheme}`,
      title: 'Settings Updated',
    });
  };

  // Handle form submission
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    
    // In a real app, this would save the settings to the backend
    addNotification({
      type: 'success',
      message: 'Settings saved successfully',
      title: 'Settings Updated',
    });
  };

  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-bold text-gray-800 dark:text-white">Settings</h1>

      <div className="bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden">
        <div className="border-b border-gray-200 dark:border-gray-700">
          <nav className="flex -mb-px">
            <button className="py-4 px-6 text-sm font-medium border-b-2 border-blue-500 text-blue-600 dark:text-blue-400">
              General
            </button>
            <button className="py-4 px-6 text-sm font-medium text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300">
              Git
            </button>
            <button className="py-4 px-6 text-sm font-medium text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300">
              CAD Integration
            </button>
            <button className="py-4 px-6 text-sm font-medium text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300">
              Advanced
            </button>
          </nav>
        </div>

        <form onSubmit={handleSubmit} className="p-6 space-y-6">
          {/* Appearance Section */}
          <div>
            <h2 className="text-lg font-medium text-gray-900 dark:text-white">Appearance</h2>
            <div className="mt-4 space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Theme</label>
                <div className="mt-2 grid grid-cols-3 gap-3">
                  <div
                    className={`flex items-center justify-center p-3 border rounded-md cursor-pointer ${
                      theme === 'light'
                        ? 'border-blue-500 bg-blue-50 dark:bg-blue-900'
                        : 'border-gray-300 dark:border-gray-600'
                    }`}
                    onClick={() => handleThemeChange('light')}
                  >
                    <span className="text-lg mr-2">☀️</span>
                    <span className="text-sm font-medium">Light</span>
                  </div>
                  <div
                    className={`flex items-center justify-center p-3 border rounded-md cursor-pointer ${
                      theme === 'dark'
                        ? 'border-blue-500 bg-blue-50 dark:bg-blue-900'
                        : 'border-gray-300 dark:border-gray-600'
                    }`}
                    onClick={() => handleThemeChange('dark')}
                  >
                    <span className="text-lg mr-2">🌙</span>
                    <span className="text-sm font-medium">Dark</span>
                  </div>
                  <div
                    className={`flex items-center justify-center p-3 border rounded-md cursor-pointer ${
                      theme === 'system'
                        ? 'border-blue-500 bg-blue-50 dark:bg-blue-900'
                        : 'border-gray-300 dark:border-gray-600'
                    }`}
                    onClick={() => handleThemeChange('system')}
                  >
                    <span className="text-lg mr-2">🖥️</span>
                    <span className="text-sm font-medium">System</span>
                  </div>
                </div>
              </div>
            </div>
          </div>

          {/* Git Configuration Section */}
          <div>
            <h2 className="text-lg font-medium text-gray-900 dark:text-white">Git Configuration</h2>
            <div className="mt-4 space-y-4">
              <div>
                <label htmlFor="git-path" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                  Git Executable Path
                </label>
                <input
                  type="text"
                  id="git-path"
                  value={gitPath}
                  onChange={(e) => setGitPath(e.target.value)}
                  className="mt-1 block w-full rounded-md border-gray-300 dark:border-gray-600 shadow-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-gray-700 dark:text-white sm:text-sm"
                />
                <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
                  Path to the Git executable. Use 'git' for the default system Git.
                </p>
              </div>

              <div>
                <label htmlFor="git-lfs-path" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                  Git LFS Executable Path
                </label>
                <input
                  type="text"
                  id="git-lfs-path"
                  value={gitLfsPath}
                  onChange={(e) => setGitLfsPath(e.target.value)}
                  className="mt-1 block w-full rounded-md border-gray-300 dark:border-gray-600 shadow-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-gray-700 dark:text-white sm:text-sm"
                />
                <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
                  Path to the Git LFS executable. Use 'git-lfs' for the default system Git LFS.
                </p>
              </div>
            </div>
          </div>

          {/* Workspace Section */}
          <div>
            <h2 className="text-lg font-medium text-gray-900 dark:text-white">Workspace</h2>
            <div className="mt-4 space-y-4">
              <div>
                <label htmlFor="default-workspace" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                  Default Workspace Path
                </label>
                <div className="mt-1 flex rounded-md shadow-sm">
                  <input
                    type="text"
                    id="default-workspace"
                    value={defaultWorkspacePath}
                    onChange={(e) => setDefaultWorkspacePath(e.target.value)}
                    className="flex-1 rounded-none rounded-l-md border-gray-300 dark:border-gray-600 focus:border-blue-500 focus:ring-blue-500 dark:bg-gray-700 dark:text-white sm:text-sm"
                  />
                  <button
                    type="button"
                    className="inline-flex items-center rounded-r-md border border-l-0 border-gray-300 dark:border-gray-600 bg-gray-50 dark:bg-gray-600 px-3 text-sm text-gray-500 dark:text-gray-300"
                    onClick={() => {
                      // In a real app, this would open a file picker
                      alert('Select a directory');
                    }}
                  >
                    Browse
                  </button>
                </div>
                <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
                  Default location for new workspaces.
                </p>
              </div>

              <div>
                <label htmlFor="auto-save" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                  Auto-Save Interval (minutes)
                </label>
                <input
                  type="number"
                  id="auto-save"
                  min="1"
                  max="60"
                  value={autoSaveInterval}
                  onChange={(e) => setAutoSaveInterval(parseInt(e.target.value, 10))}
                  className="mt-1 block w-full rounded-md border-gray-300 dark:border-gray-600 shadow-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-gray-700 dark:text-white sm:text-sm"
                />
                <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
                  How often to automatically save changes.
                </p>
              </div>
            </div>
          </div>

          {/* Notifications Section */}
          <div>
            <h2 className="text-lg font-medium text-gray-900 dark:text-white">Notifications</h2>
            <div className="mt-4 space-y-4">
              <div className="flex items-start">
                <div className="flex items-center h-5">
                  <input
                    id="enable-notifications"
                    type="checkbox"
                    checked={enableNotifications}
                    onChange={(e) => setEnableNotifications(e.target.checked)}
                    className="h-4 w-4 rounded border-gray-300 dark:border-gray-600 text-blue-600 focus:ring-blue-500"
                  />
                </div>
                <div className="ml-3 text-sm">
                  <label htmlFor="enable-notifications" className="font-medium text-gray-700 dark:text-gray-300">
                    Enable Notifications
                  </label>
                  <p className="text-gray-500 dark:text-gray-400">
                    Receive notifications for important events like part status changes.
                  </p>
                </div>
              </div>
            </div>
          </div>

          {/* User Information Section */}
          <div>
            <h2 className="text-lg font-medium text-gray-900 dark:text-white">User Information</h2>
            <div className="mt-4 space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Name</label>
                <p className="mt-1 text-sm text-gray-900 dark:text-white">{user?.name || 'Not logged in'}</p>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Email</label>
                <p className="mt-1 text-sm text-gray-900 dark:text-white">{user?.email || 'Not logged in'}</p>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">Role</label>
                <p className="mt-1 text-sm text-gray-900 dark:text-white">{user?.role || 'Not logged in'}</p>
              </div>
            </div>
          </div>

          {/* Submit Button */}
          <div className="flex justify-end">
            <button
              type="submit"
              className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
            >
              Save Settings
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default Settings;