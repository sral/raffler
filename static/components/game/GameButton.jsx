import React from 'react';
import { Button, ButtonGroup, Dropdown, DropdownButton, OverlayTrigger, Tooltip } from 'react-bootstrap';
import { RESERVED_WARNING_THRESHOLD_MINUTES } from '../../constants.js';

/**
 * Individual game button with reserve/release and dropdown menu
 */
export const GameButton = React.memo(function GameButton({
  game,
  onButtonClick,
  onToggleDisabled,
  onEdit,
  onRemove,
  onShowDetails,
  onShowStats,
}) {
  const { name, abbreviation, disabled_at, reserved_at, reserved_minutes } = game;

  const isDisabled = Boolean(disabled_at);
  const isReserved = Boolean(reserved_at);

  const getButtonVariant = () => {
    if (isDisabled) return 'secondary';
    if (!isReserved) return 'success';
    // If reserved for more than threshold, show yellow (warning)
    if (reserved_minutes > RESERVED_WARNING_THRESHOLD_MINUTES) return 'warning';
    // Otherwise show red (danger)
    return 'danger';
  };

  const buttonVariant = getButtonVariant();
  const buttonText = isReserved ? `${abbreviation} (${reserved_minutes}m)` : abbreviation;

  const handleButtonClick = React.useCallback(() => {
    if (!isDisabled) {
      onButtonClick();
    }
  }, [isDisabled, onButtonClick]);

  const handleToggleDisabled = React.useCallback(() => {
    onToggleDisabled();
  }, [onToggleDisabled]);

  const handleEdit = React.useCallback(() => {
    onEdit();
  }, [onEdit]);

  const handleRemove = React.useCallback(() => {
    onRemove();
  }, [onRemove]);

  const handleShowDetails = React.useCallback(() => {
    onShowDetails();
  }, [onShowDetails]);

  const handleShowStats = React.useCallback(() => {
    onShowStats();
  }, [onShowStats]);

  return (
    <OverlayTrigger placement="top" overlay={<Tooltip><strong>{name}</strong></Tooltip>}>
      <ButtonGroup key={`buttongroup-${game.id}`} className="fixed-width-button mx-1 my-2">
        <Button
          title={name}
          variant={buttonVariant}
          disabled={isDisabled}
          onClick={handleButtonClick}
          aria-label={`${name} - ${isReserved ? `Reserved for ${reserved_minutes} minutes` : 'Available'}`}
        >
          {buttonText}
        </Button>
        <DropdownButton
          variant={buttonVariant}
          as={ButtonGroup}
          id={`dropdown-${game.id}`}
          drop="end"
          aria-label={`Options for ${name}`}
        >
          <Dropdown.Item onClick={handleToggleDisabled}>
            {isDisabled ? 'Enable' : 'Disable'}
          </Dropdown.Item>
          <Dropdown.Item onClick={handleEdit}>
            Edit
          </Dropdown.Item>
          <Dropdown.Item onClick={handleRemove}>
            Remove
          </Dropdown.Item>
          <Dropdown.Item onClick={handleShowDetails}>
            Details
          </Dropdown.Item>
          <Dropdown.Item onClick={handleShowStats}>
            Stats
          </Dropdown.Item>
        </DropdownButton>
      </ButtonGroup>
    </OverlayTrigger>
  );
});
