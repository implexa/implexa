import React from 'react';
import { NavLink } from 'react-router-dom';
import { useUI } from '../context/UIContext';

interface NavigationProps {
  collapsed: boolean;
}

/**
 * Navigation component for the sidebar
 */
const Navigation: React.FC<NavigationProps> = ({ collapsed }) => {
  const { toggleSidebar } = useUI();

  // Navigation items
  const navItems = [
    { path: '/', label: 'Dashboard', icon: '📊' },
    { path: '/parts', label: 'Parts', icon: '🔩' },
    { path: '/workspaces', label: 'Workspaces', icon: '📁' },
    { path: '/reviews', label: 'Reviews', icon: '📝' },
    { path: '/settings', label: 'Settings', icon: '⚙️' },
  ];

  return (
    <nav className="h-full flex flex-col">
      {/* Logo and Toggle */}
      <div className="p-4 flex items-center justify-between border-b border-gray-200 dark:border-gray-700">
        {!collapsed && <span className="text-lg font-bold text-gray-800 dark:text-white">Implexa</span>}
        <button
          onClick={toggleSidebar}
          className="p-2 rounded-md text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-white"
          aria-label={collapsed ? 'Expand sidebar' : 'Collapse sidebar'}
        >
          {collapsed ? '→' : '←'}
        </button>
      </div>

      {/* Navigation Links */}
      <div className="flex-1 py-4 overflow-y-auto">
        <ul className="space-y-2 px-2">
          {navItems.map((item) => (
            <li key={item.path}>
              <NavLink
                to={item.path}
                className={({ isActive }) =>
                  `flex items-center p-2 rounded-md transition-colors ${
                    isActive
                      ? 'bg-blue-100 text-blue-700 dark:bg-blue-900 dark:text-blue-200'
                      : 'text-gray-700 hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700'
                  }`
                }
              >
                <span className="text-xl">{item.icon}</span>
                {!collapsed && <span className="ml-3">{item.label}</span>}
              </NavLink>
            </li>
          ))}
        </ul>
      </div>

      {/* User Section */}
      <div className="p-4 border-t border-gray-200 dark:border-gray-700">
        <div className="flex items-center">
          <div className="w-8 h-8 rounded-full bg-gray-300 dark:bg-gray-600 flex items-center justify-center">
            <span className="text-sm font-medium text-gray-700 dark:text-gray-300">U</span>
          </div>
          {!collapsed && (
            <div className="ml-3">
              <p className="text-sm font-medium text-gray-700 dark:text-gray-300">User Name</p>
              <p className="text-xs text-gray-500 dark:text-gray-400">user@example.com</p>
            </div>
          )}
        </div>
      </div>
    </nav>
  );
};

export default Navigation;