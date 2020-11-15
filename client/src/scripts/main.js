import '../styles/main.css';
import 'regenerator-runtime/runtime'
import { fetchRoomInfo, createRoom } from './controller.js';
import $ from 'jquery';

$(() => {
    let App = {};

    $('#join-form').on('submit', async (e) => {
        e.preventDefault();
        let id = $('#join-input').val().trim();
        if (id.length > 0) {
            try {
                let data = await fetchRoomInfo(id);
                openSocket(data.addr);
                $('#room-id').text(data.id);
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
            $("#chat-room, #choose-room").toggle();
        } catch (err) {
            alert(err.message);
        }
    });

    $('#join-btn, #back-btn').on('click', () => $("#choose-room, #join-section").toggle())

    $(".chat-input input").on('keyup', (e) => {
        let val = $(e.currentTarget).val().trim();
        if (val.length > 0 && e.key == 'Enter')
            App.socket.send(val);
    })

    /** SOCKETS LOGIC */
    function openSocket(addr) {
        App.socket = new WebSocket(`ws://${addr}`);

        App.socket.addEventListener("open", () => {
            console.log("Connected!");
        });

        App.socket.addEventListener("message", (event) => {
            $('.chat-body').append($(`<div class="left-bubble msg-bubble">${event.data}</div>`))
        })
    }
});



