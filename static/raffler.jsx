import {API} from './api.js';

const API_URL = 'http://localhost:8000';


function RaffleButton({setGameStates, setReservedGame}) {
    async function handleClick() {
        setReservedGame(await API.reserveRandom(1));
        // Wasteful round-trip! See comment below.
        setGameStates(await API.getGames(1));
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

function GameList({gameStates, setGameStates, setReservedGame}) {
    async function onButtonClick(i) {
        let game = gameStates[i];
        let gameId = gameStates[i].id;
        let reservedAt = gameStates[i].reserved_at;

        if (reservedAt) {
            await API.cancelReservation(1, gameId);
        } else {
            await API.reserve(1, gameId);
            setReservedGame(game);
        }
        // Wasteful! This roundtrip could be avoided and only the
        // affected game could be updated. On the plus side this
        // probably helps keep UI slightly more in sync if we have
        // concurrent user fiddling with things.
        let nextGameState = await API.getGames(1)
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
            setGameStates(await API.getGames(1));
        }

        getGameStates();
       }, []);

    return (
            <div class="container">
                <div class="text-center">
                    <RaffleButton
                        setGameStates={setGameStates}
                        setReservedGame={setReservedGame}
                    />
                </div>
            <div>
                <h3 class="text-center">{reservedGame ? reservedGame.name : ''}</h3>
            </div>
            <GameList
                gameStates={gameStates}
                setGameStates={setGameStates}
                setReservedGame={setReservedGame}
            />
        </div>
    );
}

ReactDOM.render(
    <Raffler />,
    document.getElementById('root')
);