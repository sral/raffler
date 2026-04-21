import React from 'react';
import { createRoot } from 'react-dom/client';
import 'bootstrap/dist/css/bootstrap.min.css';
import { API } from './api.js';
import { useModal } from './hooks/useModal.js';
import { useLocations } from './hooks/useLocations.js';
import { useGames } from './hooks/useGames.js';
import { useToast } from './components/shared/Toast.jsx';
import { useConfirmation } from './components/modals/ConfirmationModal.jsx';
import { LocationPicker } from './components/location/LocationPicker.jsx';
import { RandomizeButton } from './components/game/RandomizeButton.jsx';
import { GameList } from './components/game/GameList.jsx';
import { AddLocationModal } from './components/modals/AddLocationModal.jsx';
import { AddGameModal } from './components/modals/AddGameModal.jsx';
import { GameDetailsModal } from './components/modals/GameDetailsModal.jsx';
import { GameStatsModal } from './components/modals/GameStatsModal.jsx';

/**
 * Main Raffler application component
 */
export function Raffler() {
  // Location state
  const { locations, addLocation } = useLocations();
  const [selectedLocation, setSelectedLocation] = React.useState(null);

  // Game state
  const {
    games,
    loading: gamesLoading,
    error: gamesError,
    fetchGames,
    updateGameOptimistically,
  } = useGames(selectedLocation?.id);

  // Ref for selectedLocation so async callbacks always read the current value
  const selectedLocationRef = React.useRef(selectedLocation);
  selectedLocationRef.current = selectedLocation;

  const [reservedGame, setReservedGame] = React.useState(null);
  const [gameDetails, setGameDetails] = React.useState(null);
  const [gameStats, setGameStats] = React.useState(null);

  // Modals
  const addLocationModal = useModal();
  const addGameModal = useModal();
  const gameDetailsModal = useModal();
  const gameStatsModal = useModal();

  // Toast notifications
  const { showError, showSuccess, toastList } = useToast();

  // Confirmation dialog
  const { showConfirmation, confirmationDialog } = useConfirmation();

  // Handler: Select location
  const handleSelectLocation = React.useCallback((location) => {
    if (selectedLocation?.id === location.id) {
      return; // Don't update if location doesn't change
    }
    setReservedGame(null);
    setSelectedLocation(location);
  }, [selectedLocation]);

  // Handler: Add location
  const handleAddLocation = React.useCallback(async (name) => {
    try {
      const newLocation = await addLocation(name);
      setSelectedLocation(newLocation);
      setReservedGame(null);
      showSuccess('Location added successfully');
    } catch (error) {
      showError('Failed to add location', error);
      throw error;
    }
  }, [addLocation, showSuccess, showError]);

  // Handler: Add game
  const handleAddGame = React.useCallback(async (name, abbreviation) => {
    if (!selectedLocation) return;

    try {
      await API.games.add(selectedLocation.id, name, abbreviation);
      await fetchGames();
      showSuccess('Game added successfully');
    } catch (error) {
      showError('Failed to add game', error);
      throw error;
    }
  }, [selectedLocation, fetchGames, showSuccess, showError]);

  // Handler: Reserve/Release game
  const handleGameClick = React.useCallback(async (game) => {
    if (!selectedLocation) return;

    const isReserved = Boolean(game.reserved_at);

    // Optimistic update
    if (isReserved) {
      updateGameOptimistically(game.id, {
        reserved_at: null,
        reserved_minutes: null,
      });
      setReservedGame((prev) => prev?.id === game.id ? null : prev);
    } else {
      updateGameOptimistically(game.id, {
        reserved_at: new Date().toISOString(),
        reserved_minutes: 0,
      });
      setReservedGame(game);
    }

    try {
      if (isReserved) {
        await API.games.release(game.id);
      } else {
        await API.games.reserve(game.id);
      }
    } catch (error) {
      showError(`Failed to ${isReserved ? 'release' : 'reserve'} game`, error);
    }
    // Always sync with server — if mutation failed, corrects the optimistic state
    await fetchGames();
  }, [selectedLocation, updateGameOptimistically, fetchGames, showError]);

  // Handler: Toggle game disabled
  const handleToggleDisabled = React.useCallback(async (game) => {
    if (!selectedLocation) return;

    const isDisabled = Boolean(game.disabled_at);

    updateGameOptimistically(game.id, {
      disabled_at: isDisabled ? null : new Date().toISOString(),
      reserved_at: isDisabled ? game.reserved_at : null,
      reserved_minutes: isDisabled ? game.reserved_minutes : null,
    });

    try {
      if (isDisabled) {
        await API.games.enable(game.id);
      } else {
        await API.games.disable(game.id);
      }
    } catch (error) {
      showError(`Failed to ${isDisabled ? 'enable' : 'disable'} game`, error);
    }
    await fetchGames();
  }, [selectedLocation, updateGameOptimistically, fetchGames, showError]);

  // Handler: Remove game
  const handleRemoveGame = React.useCallback((game) => {
    if (!selectedLocation) return;

    showConfirmation({
      title: 'Remove Game',
      message: `Are you sure you want to remove ${game.name}?`,
      confirmText: 'Remove',
      variant: 'danger',
      onConfirm: async () => {
        const location = selectedLocationRef.current;
        if (!location) return;
        try {
          await API.games.remove(game.id);
          await fetchGames();
          showSuccess('Game removed successfully');
        } catch (error) {
          showError('Failed to remove game', error);
        }
      },
    });
  }, [selectedLocation, fetchGames, showConfirmation, showSuccess, showError]);

  // Handler: Show game details
  const handleShowDetails = React.useCallback(async (game) => {
    if (!selectedLocation) return;

    try {
      const details = await API.games.get(game.id);
      setGameDetails(details);
      gameDetailsModal.open();
    } catch (error) {
      showError('Failed to load game details', error);
    }
  }, [selectedLocation, gameDetailsModal, showError]);

  // Handler: Show game stats
  const handleShowStats = React.useCallback(async (game) => {
    if (!selectedLocation) return;

    try {
      const stats = await API.games.getStats(game.id);
      setGameStats(stats);
      gameStatsModal.open();
    } catch (error) {
      showError('Failed to load game statistics', error);
    }
  }, [selectedLocation, gameStatsModal, showError]);

  // Handler: Add note to game
  const handleAddNote = React.useCallback(async (noteText) => {
    if (!selectedLocation || !gameDetails) return;

    try {
      const newNote = await API.notes.add(gameDetails.id, noteText);
      setGameDetails((prev) => {
        if (!prev) return prev;
        return { ...prev, notes: [...(prev.notes || []), newNote] };
      });
      showSuccess('Note added successfully');
    } catch (error) {
      showError('Failed to add note', error);
      throw error;
    }
  }, [selectedLocation, gameDetails, showSuccess, showError]);

  // Handler: Delete note from game
  const handleDeleteNote = React.useCallback(async (noteId) => {
    if (!selectedLocation || !gameDetails) return;

    try {
      await API.notes.remove(gameDetails.id, noteId);
      setGameDetails((prev) => {
        if (!prev) return prev;
        return { ...prev, notes: prev.notes.filter((note) => note.id !== noteId) };
      });
      showSuccess('Note deleted successfully');
    } catch (error) {
      showError('Failed to delete note', error);
    }
  }, [selectedLocation, gameDetails, showSuccess, showError]);

  // Handler: Randomize game selection
  const handleRandomize = React.useCallback(async () => {
    if (!selectedLocation) return;

    try {
      const game = await API.games.reserveRandom(selectedLocation.id);
      setReservedGame(game);
      // Refresh games list since we don't know which one was reserved
      await fetchGames();
      showSuccess(`Reserved ${game.name}!`);
    } catch (error) {
      showError('Failed to reserve random game', error);
    }
  }, [selectedLocation, fetchGames, showSuccess, showError]);

  // Handler: Close game details modal
  const handleCloseGameDetails = React.useCallback(() => {
    gameDetailsModal.close();
  }, [gameDetailsModal]);

  const handleGameDetailsExited = React.useCallback(() => {
    setGameDetails(null);
  }, []);

  // Handler: Close game stats modal
  const handleCloseGameStats = React.useCallback(() => {
    gameStatsModal.close();
  }, [gameStatsModal]);

  const handleGameStatsExited = React.useCallback(() => {
    setGameStats(null);
  }, []);

  return (
    <div className="container-fluid">
      {toastList}
      {confirmationDialog}

      <header>
        <LocationPicker
          locations={locations}
          selectedLocation={selectedLocation}
          onSelectLocation={handleSelectLocation}
          onAddLocation={addLocationModal.open}
        />
      </header>

      <div className={selectedLocation ? 'visible' : 'invisible'}>
        <div className="text-center my-2">
          <RandomizeButton onClick={handleRandomize} disabled={gamesLoading} />
        </div>
        <div className="fixed-height-selected-game my-2">
          <h3>{reservedGame ? reservedGame.name : ''}</h3>
        </div>
        <div>
          <GameList
            games={games}
            loading={gamesLoading}
            onGameClick={handleGameClick}
            onToggleDisabled={handleToggleDisabled}
            onRemove={handleRemoveGame}
            onShowDetails={handleShowDetails}
            onShowStats={handleShowStats}
            onAddGame={addGameModal.open}
          />
        </div>
      </div>

      <AddLocationModal
        show={addLocationModal.show}
        onHide={addLocationModal.close}
        onAddLocation={handleAddLocation}
      />

      <AddGameModal
        show={addGameModal.show}
        onHide={addGameModal.close}
        onAddGame={handleAddGame}
      />

      <GameDetailsModal
        show={gameDetailsModal.show}
        onHide={handleCloseGameDetails}
        onExited={handleGameDetailsExited}
        game={gameDetails}
        onAddNote={handleAddNote}
        onDeleteNote={handleDeleteNote}
      />

      <GameStatsModal
        show={gameStatsModal.show}
        onHide={handleCloseGameStats}
        onExited={handleGameStatsExited}
        stats={gameStats}
      />
    </div>
  );
}

// Initialize React app
const container = document.getElementById('root');
const root = createRoot(container);
root.render(<Raffler />);
