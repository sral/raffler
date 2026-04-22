import React from 'react';
import { API } from '../api.js';

/**
 * Custom hook for managing locations state
 * @returns {Object} Locations state and operations
 */
export function useLocations() {
  const [locations, setLocations] = React.useState([]);
  const [loading, setLoading] = React.useState(true);
  const [error, setError] = React.useState(null);

  const fetchLocations = React.useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      const fetchedLocations = await API.locations.getAll();
      setLocations(fetchedLocations);
    } catch (err) {
      setError(err.message);
      console.error('Failed to fetch locations:', err);
    } finally {
      setLoading(false);
    }
  }, []);

  // Fetch locations on mount — inlined so the effect doesn't synchronously
  // set state via fetchLocations (which would trigger react-hooks/set-state-in-effect).
  React.useEffect(() => {
    let cancelled = false;
    API.locations.getAll()
      .then((fetchedLocations) => {
        if (!cancelled) setLocations(fetchedLocations);
      })
      .catch((err) => {
        if (cancelled) return;
        setError(err.message);
        console.error('Failed to fetch locations:', err);
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });
    return () => { cancelled = true; };
  }, []);

  const addLocation = React.useCallback(async (name) => {
    try {
      const newLocation = await API.locations.add(name);
      setLocations((prev) => [...prev, newLocation]);
      return newLocation;
    } catch (err) {
      setError(err.message);
      throw err;
    }
  }, []);

  return {
    locations,
    loading,
    error,
    fetchLocations,
    addLocation,
  };
}
