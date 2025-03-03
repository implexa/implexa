import React, { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { useParts, Part } from '../context/PartsContext';
import PartCard from '../components/PartCard';
import SearchBar from '../components/SearchBar';

/**
 * Parts page component
 * Displays a list of parts with filtering and sorting capabilities
 */
const Parts: React.FC = () => {
  const { parts, loading, error, fetchParts } = useParts();
  const [filteredParts, setFilteredParts] = useState<Part[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [categoryFilter, setCategoryFilter] = useState<string>('');
  const [statusFilter, setStatusFilter] = useState<string>('');
  const [sortBy, setSortBy] = useState<string>('updatedAt');
  const [sortDirection, setSortDirection] = useState<'asc' | 'desc'>('desc');

  // Fetch parts on component mount
  useEffect(() => {
    fetchParts();
  }, [fetchParts]);

  // Filter and sort parts when parts, filters, or sort options change
  useEffect(() => {
    let result = [...parts];

    // Apply search filter
    if (searchQuery) {
      const query = searchQuery.toLowerCase();
      result = result.filter(
        (part) =>
          part.displayId.toLowerCase().includes(query) ||
          part.name.toLowerCase().includes(query) ||
          part.description.toLowerCase().includes(query)
      );
    }

    // Apply category filter
    if (categoryFilter) {
      result = result.filter((part) => part.category === categoryFilter);
    }

    // Apply status filter
    if (statusFilter) {
      result = result.filter((part) => part.status === statusFilter);
    }

    // Apply sorting
    result.sort((a, b) => {
      let valueA: any = a[sortBy as keyof Part];
      let valueB: any = b[sortBy as keyof Part];

      // Handle date strings
      if (typeof valueA === 'string' && (valueA as string).includes('-')) {
        valueA = new Date(valueA).getTime();
        valueB = new Date(valueB).getTime();
      }

      if (sortDirection === 'asc') {
        return valueA > valueB ? 1 : -1;
      } else {
        return valueA < valueB ? 1 : -1;
      }
    });

    setFilteredParts(result);
  }, [parts, searchQuery, categoryFilter, statusFilter, sortBy, sortDirection]);

  // Get unique categories for filter dropdown
  const categories = Array.from(new Set(parts.map((part) => part.category)));

  // Handle search input change
  const handleSearchChange = (query: string) => {
    setSearchQuery(query);
  };

  // Toggle sort direction
  const handleSortChange = (field: string) => {
    if (sortBy === field) {
      setSortDirection(sortDirection === 'asc' ? 'desc' : 'asc');
    } else {
      setSortBy(field);
      setSortDirection('desc');
    }
  };

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h1 className="text-2xl font-bold text-gray-800 dark:text-white">Parts</h1>
        <Link
          to="/parts/new"
          className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors"
        >
          Create Part
        </Link>
      </div>

      {/* Search and Filters */}
      <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
        <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
          <div className="md:col-span-2">
            <SearchBar onSearch={handleSearchChange} placeholder="Search parts..." />
          </div>
          <div>
            <label htmlFor="category-filter" className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Category
            </label>
            <select
              id="category-filter"
              value={categoryFilter}
              onChange={(e) => setCategoryFilter(e.target.value)}
              className="w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-200 px-3 py-2"
            >
              <option value="">All Categories</option>
              {categories.map((category) => (
                <option key={category} value={category}>
                  {category}
                </option>
              ))}
            </select>
          </div>
          <div>
            <label htmlFor="status-filter" className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Status
            </label>
            <select
              id="status-filter"
              value={statusFilter}
              onChange={(e) => setStatusFilter(e.target.value)}
              className="w-full rounded-md border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-200 px-3 py-2"
            >
              <option value="">All Statuses</option>
              <option value="Draft">Draft</option>
              <option value="In Review">In Review</option>
              <option value="Released">Released</option>
              <option value="Obsolete">Obsolete</option>
            </select>
          </div>
        </div>
      </div>

      {/* Sort Controls */}
      <div className="flex justify-end space-x-4">
        <div className="flex items-center">
          <span className="text-sm text-gray-600 dark:text-gray-400 mr-2">Sort by:</span>
          <button
            onClick={() => handleSortChange('displayId')}
            className={`text-sm px-2 py-1 rounded ${
              sortBy === 'displayId' ? 'bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-200' : 'text-gray-700 dark:text-gray-300'
            }`}
          >
            Part Number {sortBy === 'displayId' && (sortDirection === 'asc' ? '↑' : '↓')}
          </button>
          <button
            onClick={() => handleSortChange('name')}
            className={`text-sm px-2 py-1 rounded ${
              sortBy === 'name' ? 'bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-200' : 'text-gray-700 dark:text-gray-300'
            }`}
          >
            Name {sortBy === 'name' && (sortDirection === 'asc' ? '↑' : '↓')}
          </button>
          <button
            onClick={() => handleSortChange('updatedAt')}
            className={`text-sm px-2 py-1 rounded ${
              sortBy === 'updatedAt' ? 'bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-200' : 'text-gray-700 dark:text-gray-300'
            }`}
          >
            Last Updated {sortBy === 'updatedAt' && (sortDirection === 'asc' ? '↑' : '↓')}
          </button>
        </div>
      </div>

      {/* Parts List */}
      {loading ? (
        <div className="text-center py-8">
          <p className="text-gray-600 dark:text-gray-400">Loading parts...</p>
        </div>
      ) : error ? (
        <div className="bg-red-50 dark:bg-red-900 text-red-700 dark:text-red-200 p-4 rounded-md">
          <p>{error}</p>
        </div>
      ) : filteredParts.length === 0 ? (
        <div className="text-center py-8">
          <p className="text-gray-600 dark:text-gray-400">No parts found</p>
          <p className="text-sm text-gray-500 dark:text-gray-500 mt-2">
            Try adjusting your search or filters, or{' '}
            <Link to="/parts/new" className="text-blue-600 dark:text-blue-400 hover:underline">
              create a new part
            </Link>
          </p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {filteredParts.map((part) => (
            <PartCard key={part.id} part={part} />
          ))}
        </div>
      )}

      {/* Pagination (simplified) */}
      {filteredParts.length > 0 && (
        <div className="flex justify-center mt-8">
          <nav className="flex items-center space-x-2">
            <button className="px-3 py-1 rounded border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 disabled:opacity-50">
              Previous
            </button>
            <span className="px-3 py-1 rounded bg-blue-100 dark:bg-blue-900 text-blue-700 dark:text-blue-200">1</span>
            <button className="px-3 py-1 rounded border border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 disabled:opacity-50">
              Next
            </button>
          </nav>
        </div>
      )}
    </div>
  );
};

export default Parts;