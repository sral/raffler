import {API} from './api.js';

// TODO;: Clean this crap up once imports are sorted etc.
const Button = ReactBootstrap.Button;
const ButtonGroup = ReactBootstrap.ButtonGroup;
const Col = ReactBootstrap.Col;
const Container = ReactBootstrap.Container;
const Dropdown = ReactBootstrap.Dropdown;
const DropdownButton = ReactBootstrap.DropdownButton;
//const Modal = ReactBootstrap.Modal;
const Nav = ReactBootstrap.Nav;
const Navbar = ReactBootstrap.Navbar;
const NavDropdown = ReactBootstrap.NavDropdown;
const OverlayTrigger = ReactBootstrap.OverlayTrigger;
const Row = ReactBootstrap.Row;
const Tooltip = ReactBootstrap.Tooltip;


const API_URL = 'http://localhost:8000';

const Event = {
  Disable: "disable",
  Comment: "comment",
  Update: "update",
  Remove: "remove",
}
function LocationPicker({locations, onSelectLocationClick}) {
  return (
    <Navbar bg="light" expand="lg">
      <Container>
        <Navbar.Brand
          href="#">Razzle dazzle raffle!
        </Navbar.Brand>
        <Navbar.Toggle aria-controls="basic-navbar-nav" />
          <Navbar.Collapse id="basic-navbar-nav">
            <Nav className="ms-auto">
              <NavDropdown title="Locations" id="basic-nav-dropdown">
                {locations.map((location) => (
                  <NavDropdown.Item href="#" onClick={() => onSelectLocationClick(location)}>
                  {location.name}
                  </NavDropdown.Item>
                ))}
              </NavDropdown>
            </Nav>
          </Navbar.Collapse>
      </Container>
    </Navbar>
    );
  }


function RaffleButton({onRaffleClick}) {
  return (
    <Button
      variant="primary"
      onClick={onRaffleClick}
      size='lg'
      className="fixed-width-button mx-1 my-2"
    >
    Randomize!
    </Button>
  );
}

function GameButton({game, onButtonClick, onToggleGameDisabledClick, onRemoveGameClick}) {
  const isDisabled = game.disabled_at;
  const isReserved = game.reserved_at;

  let variant = isReserved ? 'danger' : 'success';
  if (isDisabled) {
    variant = 'secondary';
  }

  const buttonText = isReserved ? `${game.abbreviation} (${game.reserved_minutes}m)` : game.abbreviation;

  return (
    <OverlayTrigger
            key={game.name}
            placement='top'
            overlay={
              <Tooltip id='tooltip-top'>
                <strong>{game.name}</strong>
              </Tooltip>
            }
    >
      <ButtonGroup
        className="fixed-width-button mx-1 my-2"
      >
        <Button
          title={game.name}
          variant={variant}
          disabled={isDisabled}
          onClick={!isDisabled ? onButtonClick: null}
        >
          {buttonText}
          </Button>
            <DropdownButton
              variant={variant}
              as={ButtonGroup}
              id='bg-nested-dropdown'
              drop='end'
            >
              <Dropdown.Item
                eventKey={Event.Disable}
                onClick={onToggleGameDisabledClick}>{isDisabled ? 'Enable' : 'Disable'}
              </Dropdown.Item>
              <Dropdown.Item 
                eventKey={Event.Update}>Update
              </Dropdown.Item>
              <Dropdown.Item 
                eventKey={Event.Comment}>Comment
              </Dropdown.Item>
              <Dropdown.Item
                eventKey={Event.Remove}
                onClick={onRemoveGameClick}
              >
                  Remove
              </Dropdown.Item>
            </DropdownButton>
        </ButtonGroup>
      </OverlayTrigger>
  );
}

function GameList({gameStates, selectedLocation, onGameClick, onToggleGameDisabledClick, onRemoveGameClick}) {
  return (
    <Container fluid='md'>
      <Row>
        <Col>
          {gameStates.map((game, index) => (
            <GameButton
              game={game}
              onButtonClick={() => onGameClick(index)}
              onToggleGameDisabledClick={() => onToggleGameDisabledClick(selectedLocation, game)}
              onRemoveGameClick={() => onRemoveGameClick(selectedLocation, game)}
            />
          ))}
        </Col>
      </Row>
    </Container>
  );
}

