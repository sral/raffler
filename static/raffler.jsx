import {API} from './api.js';

// TODO;: Clean this crap up once imports are sorted etc.
const Button = ReactBootstrap.Button;
const ButtonGroup = ReactBootstrap.ButtonGroup;
const Col = ReactBootstrap.Col;
const Container = ReactBootstrap.Container;
const Dropdown = ReactBootstrap.Dropdown;
const DropdownButton = ReactBootstrap.DropdownButton;
const Nav = ReactBootstrap.Nav;
const Navbar = ReactBootstrap.Navbar;
const NavDropdown = ReactBootstrap.NavDropdown;
const Row = ReactBootstrap.Row;


const API_URL = 'http://localhost:8000';

function LocationPicker({locations, setSelectedLocation, selectedLocation, setReservedGame}) {

  function handleClick(locationId) {
    if (locationId == selectedLocation) {
      return;
    }

    setReservedGame(null);
    setSelectedLocation(locationId);
  }

  return (
    <Navbar bg="light" expand="lg">
      <Container>
        <Navbar.Brand
          href="#">Raffler frob baz bar foo
        </Navbar.Brand>
        <Navbar.Toggle aria-controls="basic-navbar-nav" />
          <Navbar.Collapse id="basic-navbar-nav">
            <Nav className="me-auto">
              <NavDropdown title="Locations" id="basic-nav-dropdown">
                {locations.map((location, index) => (
                  <NavDropdown.Item href="#" onClick={() => handleClick(location.id)}>
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


function RaffleButton({setGameStates, setReservedGame, selectedLocation}) {
  async function handleClick() {
    setReservedGame(await API.reserveRandom(selectedLocation));
    // Wasteful round-trip! See comment below.
    setGameStates(await API.getGames(selectedLocation));
  }

  return (
    <Button
      variant="primary"
      onClick={handleClick}
      size='lg'
      className="fixed-width-button mx-1 my-2"
    >
    Raffle!
    </Button>
  );
}

function GameButton({name, abbreviation, disabledAt, reservedAt, reservedMinutes, onButtonClick}) {
  let variant = reservedAt ? 'danger' : 'success';
  let disabled = disabledAt ? true : false;
  if (disabled) {
    variant = 'secondary';
  }
  let buttonText = reservedAt ? `${abbreviation} (${reservedMinutes}m)` : abbreviation;

  return (
    <ButtonGroup
      className="fixed-width-button mx-1 my-2"
    >
      <Button
        title={name}
        variant={variant}
        disabled={disabled}
        onClick={!disabled ? onButtonClick: null}
      >
        {buttonText}
        </Button>
          <DropdownButton
            variant={variant}
            as={ButtonGroup}
            id='bg-nested-dropdown'
            drop='end'
          >
            <Dropdown.Item eventKey="1">Disable/enable</Dropdown.Item>
            <Dropdown.Item eventKey="2">Update</Dropdown.Item>
            <Dropdown.Item eventKey="3">Comment</Dropdown.Item>
            <Dropdown.Item eventKey="4">Remove</Dropdown.Item>
          </DropdownButton>
      </ButtonGroup>
  );
}

function GameList({gameStates, setGameStates, setReservedGame, selectedLocation}) {
  async function onButtonClick(i) {
    let game = gameStates[i];
    let gameId = gameStates[i].id;
    let reservedAt = gameStates[i].reserved_at;

    if (reservedAt) {
      await API.cancelReservation(selectedLocation, gameId);
    } else {
      await API.reserve(selectedLocation, gameId);
      setReservedGame(game);
    }
    // Wasteful! This roundtrip could be avoided and only the
    // affected game could be updated. On the plus side this
    // probably helps keep UI slightly more in sync if we have
    // concurrent user fiddling with things.
    let nextGameState = await API.getGames(selectedLocation)
    setGameStates(nextGameState);
  }

  return (
    <Container fluid='md'>
      <Row>
        <Col>
          {gameStates.map((game, index) => (
            <GameButton
            name={game.name}
            abbreviation={game.abbreviation}
            disabledAt={game.disabled_at}
            reservedAt={game.reserved_at}
            reservedMinutes={game.reserved_minutes}
            onButtonClick={() => onButtonClick(index)}
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
        setGameStates(await API.getGames(selectedLocation));
      }
    }

    getGameStates();
  }, [selectedLocation]);

  return (
    <div class="container">
      <div>
        <LocationPicker
          locations={locations}
          setSelectedLocation={setSelectedLocation}
          setReservedGame={setReservedGame}
          selectedLocation={selectedLocation}
        />
      </div>
      <div  class={selectedLocation ? 'visible': 'invisible'}>
        <div class="text-center my-2">
          <RaffleButton
            setGameStates={setGameStates}
            setReservedGame={setReservedGame}
            selectedLocation={selectedLocation}
          />
        </div>
        <div class="fixed-height-selected-game my-2">
            <h3>{reservedGame ? reservedGame.name : ''}</h3>
        </div>
        <GameList
          gameStates={gameStates}
          setGameStates={setGameStates}
          setReservedGame={setReservedGame}
          selectedLocation={selectedLocation}
        />
      </div>
    </div>
  );
}

ReactDOM.render(
  <Raffler />,
  document.getElementById('root')
);
