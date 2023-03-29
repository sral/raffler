const API_URL = 'http://localhost:8000';

// TODO: 
//   - Add error handling, ex 404s
//   - Organise API, ex API.Games.reserve()

export class API {
    static async reserveRandom(locationId) {
        return fetch(API_URL + `/v1/locations/${locationId}/games/reservations`, {
            method: 'POST',
            headers: {
                "Content-Type": "application/json",
            },
        })
        .then((response) => response.json())
        .catch((error) => console.log(`Error: ${error}`));
    }

    static async reserve(locationId, gameId) {
        return fetch(API_URL + `/v1/locations/${locationId}/games/${gameId}/reservations`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
        })
        .then((response) => response.json())
        .catch((error) => console.log(`Error: ${error}`));
    }

    static async cancelReservation(locationId, gameId) {
        return fetch(API_URL + `/v1/locations/${locationId}/games/${gameId}/reservations`, {
            method: 'DELETE',
            headers: {
                'Content-Type': 'application/json',
            },
        })
        .then((response) => null)
        .catch((error) => console.log(`Error: ${error}`));
    }

    static async getGames(locationId) {
        return fetch(API_URL + `/v1/locations/${locationId}/games`, {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
            },
        })
        .then((response) => response.json())
        .catch((error) => console.log(`Error: ${error}`));
    }

    static async getLocations() {
        return fetch(API_URL + '/v1/locations', {
            method: 'GET',
            headers: {
                'Content-Type': 'application/json',
            },
        })
        .then((response) => response.json())
        .catch((error) => console.log(`Error: ${error}`));
    }

}

