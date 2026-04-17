import React from 'react';
import { API } from '../api.js';

/**
 * Custom hook for managing games state and operations
 * @param {number|null} locationId - The selected location ID
 * @returns {Object} Games state and operations
 */
export function useGames(locationId) {
  const [games, setGames] = React.useState([]);
  const [loading, setLoading] = React.useState(false);
  const [error, setError] = React.useState(null);

  const fetchGames = React.useCallback(async () => {
    if (!locationId) {
      setGames([]);
      return;
    }

    try {
      const fetchedGames = await API.games.getAll(locationId);
      setGames(fetchedGames);
    } catch (err) {
      setError(err.message);
      console.error('Failed to fetch games:', err);
    }
  }, [locationId]);

  // Fetch games when location changes, with cancellation for rapid switches
  React.useEffect(() => {
    let cancelled = false;

    const load = async () => {
      if (!locationId) {
        setGames([]);
        return;
      }

      setLoading(true);
      setError(null);

      try {
        const fetchedGames = await API.games.getAll(locationId);
        if (!cancelled) {
          setGames(fetchedGames);
        }
      } catch (err) {
        if (!cancelled) {
          setError(err.message);
          console.error('Failed to fetch games:', err);
        }
      } finally {
        if (!cancelled) {
          setLoading(false);
        }
      }
    };

    load();
    return () => { cancelled = true; };
  }, [locationId]);

  // Optimistic update helper
  const updateGameOptimistically = React.useCallback((gameId, updates) => {
    setGames((prevGames) =>
      prevGames.map((game) =>
        game.id === gameId ? { ...game, ...updates } : game
      )
    );
  }, []);

  return {
    games,
    loading,
    error,
    fetchGames,
    updateGameOptimistically,
  };
}
