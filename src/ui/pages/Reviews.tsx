import React, { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { useParts, Part, PartStatus } from '../context/PartsContext';
import StatusBadge from '../components/StatusBadge';
import SearchBar from '../components/SearchBar';

/**
 * Reviews page component
 * Displays a list of parts in review status
 */
const Reviews: React.FC = () => {
  const { parts, loading, error, fetchParts, changePartStatus } = useParts();
  const [filteredParts, setFilteredParts] = useState<Part[]>([]);
  const [searchQuery, setSearchQuery] = useState('');

  // Fetch parts on component mount
  useEffect(() => {
    fetchParts();
  }, [fetchParts]);

  // Filter parts to show only those in review status
  useEffect(() => {
    let result = parts.filter((part) => part.status === 'In Review');

    // Apply search filter if there's a query
    if (searchQuery) {
      const query = searchQuery.toLowerCase();
      result = result.filter(
        (part) =>
          part.displayId.toLowerCase().includes(query) ||
          part.name.toLowerCase().includes(query) ||
          part.description.toLowerCase().includes(query)
      );
    }

    // Sort by updated date (newest first)
    result.sort((a, b) => new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime());

    setFilteredParts(result);
  }, [parts, searchQuery]);

  // Handle search input change
  const handleSearchChange = (query: string) => {
    setSearchQuery(query);
  };

  // Handle approving a part
  const handleApprove = async (partId: number) => {
    try {
      await changePartStatus(partId, 'Released');
      // Refresh the parts list
      fetchParts();
    } catch (error) {
      console.error('Failed to approve part:', error);
    }
  };

  // Handle rejecting a part
  const handleReject = async (partId: number) => {
    try {
      await changePartStatus(partId, 'Draft');
      // Refresh the parts list
      fetchParts();
    } catch (error) {
      console.error('Failed to reject part:', error);
    }
  };

  return (
    <div className="space-y-6">
      <h1 className="text-2xl font-bold text-gray-800 dark:text-white">Reviews</h1>

      {/* Search */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
        <SearchBar onSearch={handleSearchChange} placeholder="Search reviews..." />
      </div>

      {/* Reviews List */}
      {loading ? (
        <div className="text-center py-8">
          <p className="text-gray-600 dark:text-gray-400">Loading reviews...</p>
        </div>
      ) : error ? (
        <div className="bg-red-50 dark:bg-red-900 text-red-700 dark:text-red-200 p-4 rounded-md">
          <p>{error}</p>
        </div>
      ) : filteredParts.length === 0 ? (
        <div className="text-center py-8">
          <p className="text-gray-600 dark:text-gray-400">No parts in review</p>
          <p className="text-sm text-gray-500 dark:text-gray-500 mt-2">
            When parts are submitted for review, they will appear here.
          </p>
        </div>
      ) : (
        <div className="space-y-4">
          {filteredParts.map((part) => (
            <div
              key={part.id}
              className="bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden"
            >
              <div className="p-4">
                <div className="flex justify-between items-start">
                  <div>
                    <Link
                      to={`/parts/${part.id}`}
                      className="text-lg font-medium text-gray-900 dark:text-white hover:text-blue-600 dark:hover:text-blue-400"
                    >
                      {part.name}
                    </Link>
                    <p className="text-sm text-gray-500 dark:text-gray-400">{part.displayId}</p>
                  </div>
                  <StatusBadge status={part.status} />
                </div>

                <p className="mt-2 text-sm text-gray-600 dark:text-gray-300">
                  {part.description || 'No description provided'}
                </p>

                <div className="mt-4 flex justify-between items-center">
                  <div className="text-xs text-gray-500 dark:text-gray-400">
                    <span className="block">Category: {part.category}</span>
                    <span className="block">Subcategory: {part.subcategory}</span>
                  </div>
                  <div className="text-xs text-gray-500 dark:text-gray-400 text-right">
                    <span className="block">Submitted: {new Date(part.updatedAt).toLocaleDateString()}</span>
                  </div>
                </div>
              </div>

              <div className="border-t border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-750 px-4 py-3 flex justify-between">
                <div className="flex space-x-2">
                  <Link
                    to={`/parts/${part.id}`}
                    className="px-3 py-1 bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-white rounded hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors"
                  >
                    View Details
                  </Link>
                </div>
                <div className="flex space-x-2">
                  <button
                    onClick={() => handleReject(part.id)}
                    className="px-3 py-1 bg-yellow-600 text-white rounded hover:bg-yellow-700 transition-colors"
                  >
                    Return to Draft
                  </button>
                  <button
                    onClick={() => handleApprove(part.id)}
                    className="px-3 py-1 bg-green-600 text-white rounded hover:bg-green-700 transition-colors"
                  >
                    Approve
                  </button>
                </div>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
};

export default Reviews;