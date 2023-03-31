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

    static async release(locationId, gameId) {
        return fetch(API_URL + `/v1/locations/${locationId}/games/${gameId}/reservations`, {
            method: 'DELETE',
        })
        .then((response) => null)
        .catch((error) => console.log(`Error: ${error}`));
    }

    static async remove(locationId, gameId) {
        return fetch(API_URL + `/v1/locations/${locationId}/games/${gameId}`, {
            method: 'DELETE',
        })
        .then((response) => null)
        .catch((error) => console.log(`Error: ${error}`));
    }

    static async enable(locationId, gameId) {
        return fetch(API_URL + `/v1/locations/${locationId}/games/${gameId}/enable`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
        })
        .then((response) => null)
        .catch((error) => console.log(`Error: ${error}`));
    }

    static async disable(locationId, gameId) {
        return fetch(API_URL + `/v1/locations/${locationId}/games/${gameId}/disable`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
        })
        .then((response) => null)
        .catch((error) => console.log(`Error: ${error}`));
    }

    static async add(locationId, name, abbreviation) {
        return fetch(API_URL + `/v1/locations/${locationId}/games`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                name: name,
                abbreviation: abbreviation,
            })
        })
        .then((response) => response.json())
        .catch((error) => console.log(`Error: ${error}`));
    }

    static async update(locationId, gameId, name, abbreviation) {
        return fetch(API_URL + `/v1/locations/${locationId}/games/${gameId}`, {
            method: 'PUT',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                name: name,
                abbreviation: abbreviation,
            })
        })
        .then((response) => response.json())
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

    static async addLocation(name) {
        return fetch(API_URL + `/v1/locations`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                name: name,
            })
        })
        .then((response) => response.json())
        .catch((error) => console.log(`Error: ${error}`));
    }
}

