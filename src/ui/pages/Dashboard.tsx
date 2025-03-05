import React, { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { useParts } from '../context/PartsContext';
import { useWorkspace } from '../context/WorkspaceContext';
import { useRepository } from '../context/RepositoryContext';
import { open } from '@tauri-apps/api/dialog';

/**
 * Dashboard page component
 * Displays an overview of recent activity and quick access to common tasks
 */
const Dashboard: React.FC = () => {
  const { parts, fetchParts } = useParts();
  const { workspaces, fetchWorkspaces } = useWorkspace();
  const {
    repository,
    isLoading,
    error,
    createRepository,
    openRepository,
    closeRepository,
    selectDirectoryTemplate,
    selectedTemplate
  } = useRepository();
  
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [repoName, setRepoName] = useState('');
  const [repoPath, setRepoPath] = useState('');

  // Fetch data on component mount
  useEffect(() => {
    fetchParts();
    fetchWorkspaces();
  }, [fetchParts, fetchWorkspaces]);
  
  // Handle repository creation
  const handleCreateRepository = async () => {
    if (!repoPath || !repoName) return;
    
    await createRepository(repoPath, repoName);
    setShowCreateModal(false);
  };
  
  // Handle repository path selection
  const handleSelectPath = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Parent Directory'
    });
    
    if (selected && typeof selected === 'string') {
      setRepoPath(selected);
    }
  };

  // Get recent parts (last 5)
  const recentParts = [...parts]
    .sort((a, b) => new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime())
    .slice(0, 5);

  // Get recent workspaces (last 3)
  const recentWorkspaces = [...workspaces]
    .sort((a, b) => new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime())
    .slice(0, 3);

  // Mock recent activity data
  const recentActivity = [
    { id: 1, type: 'part_updated', partId: 'EL-RES-100042', partName: 'Resistor 10K', timestamp: '2025-03-02T14:30:00Z' },
    { id: 2, type: 'part_created', partId: 'EL-CAP-100103', partName: 'Capacitor 100nF', timestamp: '2025-03-01T10:15:00Z' },
    { id: 3, type: 'part_reviewed', partId: 'EL-PCB-100054', partName: 'Main Board PCB', timestamp: '2025-02-28T16:45:00Z' },
    { id: 4, type: 'workspace_created', workspaceId: 'ws-123', workspaceName: 'Power Supply Redesign', timestamp: '2025-02-27T09:20:00Z' },
    { id: 5, type: 'part_released', partId: 'ME-3DM-100076', partName: 'Enclosure Model', timestamp: '2025-02-26T11:30:00Z' },
  ];

  // Mock pending reviews data
  const pendingReviews = [
    { id: 1, partId: 'EL-PCB-100054', partName: 'Main Board PCB', requestedBy: 'John Doe', requestedAt: '2025-03-01T14:30:00Z' },
    { id: 2, partId: 'ME-3DM-100076', partName: 'Enclosure Model', requestedBy: 'Jane Smith', requestedAt: '2025-02-28T10:15:00Z' },
  ];

  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-bold text-gray-800 dark:text-white">Dashboard</h1>

      {/* Repository Section */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <h2 className="text-lg font-semibold text-gray-800 dark:text-white mb-4">Repository</h2>
        
        {error && (
          <div className="mb-4 p-3 bg-red-100 dark:bg-red-900 text-red-700 dark:text-red-200 rounded-md">
            {error}
          </div>
        )}
        
        {isLoading ? (
          <div className="flex justify-center items-center h-24">
            <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500"></div>
          </div>
        ) : repository ? (
          <div className="space-y-4">
            <div className="flex flex-col md:flex-row md:justify-between md:items-center">
              <div>
                <h3 className="text-md font-medium text-gray-800 dark:text-white">{repository.name}</h3>
                <p className="text-sm text-gray-500 dark:text-gray-400">{repository.path}</p>
              </div>
              <div className="mt-2 md:mt-0">
                <span className={`inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium ${
                  repository.hasChanges
                    ? 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200'
                    : 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200'
                }`}>
                  {repository.hasChanges ? 'Uncommitted Changes' : 'Clean'}
                </span>
                <span className="ml-2 inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200">
                  {repository.currentBranch}
                </span>
                {repository.lfsEnabled && (
                  <span className="ml-2 inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-200">
                    LFS Enabled
                  </span>
                )}
              </div>
            </div>
            <div className="flex justify-end">
              <button
                onClick={closeRepository}
                className="px-3 py-1 text-sm text-red-600 dark:text-red-400 hover:text-red-800 dark:hover:text-red-300"
              >
                Close Repository
              </button>
            </div>
          </div>
        ) : (
          <div className="space-y-4">
            <p className="text-sm text-gray-500 dark:text-gray-400">
              No repository is currently open. Create a new repository or open an existing one.
            </p>
            <div className="flex flex-col sm:flex-row space-y-2 sm:space-y-0 sm:space-x-2">
              <button
                onClick={() => setShowCreateModal(true)}
                className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
              >
                Create New Repository
              </button>
              <button
                onClick={() => openRepository()}
                className="px-4 py-2 bg-gray-200 text-gray-800 dark:bg-gray-700 dark:text-gray-200 rounded-md hover:bg-gray-300 dark:hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2"
              >
                Open Existing Repository
              </button>
            </div>
          </div>
        )}
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Recent Activity */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
          <h2 className="text-lg font-semibold text-gray-800 dark:text-white mb-4">Recent Activity</h2>
          <div className="space-y-3">
            {recentActivity.map((activity) => (
              <div key={activity.id} className="flex items-start border-b border-gray-200 dark:border-gray-700 pb-3">
                <div className="flex-1">
                  <p className="text-sm text-gray-800 dark:text-white">
                    {activity.type === 'part_updated' && `Part ${activity.partId} (${activity.partName}) was updated`}
                    {activity.type === 'part_created' && `Part ${activity.partId} (${activity.partName}) was created`}
                    {activity.type === 'part_reviewed' && `Part ${activity.partId} (${activity.partName}) was reviewed`}
                    {activity.type === 'workspace_created' && `Workspace "${activity.workspaceName}" was created`}
                    {activity.type === 'part_released' && `Part ${activity.partId} (${activity.partName}) was released`}
                  </p>
                  <p className="text-xs text-gray-500 dark:text-gray-400">
                    {new Date(activity.timestamp).toLocaleString()}
                  </p>
                </div>
              </div>
            ))}
          </div>
          <div className="mt-4">
            <Link to="/activity" className="text-sm text-blue-600 dark:text-blue-400 hover:underline">
              View all activity
            </Link>
          </div>
        </div>

        {/* My Workspaces */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
          <h2 className="text-lg font-semibold text-gray-800 dark:text-white mb-4">My Workspaces</h2>
          <div className="space-y-3">
            {recentWorkspaces.length > 0 ? (
              recentWorkspaces.map((workspace) => (
                <div key={workspace.id} className="flex items-start border-b border-gray-200 dark:border-gray-700 pb-3">
                  <div className="flex-1">
                    <Link
                      to={`/workspaces/${workspace.id}`}
                      className="text-sm font-medium text-gray-800 dark:text-white hover:text-blue-600 dark:hover:text-blue-400"
                    >
                      {workspace.name}
                    </Link>
                    <p className="text-xs text-gray-500 dark:text-gray-400">
                      {workspace.parts.length} parts • Last updated {new Date(workspace.updatedAt).toLocaleDateString()}
                    </p>
                  </div>
                </div>
              ))
            ) : (
              <p className="text-sm text-gray-500 dark:text-gray-400">No workspaces found</p>
            )}
          </div>
          <div className="mt-4 flex justify-between">
            <Link to="/workspaces" className="text-sm text-blue-600 dark:text-blue-400 hover:underline">
              View all workspaces
            </Link>
            <Link to="/workspaces/new" className="text-sm text-green-600 dark:text-green-400 hover:underline">
              Create workspace
            </Link>
          </div>
        </div>

        {/* Pending Reviews */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
          <h2 className="text-lg font-semibold text-gray-800 dark:text-white mb-4">Pending Reviews</h2>
          <div className="space-y-3">
            {pendingReviews.length > 0 ? (
              pendingReviews.map((review) => (
                <div key={review.id} className="flex items-start border-b border-gray-200 dark:border-gray-700 pb-3">
                  <div className="flex-1">
                    <Link
                      to={`/reviews/${review.id}`}
                      className="text-sm font-medium text-gray-800 dark:text-white hover:text-blue-600 dark:hover:text-blue-400"
                    >
                      {review.partId} ({review.partName})
                    </Link>
                    <p className="text-xs text-gray-500 dark:text-gray-400">
                      Requested by {review.requestedBy} on {new Date(review.requestedAt).toLocaleDateString()}
                    </p>
                  </div>
                </div>
              ))
            ) : (
              <p className="text-sm text-gray-500 dark:text-gray-400">No pending reviews</p>
            )}
          </div>
          <div className="mt-4">
            <Link to="/reviews" className="text-sm text-blue-600 dark:text-blue-400 hover:underline">
              View all reviews
            </Link>
          </div>
        </div>

        {/* Quick Actions */}
        <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
          <h2 className="text-lg font-semibold text-gray-800 dark:text-white mb-4">Quick Actions</h2>
          <div className="grid grid-cols-2 gap-4">
            <Link
              to="/parts/new"
              className="flex items-center justify-center p-4 bg-blue-50 dark:bg-blue-900 rounded-lg text-blue-700 dark:text-blue-200 hover:bg-blue-100 dark:hover:bg-blue-800 transition-colors"
            >
              <div className="text-center">
                <span className="block text-2xl mb-1">➕</span>
                <span className="text-sm font-medium">Create Part</span>
              </div>
            </Link>
            <Link
              to="/workspaces/new"
              className="flex items-center justify-center p-4 bg-green-50 dark:bg-green-900 rounded-lg text-green-700 dark:text-green-200 hover:bg-green-100 dark:hover:bg-green-800 transition-colors"
            >
              <div className="text-center">
                <span className="block text-2xl mb-1">📁</span>
                <span className="text-sm font-medium">Create Workspace</span>
              </div>
            </Link>
            <Link
              to="/parts"
              className="flex items-center justify-center p-4 bg-purple-50 dark:bg-purple-900 rounded-lg text-purple-700 dark:text-purple-200 hover:bg-purple-100 dark:hover:bg-purple-800 transition-colors"
            >
              <div className="text-center">
                <span className="block text-2xl mb-1">🔍</span>
                <span className="text-sm font-medium">Search Parts</span>
              </div>
            </Link>
            <Link
              to="/settings"
              className="flex items-center justify-center p-4 bg-gray-50 dark:bg-gray-700 rounded-lg text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-600 transition-colors"
            >
              <div className="text-center">
                <span className="block text-2xl mb-1">⚙️</span>
                <span className="text-sm font-medium">Settings</span>
              </div>
            </Link>
          </div>
        </div>
      </div>

      {/* Create Repository Modal */}
      {showCreateModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-50">
          <div className="bg-white dark:bg-gray-800 rounded-lg shadow-lg max-w-md w-full p-6">
            <h3 className="text-lg font-semibold text-gray-800 dark:text-white mb-4">Create New Repository</h3>
            
            <div className="space-y-4">
              <div>
                <label htmlFor="repoName" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                  Repository Name
                </label>
                <input
                  type="text"
                  id="repoName"
                  value={repoName}
                  onChange={(e) => setRepoName(e.target.value)}
                  className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                  placeholder="my-plm-repository"
                />
              </div>
              
              <div>
                <label htmlFor="repoPath" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                  Parent Directory
                </label>
                <div className="mt-1 flex rounded-md shadow-sm">
                  <input
                    type="text"
                    id="repoPath"
                    value={repoPath}
                    readOnly
                    className="block w-full rounded-l-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                    placeholder="Select a directory..."
                  />
                  <button
                    type="button"
                    onClick={handleSelectPath}
                    className="inline-flex items-center px-3 py-2 border border-l-0 border-gray-300 dark:border-gray-600 rounded-r-md bg-gray-50 dark:bg-gray-600 text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-500"
                  >
                    Browse
                  </button>
                </div>
              </div>
              
              <div>
                <label htmlFor="templateType" className="block text-sm font-medium text-gray-700 dark:text-gray-300">
                  Directory Template
                </label>
                <select
                  id="templateType"
                  value={selectedTemplate}
                  onChange={(e) => selectDirectoryTemplate(e.target.value)}
                  className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-blue-500 focus:ring-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:text-white"
                >
                  <option value="minimal">Minimal - Essential directories only</option>
                  <option value="standard">Standard - Commonly used directories (Default)</option>
                  <option value="extended">Extended - All possible directories</option>
                </select>
              </div>
            </div>
            
            <div className="mt-6 flex justify-end space-x-3">
              <button
                type="button"
                onClick={() => setShowCreateModal(false)}
                className="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-gray-700 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2"
              >
                Cancel
              </button>
              <button
                type="button"
                onClick={handleCreateRepository}
                disabled={!repoPath || !repoName}
                className={`px-4 py-2 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 ${
                  !repoPath || !repoName
                    ? 'bg-blue-400 cursor-not-allowed'
                    : 'bg-blue-600 hover:bg-blue-700'
                }`}
              >
                Create
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default Dashboard;