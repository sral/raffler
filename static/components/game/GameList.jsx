import React from 'react';
import { Button, Container } from 'react-bootstrap';
import { GameButton } from './GameButton.jsx';
import { PageLoader } from '../shared/LoadingSpinner.jsx';

/**
 * List of game buttons with an add game button at the end
 */
export const GameList = React.memo(function GameList({
  games,
  loading,
  onGameClick,
  onToggleDisabled,
  onRemove,
  onShowDetails,
  onShowStats,
  onAddGame,
}) {
  if (loading) {
    return <PageLoader />;
  }

  if (!games || games.length === 0) {
    return (
      <Container className="text-center mt-5">
        <p className="text-muted">No games at this location yet. Add one to get started!</p>
        <Button
          variant="outline-secondary"
          className="mx-1 my-2"
          style={{ border: '2px dashed #ccc', color: '#999' }}
          onClick={onAddGame}
          aria-label="Add game"
        >
          + Add game
        </Button>
      </Container>
    );
  }

  return (
    <Container className="d-flex flex-wrap">
      {games.map((game) => (
        <GameButton
          key={game.id}
          game={game}
          onButtonClick={() => onGameClick(game)}
          onToggleDisabled={() => onToggleDisabled(game)}
          onRemove={() => onRemove(game)}
          onShowDetails={() => onShowDetails(game)}
          onShowStats={() => onShowStats(game)}
        />
      ))}
      <Button
        variant="outline-secondary"
        className="fixed-width-button mx-1 my-2"
        style={{ border: '2px dashed #ccc', color: '#999' }}
        onClick={onAddGame}
        aria-label="Add game"
      >
        +
      </Button>
    </Container>
  );
});
