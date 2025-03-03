import React, { useEffect, useState } from 'react';
import { useParams, Link, useNavigate } from 'react-router-dom';
import { useParts, Part, PartStatus } from '../context/PartsContext';
import { useNotifications } from '../context/NotificationContext';
import StatusBadge from '../components/StatusBadge';

/**
 * PartDetail page component
 * Displays detailed information about a part
 */
const PartDetail: React.FC = () => {
  const { partId } = useParams<{ partId: string }>();
  const navigate = useNavigate();
  const { getPart, changePartStatus, deletePart } = useParts();
  const { addNotification } = useNotifications();
  const [part, setPart] = useState<Part | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [activeTab, setActiveTab] = useState('overview');

  // Fetch part data on component mount
  useEffect(() => {
    const fetchPartData = async () => {
      if (!partId) {
        setError('Part ID is missing');
        setLoading(false);
        return;
      }

      try {
        const partData = await getPart(parseInt(partId, 10));
        if (partData) {
          setPart(partData);
        } else {
          setError('Part not found');
        }
      } catch (err) {
        setError('Failed to load part data');
        console.error(err);
      } finally {
        setLoading(false);
      }
    };

    fetchPartData();
  }, [partId, getPart]);

  // Handle status change
  const handleStatusChange = async (newStatus: PartStatus) => {
    if (!part) return;

    try {
      const success = await changePartStatus(part.id, newStatus);
      if (success) {
        setPart({ ...part, status: newStatus });
        addNotification({
          type: 'success',
          message: `Part status changed to ${newStatus}`,
          title: 'Status Updated',
        });
      }
    } catch (err) {
      addNotification({
        type: 'error',
        message: `Failed to change status: ${err instanceof Error ? err.message : 'Unknown error'}`,
        title: 'Error',
      });
    }
  };

  // Handle part deletion
  const handleDelete = async () => {
    if (!part) return;

    if (window.confirm(`Are you sure you want to delete part ${part.displayId}?`)) {
      try {
        const success = await deletePart(part.id);
        if (success) {
          addNotification({
            type: 'success',
            message: 'Part deleted successfully',
            title: 'Part Deleted',
          });
          navigate('/parts');
        }
      } catch (err) {
        addNotification({
          type: 'error',
          message: `Failed to delete part: ${err instanceof Error ? err.message : 'Unknown error'}`,
          title: 'Error',
        });
      }
    }
  };

  if (loading) {
    return (
      <div className="flex justify-center items-center h-64">
        <p className="text-gray-500 dark:text-gray-400">Loading part data...</p>
      </div>
    );
  }

  if (error || !part) {
    return (
      <div className="bg-red-50 dark:bg-red-900 text-red-700 dark:text-red-200 p-4 rounded-md">
        <h2 className="text-lg font-medium">Error</h2>
        <p>{error || 'Failed to load part data'}</p>
        <Link to="/parts" className="text-red-700 dark:text-red-200 underline mt-2 inline-block">
          Return to Parts List
        </Link>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Part Header */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-6">
        <div className="flex flex-col md:flex-row md:justify-between md:items-center">
          <div>
            <h1 className="text-2xl font-bold text-gray-900 dark:text-white">{part.name}</h1>
            <p className="text-gray-500 dark:text-gray-400">{part.displayId}</p>
          </div>
          <div className="mt-4 md:mt-0 flex items-center space-x-4">
            <StatusBadge status={part.status} />
            <div className="flex space-x-2">
              <Link
                to={`/parts/${part.id}/edit`}
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
              onClick={() => setActiveTab('properties')}
              className={`py-4 px-6 text-sm font-medium ${
                activeTab === 'properties'
                  ? 'border-b-2 border-blue-500 text-blue-600 dark:text-blue-400'
                  : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
              }`}
            >
              Properties
            </button>
            <button
              onClick={() => setActiveTab('relationships')}
              className={`py-4 px-6 text-sm font-medium ${
                activeTab === 'relationships'
                  ? 'border-b-2 border-blue-500 text-blue-600 dark:text-blue-400'
                  : 'text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300'
              }`}
            >
              Relationships
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
                  {part.description || 'No description provided'}
                </p>
              </div>

              <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div>
                  <h2 className="text-lg font-medium text-gray-900 dark:text-white">Details</h2>
                  <div className="mt-2 space-y-2">
                    <div className="flex justify-between">
                      <span className="text-gray-500 dark:text-gray-400">Category:</span>
                      <span className="text-gray-900 dark:text-white">{part.category}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-500 dark:text-gray-400">Subcategory:</span>
                      <span className="text-gray-900 dark:text-white">{part.subcategory}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-500 dark:text-gray-400">Status:</span>
                      <StatusBadge status={part.status} />
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-500 dark:text-gray-400">Created:</span>
                      <span className="text-gray-900 dark:text-white">
                        {new Date(part.createdAt).toLocaleString()}
                      </span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-500 dark:text-gray-400">Last Updated:</span>
                      <span className="text-gray-900 dark:text-white">
                        {new Date(part.updatedAt).toLocaleString()}
                      </span>
                    </div>
                  </div>
                </div>

                <div>
                  <h2 className="text-lg font-medium text-gray-900 dark:text-white">Actions</h2>
                  <div className="mt-2 space-y-2">
                    {part.status === 'Draft' && (
                      <button
                        onClick={() => handleStatusChange('In Review')}
                        className="w-full py-2 px-4 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
                      >
                        Submit for Review
                      </button>
                    )}
                    {part.status === 'In Review' && (
                      <>
                        <button
                          onClick={() => handleStatusChange('Released')}
                          className="w-full py-2 px-4 bg-green-600 text-white rounded hover:bg-green-700 transition-colors"
                        >
                          Approve and Release
                        </button>
                        <button
                          onClick={() => handleStatusChange('Draft')}
                          className="w-full py-2 px-4 bg-yellow-600 text-white rounded hover:bg-yellow-700 transition-colors"
                        >
                          Return to Draft
                        </button>
                      </>
                    )}
                    {part.status === 'Released' && (
                      <button
                        onClick={() => handleStatusChange('Obsolete')}
                        className="w-full py-2 px-4 bg-gray-600 text-white rounded hover:bg-gray-700 transition-colors"
                      >
                        Mark as Obsolete
                      </button>
                    )}
                    <Link
                      to={`/parts/${part.id}/edit`}
                      className="block w-full py-2 px-4 bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-white rounded hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors text-center"
                    >
                      Edit Part
                    </Link>
                  </div>
                </div>
              </div>
            </div>
          )}

          {activeTab === 'files' && (
            <div>
              <h2 className="text-lg font-medium text-gray-900 dark:text-white">Files</h2>
              <p className="mt-2 text-gray-600 dark:text-gray-300">No files attached to this part.</p>
              <button className="mt-4 px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">
                Add Files
              </button>
            </div>
          )}

          {activeTab === 'properties' && (
            <div>
              <div className="flex justify-between items-center">
                <h2 className="text-lg font-medium text-gray-900 dark:text-white">Properties</h2>
                <button className="px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">
                  Add Property
                </button>
              </div>
              {Object.keys(part.properties).length > 0 ? (
                <div className="mt-4 border border-gray-200 dark:border-gray-700 rounded-md overflow-hidden">
                  <table className="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
                    <thead className="bg-gray-50 dark:bg-gray-800">
                      <tr>
                        <th
                          scope="col"
                          className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider"
                        >
                          Property
                        </th>
                        <th
                          scope="col"
                          className="px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider"
                        >
                          Value
                        </th>
                        <th scope="col" className="relative px-6 py-3">
                          <span className="sr-only">Actions</span>
                        </th>
                      </tr>
                    </thead>
                    <tbody className="bg-white dark:bg-gray-900 divide-y divide-gray-200 dark:divide-gray-700">
                      {Object.entries(part.properties).map(([key, value]) => (
                        <tr key={key}>
                          <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-white">
                            {key}
                          </td>
                          <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500 dark:text-gray-400">
                            {value}
                          </td>
                          <td className="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
                            <button className="text-blue-600 dark:text-blue-400 hover:text-blue-900 dark:hover:text-blue-300">
                              Edit
                            </button>
                            <button className="ml-4 text-red-600 dark:text-red-400 hover:text-red-900 dark:hover:text-red-300">
                              Delete
                            </button>
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              ) : (
                <p className="mt-2 text-gray-600 dark:text-gray-300">No properties defined for this part.</p>
              )}
            </div>
          )}

          {activeTab === 'relationships' && (
            <div>
              <div className="flex justify-between items-center">
                <h2 className="text-lg font-medium text-gray-900 dark:text-white">Relationships</h2>
                <button className="px-3 py-1 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">
                  Add Relationship
                </button>
              </div>
              <p className="mt-2 text-gray-600 dark:text-gray-300">No relationships defined for this part.</p>
            </div>
          )}

          {activeTab === 'history' && (
            <div>
              <h2 className="text-lg font-medium text-gray-900 dark:text-white">History</h2>
              <div className="mt-4 space-y-4">
                <div className="border-l-2 border-blue-500 pl-4 pb-4">
                  <p className="text-sm text-gray-500 dark:text-gray-400">
                    {new Date(part.createdAt).toLocaleString()}
                  </p>
                  <p className="text-gray-900 dark:text-white">Part created</p>
                </div>
                <div className="border-l-2 border-blue-500 pl-4 pb-4">
                  <p className="text-sm text-gray-500 dark:text-gray-400">
                    {new Date(part.updatedAt).toLocaleString()}
                  </p>
                  <p className="text-gray-900 dark:text-white">Part updated</p>
                </div>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default PartDetail;