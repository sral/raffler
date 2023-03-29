import {API} from './api.js';

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
    <ReactBootstrap.Navbar bg="light" expand="lg">
      <ReactBootstrap.Container>
        <ReactBootstrap.Navbar.Brand
            href="#home">Raffler frob baz bar foo
        </ReactBootstrap.Navbar.Brand>
        <ReactBootstrap.Navbar.Toggle aria-controls="basic-navbar-nav" />
        <ReactBootstrap.Navbar.Collapse id="basic-navbar-nav">
          <ReactBootstrap.Nav className="me-auto">
            <ReactBootstrap.NavDropdown title="Locations" id="basic-nav-dropdown">
            {locations.map((location, index) => (
                <ReactBootstrap.NavDropdown.Item href="#" onClick={() => handleClick(location.id)}>
                    {location.name}
                </ReactBootstrap.NavDropdown.Item>
            ))}
            </ReactBootstrap.NavDropdown>
          </ReactBootstrap.Nav>
        </ReactBootstrap.Navbar.Collapse>
      </ReactBootstrap.Container>
    </ReactBootstrap.Navbar>
  );
}


function RaffleButton({setGameStates, setReservedGame, selectedLocation}) {
    async function handleClick() {
        setReservedGame(await API.reserveRandom(selectedLocation));
        // Wasteful round-trip! See comment below.
        setGameStates(await API.getGames(selectedLocation));
    }

    return (
        <ReactBootstrap.Button
            variant="primary"
            onClick={handleClick}
            size='lg'
            className="fixed-width-button mx-1 my-2"
        >
            Raffle!
        </ReactBootstrap.Button>
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
        <ReactBootstrap.ButtonGroup
            className="fixed-width-button mx-1 my-2"
        >
            <ReactBootstrap.Button
                title={name}
                variant={variant}
                disabled={disabled}
                onClick={!disabled ? onButtonClick: null}
            >
                {buttonText}
            </ReactBootstrap.Button>
            <ReactBootstrap.DropdownButton
                variant={variant}
                as={ReactBootstrap.ButtonGroup}
                id='bg-nested-dropdown'
                drop='end'
            >
                <ReactBootstrap.Dropdown.Item eventKey="1">Disable/enable</ReactBootstrap.Dropdown.Item>
                <ReactBootstrap.Dropdown.Item eventKey="2">Update</ReactBootstrap.Dropdown.Item>
                <ReactBootstrap.Dropdown.Item eventKey="3">Comment</ReactBootstrap.Dropdown.Item>
                <ReactBootstrap.Dropdown.Item eventKey="4">Remove</ReactBootstrap.Dropdown.Item>
            </ReactBootstrap.DropdownButton>
        </ReactBootstrap.ButtonGroup>
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
        <ReactBootstrap.Container fluid='md'>
            <ReactBootstrap.Row>
                <ReactBootstrap.Col>
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
                </ReactBootstrap.Col>
            </ReactBootstrap.Row>
        </ReactBootstrap.Container>
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