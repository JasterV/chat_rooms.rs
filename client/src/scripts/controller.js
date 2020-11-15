const ROOMS_API = "http://localhost:8000";

export async function createRoom() {
    let response = await fetch(ROOMS_API + '/rooms?create');
    if (!response.ok) throw new Error(response.statusText);
    return response.json();
}

export async function fetchRoomInfo(id) {
    let response = await fetch(ROOMS_API + '/rooms/' + id);
    if (!response.ok) throw new Error(response.statusText);
    return response.json();
}