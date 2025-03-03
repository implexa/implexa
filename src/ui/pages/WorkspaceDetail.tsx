import React, { useEffect, useState } from 'react';
import { useParams, Link, useNavigate } from 'react-router-dom';
import { useWorkspace, Workspace } from '../context/WorkspaceContext';
import { useParts } from '../context/PartsContext';
import { useNotifications } from '../context/NotificationContext';
import PartCard from '../components/PartCard';

/**
 * WorkspaceDetail page component
 * Displays detailed information about a workspace
 */
const WorkspaceDetail: React.FC = () => {
  const { workspaceId } = useParams<{ workspaceId: string }>();
  const navigate = useNavigate();
  const { getWorkspace, updateWorkspace, deleteWorkspace, switchWorkspace } = useWorkspace();
  const { parts } = useParts();
  const { addNotification } = useNotifications();
  const [workspace, setWorkspace] = useState<Workspace | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState('overview');

  // Fetch workspace data on component mount
  useEffect(() => {
    const fetchWorkspaceData = async () => {
      if (!workspaceId) {
        setError('Workspace ID is missing');
        setLoading(false);
        return;
      }

      try {
        const workspaceData = await getWorkspace(workspaceId);
        if (workspaceData) {
          setWorkspace(workspaceData);
          // Switch to this workspace
          await switchWorkspace(workspaceId);
        } else {
          setError('Workspace not found');
        }
      } catch (err) {
        setError('Failed to load workspace data');
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    fetchWorkspaceData();
  }, [workspaceId, getWorkspace, switchWorkspace]);

  // Handle workspace deletion
  const handleDelete = async () => {
    if (!workspace) return;

    if (window.confirm(`Are you sure you want to delete workspace "${workspace.name}"?`)) {
      try {
        const success = await deleteWorkspace(workspace.id);
        if (success) {
          addNotification({
            type: 'success',
            message: 'Workspace deleted successfully',
            title: 'Workspace Deleted',
          });
          navigate('/workspaces');
        }
      } catch (err) {
        addNotification({
          type: 'error',
          message: `Failed to delete workspace: ${err instanceof Error ? err.message : 'Unknown error'}`,
          title: 'Error',
        });
      }
    }
  };

  // Handle opening workspace in CAD
  const handleOpenInCAD = () => {
    if (!workspace) return;

    // This would be implemented to open the workspace in CAD
    addNotification({
      type: 'info',
      message: `Opening workspace "${workspace.name}" in CAD...`,
      title: 'Opening in CAD',
    });
  };

  if (loading) {
    return (
      <div className="flex justify-center items-center h-64">
        <p className="text-gray-500 dark:text-gray-400">Loading workspace data...</p>
      </div>
    );
  }

  if (error || !workspace) {
    return (
      <div className="bg-red-50 dark:bg-red-900 text-red-700 dark:text-red-200 p-4 rounded-md">
        <h2 className="text-lg font-medium">Error</h2>
        <p>{error || 'Failed to load workspace data'}</p>
        <Link to="/workspaces" className="text-red-700 dark:text-red-200 underline mt-2 inline-block">
          Return to Workspaces List
        </Link>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Workspace Header */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <div className="flex flex-col md:flex-row md:justify-between md:items-center">
          <div>
            <h1 className="text-2xl font-bold text-gray-900 dark:text-white">{workspace.name}</h1>
            <p className="text-gray-500 dark:text-gray-400">
              Created {new Date(workspace.createdAt).toLocaleDateString()}
            </p>
          </div>
          <div className="mt-4 md:mt-0 flex items-center space-x-4">
            <button
              onClick={handleOpenInCAD}
              className="px-3 py-1 bg-green-600 text-white rounded hover:bg-green-700 transition-colors"
            >
              Open in CAD
            </button>
            <Link
              to={`/workspaces/${workspace.id}/edit`}
              className="px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
            >
              Edit
            </Link>
            <button
              onClick={handleDelete}
              className="px-3 py-1 bg-red-600 text-white rounded hover:bg-red-700 transition-colors"
            >
              Delete
            </button>
          </div>
        </div>
      </div>

      {/* Navigation Tabs */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow">
        <div className="border-b border-gray-200 dark:border-gray-700">
          <nav className="flex -mb-px">
            <button
              onClick={() => setActiveTab('overview')}
              className={`py-4 px-6 text-sm font-medium ${
                activeTab === 'overview'
                  ? 'border-b-2 border-blue-500 text-blue-600 dark:text-blue-400'
                  : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
              }`}
            >
              Overview
            </button>
            <button
              onClick={() => setActiveTab('parts')}
              className={`py-4 px-6 text-sm font-medium ${
                activeTab === 'parts'
                  ? 'border-b-2 border-blue-500 text-blue-600 dark:text-blue-400'
                  : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
              }`}
            >
              Parts
            </button>
            <button
              onClick={() => setActiveTab('files')}
              className={`py-4 px-6 text-sm font-medium ${
                activeTab === 'files'
                  ? 'border-b-2 border-blue-500 text-blue-600 dark:text-blue-400'
                  : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
              }`}
            >
              Files
            </button>
            <button
              onClick={() => setActiveTab('history')}
              className={`py-4 px-6 text-sm font-medium ${
                activeTab === 'history'
                  ? 'border-b-2 border-blue-500 text-blue-600 dark:text-blue-400'
                  : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
              }`}
            >
              History
            </button>
          </nav>
        </div>

        {/* Tab Content */}
        <div className="p-6">
          {activeTab === 'overview' && (
            <div className="space-y-6">
              <div>
                <h2 className="text-lg font-medium text-gray-900 dark:text-white">Description</h2>
                <p className="mt-2 text-gray-600 dark:text-gray-300">
                  {workspace.description || 'No description provided'}
                </p>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div>
                  <h2 className="text-lg font-medium text-gray-900 dark:text-white">Details</h2>
                  <div className="mt-2 space-y-2">
                    <div className="flex justify-between">
                      <span className="text-gray-500 dark:text-gray-400">Parts:</span>
                      <span className="text-gray-900 dark:text-white">{workspace.parts.length}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-500 dark:text-gray-400">Created:</span>
                      <span className="text-gray-900 dark:text-white">
                        {new Date(workspace.createdAt).toLocaleString()}
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-500 dark:text-gray-400">Last Updated:</span>
                      <span className="text-gray-900 dark:text-white">
                        {new Date(workspace.updatedAt).toLocaleString()}
                      </span>
                    </div>
                  </div>
                </div>

                <div>
                  <h2 className="text-lg font-medium text-gray-900 dark:text-white">Actions</h2>
                  <div className="mt-2 space-y-2">
                    <button
                      onClick={handleOpenInCAD}
                      className="w-full py-2 px-4 bg-green-600 text-white rounded hover:bg-green-700 transition-colors"
                    >
                      Open in CAD
                    </button>
                    <Link
                      to={`/workspaces/${workspace.id}/edit`}
                      className="block w-full py-2 px-4 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors text-center"
                    >
                      Edit Workspace
                    </Link>
                    <button
                      onClick={handleDelete}
                      className="w-full py-2 px-4 bg-red-600 text-white rounded hover:bg-red-700 transition-colors"
                    >
                      Delete Workspace
                    </button>
                  </div>
                </div>
              </div>
            </div>
          )}

          {activeTab === 'parts' && (
            <div>
              <div className="flex justify-between items-center">
                <h2 className="text-lg font-medium text-gray-900 dark:text-white">Parts</h2>
                <button className="px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">
                  Add Part
                </button>
              </div>

              {workspace.parts.length === 0 ? (
                <p className="mt-4 text-gray-600 dark:text-gray-300">No parts in this workspace.</p>
              ) : (
                <div className="mt-4 grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                  {workspace.parts.map((part) => (
                    <PartCard key={part.id} part={part} />
                  ))}
                </div>
              )}
            </div>
          )}

          {activeTab === 'files' && (
            <div>
              <div className="flex justify-between items-center">
                <h2 className="text-lg font-medium text-gray-900 dark:text-white">Files</h2>
                <button className="px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">
                  Add Files
                </button>
              </div>
              <p className="mt-4 text-gray-600 dark:text-gray-300">No files in this workspace.</p>
            </div>
          )}

          {activeTab === 'history' && (
            <div>
              <h2 className="text-lg font-medium text-gray-900 dark:text-white">History</h2>
              <div className="mt-4 space-y-4">
                <div className="border-l-2 border-blue-500 pl-4 pb-4">
                  <p className="text-sm text-gray-500 dark:text-gray-400">
                    {new Date(workspace.createdAt).toLocaleString()}
                  </p>
                  <p className="text-gray-900 dark:text-white">Workspace created</p>
                </div>
                <div className="border-l-2 border-blue-500 pl-4 pb-4">
                  <p className="text-sm text-gray-500 dark:text-gray-400">
                    {new Date(workspace.updatedAt).toLocaleString()}
                  </p>
                  <p className="text-gray-900 dark:text-white">Workspace updated</p>
                </div>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default WorkspaceDetail;