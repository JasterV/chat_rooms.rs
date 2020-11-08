import '../styles/main.css';
import 'regenerator-runtime/runtime'
import { ROOMS_API } from './config';
import $ from 'jquery';
const net = require('net');

$(() => {
    const App = {
        socket: "",
    }

    $('#join-form').on('submit', async (e) => {
        e.preventDefault();
        let id = $('#join-input').val().trim();
        if (id.length > 0) {
            try {
                let info = await fetchRoomInfo(id);
                openRoom(info);
                $("#chat-room, #join-section").toggle();
            } catch (err) {
                alert(err.message);
            }
        }
    })

    $("#create-btn").on('click', async (e) => {
        try {
            let info = await createRoom();
            openRoom(info);
            $("#chat-room, #choose-rom").toggle();
        } catch (err) {
            alert(err.message);
        }
    });

    $('#join-btn').on('click', () => {
        $("#choose-room, #join-section").toggle();
    })

    $("#back-btn").on('click', () => {
        $("#choose-room, #join-section").toggle();
    });

    $(".chat-input input").on('keyup', (e) => {
        let val = $(e.currentTarget).val().trim();
        if (val.length > 0 && e.key == 'Enter') {
            App.socket.write(val);
            $('.chat-body').append($(`<div class="right-bubble msg-bubble">${val}</div>`))
        }
    })
});

async function createRoom() {
    let response = await fetch(ROOMS_API + '/rooms?create');
    if (!response.ok) throw new Error(response.statusText);
    return response.json();
}

async function fetchRoomInfo(id) {
    let response = await fetch(ROOMS_API + '/rooms/' + id);
    if (!response.ok) throw new Error(response.statusText);
    return response.json();
}

function openRoom(data) {
    $('#room-id').text(data.id);
    openSocket(data.addr);
}

function openSocket(addr) {
    addr = addr.split(':');
    console.log(addr);
    const socket = new net.Socket();
    App.socket = socket;
    socket.connect(addr[1], addr[0], function () {
        console.log('CONNECTED TO: ' + HOST + ':' + PORT);
    });

    socket.on('data', function (msg) {
        $('.chat-body').append($(`<div class="left-bubble msg-bubble">${msg}</div>`))
    });

    socket.on('close', function () {
        alert("Room closed!");
    })
}


