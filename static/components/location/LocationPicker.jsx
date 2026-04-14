import React from 'react';
import { Navbar, Container, Nav, NavDropdown } from 'react-bootstrap';

/**
 * Navigation bar with location selector and add game button
 */
export const LocationPicker = React.memo(function LocationPicker({
  locations,
  selectedLocation,
  onSelectLocation,
  onAddLocation,
  onAddGame,
}) {
  const handleSelectLocation = React.useCallback((location) => {
    onSelectLocation(location);
  }, [onSelectLocation]);

  const handleAddLocation = React.useCallback((e) => {
    e.preventDefault();
    onAddLocation();
  }, [onAddLocation]);

  const handleAddGame = React.useCallback((e) => {
    e.preventDefault();
    onAddGame();
  }, [onAddGame]);

  return (
    <Navbar bg="light" expand="lg">
      <Container>
        <Navbar.Brand>{selectedLocation?.name || 'Select a location'}</Navbar.Brand>
        <Navbar.Toggle aria-controls="basic-navbar-nav" />
        <Navbar.Collapse id="basic-navbar-nav">
          <Nav className="ms-auto">
            {selectedLocation && (
              <Nav.Link
                href="#"
                variant="outline-secondary"
                onClick={handleAddGame}
              >
                Add game
              </Nav.Link>
            )}
            <NavDropdown title="Locations" id="basic-nav-dropdown">
              {locations.map((location) => (
                <NavDropdown.Item
                  key={location.id}
                  href="#"
                  onClick={() => handleSelectLocation(location)}
                  active={selectedLocation?.id === location.id}
                >
                  {location.name}
                </NavDropdown.Item>
              ))}
              <NavDropdown.Divider />
              <NavDropdown.Item href="#" onClick={handleAddLocation}>
                Add new location
              </NavDropdown.Item>
            </NavDropdown>
          </Nav>
        </Navbar.Collapse>
      </Container>
    </Navbar>
  );
});
