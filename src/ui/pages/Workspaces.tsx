import React, { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { useWorkspace, Workspace } from '../context/WorkspaceContext';
import SearchBar from '../components/SearchBar';

/**
 * Workspaces page component
 * Displays a list of workspaces
 */
const Workspaces: React.FC = () => {
  const { workspaces, loading, error, fetchWorkspaces } = useWorkspace();
  const [filteredWorkspaces, setFilteredWorkspaces] = useState<Workspace[]>([]);
  const [searchQuery, setSearchQuery] = useState('');

  // Fetch workspaces on component mount
  useEffect(() => {
    fetchWorkspaces();
  }, [fetchWorkspaces]);

  // Filter workspaces when workspaces or search query changes
  useEffect(() => {
    if (searchQuery) {
      const query = searchQuery.toLowerCase();
      setFilteredWorkspaces(
        workspaces.filter(
          (workspace) =>
            workspace.name.toLowerCase().includes(query) ||
            workspace.description.toLowerCase().includes(query)
        )
      );
    } else {
      setFilteredWorkspaces(workspaces);
    }
  }, [workspaces, searchQuery]);

  // Handle search input change
  const handleSearchChange = (query: string) => {
    setSearchQuery(query);
  };

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-bold text-gray-800 dark:text-white">Workspaces</h1>
        <Link
          to="/workspaces/new"
          className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
        >
          Create Workspace
        </Link>
      </div>

      {/* Search */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
        <SearchBar onSearch={handleSearchChange} placeholder="Search workspaces..." />
      </div>

      {/* Workspaces List */}
      {loading ? (
        <div className="text-center py-8">
          <p className="text-gray-600 dark:text-gray-400">Loading workspaces...</p>
        </div>
      ) : error ? (
        <div className="bg-red-50 dark:bg-red-900 text-red-700 dark:text-red-200 p-4 rounded-md">
          <p>{error}</p>
        </div>
      ) : filteredWorkspaces.length === 0 ? (
        <div className="text-center py-8">
          <p className="text-gray-600 dark:text-gray-400">No workspaces found</p>
          <p className="text-sm text-gray-500 dark:text-gray-500 mt-2">
            Try adjusting your search or{' '}
            <Link to="/workspaces/new" className="text-blue-600 dark:text-blue-400 hover:underline">
              create a new workspace
            </Link>
          </p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {filteredWorkspaces.map((workspace) => (
            <div
              key={workspace.id}
              className="bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden hover:shadow-md transition-shadow"
            >
              <div className="p-4">
                <Link
                  to={`/workspaces/${workspace.id}`}
                  className="text-lg font-medium text-gray-900 dark:text-white hover:text-blue-600 dark:hover:text-blue-400"
                >
                  {workspace.name}
                </Link>
                <p className="mt-2 text-sm text-gray-600 dark:text-gray-300 line-clamp-2">
                  {workspace.description || 'No description provided'}
                </p>
                <div className="mt-4 flex justify-between items-center">
                  <div className="text-xs text-gray-500 dark:text-gray-400">
                    <span className="block">{workspace.parts.length} parts</span>
                    <span className="block">
                      Created: {new Date(workspace.createdAt).toLocaleDateString()}
                    </span>
                  </div>
                  <div className="text-xs text-gray-500 dark:text-gray-400 text-right">
                    <span className="block">
                      Updated: {new Date(workspace.updatedAt).toLocaleDateString()}
                    </span>
                  </div>
                </div>
              </div>
              <div className="border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-750 px-4 py-3 flex justify-between">
                <Link
                  to={`/workspaces/${workspace.id}`}
                  className="text-sm text-blue-600 dark:text-blue-400 hover:underline"
                >
                  View Details
                </Link>
                <button
                  className="text-sm text-gray-600 dark:text-gray-400 hover:underline"
                  onClick={() => {
                    // This would be implemented to open the workspace in CAD
                    alert(`Open workspace ${workspace.name} in CAD`);
                  }}
                >
                  Open in CAD
                </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default Workspaces;