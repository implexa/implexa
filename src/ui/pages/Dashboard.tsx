import React, { useEffect } from 'react';
import { Link } from 'react-router-dom';
import { useParts } from '../context/PartsContext';
import { useWorkspace } from '../context/WorkspaceContext';

/**
 * Dashboard page component
 * Displays an overview of recent activity and quick access to common tasks
 */
const Dashboard: React.FC = () => {
  const { parts, fetchParts } = useParts();
  const { workspaces, fetchWorkspaces } = useWorkspace();

  // Fetch data on component mount
  useEffect(() => {
    fetchParts();
    fetchWorkspaces();
  }, [fetchParts, fetchWorkspaces]);

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
    </div>
  );
};

export default Dashboard;