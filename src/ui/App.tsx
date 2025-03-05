import React from 'react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import Dashboard from './pages/Dashboard';
import Parts from './pages/Parts';
import PartDetail from './pages/PartDetail';
import Workspaces from './pages/Workspaces';
import WorkspaceDetail from './pages/WorkspaceDetail';
import Reviews from './pages/Reviews';
import Settings from './pages/Settings';
import MainLayout from './layouts/MainLayout';
import { UIProvider } from './context/UIContext';
import { PartsProvider } from './context/PartsContext';
import { WorkspaceProvider } from './context/WorkspaceContext';
import { AuthProvider } from './context/AuthContext';
import { NotificationProvider } from './context/NotificationContext';
import { RepositoryProvider } from './context/RepositoryContext';

/**
 * Main App component that sets up routing and context providers
 */
function App() {
  return (
    <AuthProvider>
      <NotificationProvider>
        <RepositoryProvider>
          <UIProvider>
            <PartsProvider>
              <WorkspaceProvider>
                <BrowserRouter>
                  <Routes>
                    <Route path="/" element={<MainLayout />}>
                      <Route index element={<Dashboard />} />
                      <Route path="parts" element={<Parts />} />
                      <Route path="parts/:partId" element={<PartDetail />} />
                      <Route path="workspaces" element={<Workspaces />} />
                      <Route path="workspaces/:workspaceId" element={<WorkspaceDetail />} />
                      <Route path="reviews" element={<Reviews />} />
                      <Route path="settings" element={<Settings />} />
                    </Route>
                  </Routes>
                </BrowserRouter>
              </WorkspaceProvider>
            </PartsProvider>
          </UIProvider>
        </RepositoryProvider>
      </NotificationProvider>
    </AuthProvider>
  );
}

export default App;