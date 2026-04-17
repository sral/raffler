import React from 'react';
import { Button } from 'react-bootstrap';

/**
 * Button for reserving a random game
 */
export const RandomizeButton = React.memo(function RandomizeButton({ onClick, disabled = false }) {
  return (
    <Button
      variant="primary"
      onClick={onClick}
      size="lg"
      className="fixed-width-button mx-1 my-2"
      disabled={disabled}
      aria-label="Reserve random game"
    >
      Randomize!
    </Button>
  );
});