function Raffler() {
  const [locations, setLocations] = React.useState([]);
  const [selectedLocation, setSelectedLocation] = React.useState(null);
  const [reservedGame, setReservedGame] = React.useState(null);
  const [gameStates, setGameStates] = React.useState([]);

  React.useEffect(() => {
    const getLocations = async () => {
      setLocations(await API.getLocations());
    }

    getLocations();
  }, []);

  React.useEffect(() => {
    const getGameStates = async () => {
      if (selectedLocation) {
        setGameStates(await API.getGames(selectedLocation.id));
      }
    }

    getGameStates();
  }, [selectedLocation]);


  async function onRaffleClick() {
    setReservedGame(await API.reserveRandom(selectedLocation.id));
    // Wasteful! This roundtrip could be avoided and only the
    // affected game could be updated. On the plus side this
    // probably helps keep UI slightly more in sync if we have
    // concurrent user fiddling with things.
    setGameStates(await API.getGames(selectedLocation.id));
  }

  async function onGameClick(i) {
    const game = gameStates[i];

    if (game.reserved_at) {
      await API.release(selectedLocation.id, game.id);
    } else {
      await API.reserve(selectedLocation.id, game.id);
      setReservedGame(game);
    }
    // Wasteful! This roundtrip could be avoided and only the
    // affected game could be updated. On the plus side this
    // probably helps keep UI slightly more in sync if we have
    // concurrent user fiddling with things.
    setGameStates(await API.getGames(selectedLocation.id));
  }

  function onSelectLocationClick(location) {
    if (selectedLocation && location.id == selectedLocation.id) {
      // Don't update if location doesn't change.
      return;
    }

    setReservedGame(null);
    setSelectedLocation(location);
  }

  async function onRemoveGameClick(location, game) {
    await API.remove(location.id, game.id);
    // Wasteful! This roundtrip could be avoided and only the
    // affected game could be updated. On the plus side this
    // probably helps keep UI slightly more in sync if we have
    // concurrent user fiddling with things.
    setGameStates(await API.getGames(selectedLocation.id));
  }

  async function onUpdateGameClick(location, updatedName, updatedabbreviation) {
    await API.update(location.id, game.id, updatedName, updatedabbreviation);
    // Wasteful! This roundtrip could be avoided and only the
    // affected game could be updated. On the plus side this
    // probably helps keep UI slightly more in sync if we have
    // concurrent user fiddling with things.
    setGameStates(await API.getGames(selectedLocation.id));
  }

  async function onToggleGameDisabledClick(location, game) {
    const isDisabled = game.disabled_at;
    const isReserved = game.reserved_at;

    if (isDisabled) {
      await API.enable(location.id, game.id);
    } else {
      if (isReserved) {
        await API.release(location.id, game.id);
      }
      await API.disable(location.id, game.id);
    }
    // Wasteful! This roundtrip could be avoided and only the
    // affected game could be updated. On the plus side this
    // probably helps keep UI slightly more in sync if we have
    // concurrent user fiddling with things.
    setGameStates(await API.getGames(selectedLocation.id));
  }

  return (
    <div class="container">
      <div>
        <LocationPicker
          locations={locations}
          onSelectLocationClick={onSelectLocationClick}
        />
      </div>
      <div  class={selectedLocation ? 'visible': 'invisible'}>
        <div class="text-center my-2">
          <h1>{selectedLocation ? selectedLocation.name : ""}</h1>
        </div>
        <div class="text-center my-2">
          <RaffleButton
            onRaffleClick={onRaffleClick}
          />
        </div>
        <div class="fixed-height-selected-game my-2">
            <h3>
              {reservedGame ? reservedGame.name : ''}
            </h3>
        </div>
        <div>
          <GameList 
            gameStates={gameStates}
            selectedLocation={selectedLocation}
            onGameClick={onGameClick}
            onToggleGameDisabledClick={onToggleGameDisabledClick}
            onRemoveGameClick={onRemoveGameClick}
          />
        </div>
      </div>
    </div>
  );
}

ReactDOM.render(
  <Raffler />,
  document.getElementById('root')
);
