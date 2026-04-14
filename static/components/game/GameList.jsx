import React from 'react';
import { Container } from 'react-bootstrap';
import { GameButton } from './GameButton.jsx';
import { PageLoader } from '../shared/LoadingSpinner.jsx';

/**
 * List of game buttons
 */
export const GameList = React.memo(function GameList({
  games,
  loading,
  onGameClick,
  onToggleDisabled,
  onRemove,
  onShowDetails,
  onShowStats,
}) {
  if (loading) {
    return <PageLoader />;
  }

  if (!games || games.length === 0) {
    return (
      <Container className="text-center mt-5">
        <p className="text-muted">No games at this location yet. Add one to get started!</p>
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
    </Container>
  );
});
