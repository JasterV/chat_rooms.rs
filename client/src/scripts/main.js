import '../styles/main.css';
import 'regenerator-runtime/runtime'
import { fetchRoomInfo, createRoom } from './controller.js';
import {leftBubble, rightBubble} from './templates';

import $ from 'jquery';

$(() => {
    let App = {};

    $('#join-form').on('submit', async (e) => {
        e.preventDefault();
        let id = $('#join-input').val().trim();
        if (id.length > 0) {
            try {
                let data = await fetchRoomInfo(id);
                openRoom(data);
            } catch (err) {
                alert(err.message);
            }
        }
    })

    $("#create-btn").on('click', async (e) => {
        try {
            let data = await createRoom();
            openRoom(data);
        } catch (err) {
            alert(err.message);
        }
    });

    $('#join-btn, #back-btn').on('click', () => $("#choose-room, #join-section").toggle())

    $(".chat-input input").on('keyup', (e) => {
        let val = $(e.currentTarget).val().trim();
        if (val.length > 0 && e.key == 'Enter') {
            $(e.currentTarget).val("");
            App.socket.send(val);
        }
    })

    function openRoom(data) {
        openSocket(data.addr);
        $('#room-id').text(data.id);
        $("#chat-room").show();
        $("#join-section, #choose-room").hide();
    }

    /** SOCKETS LOGIC */
    function openSocket(addr) {
        App.socket = new WebSocket(`ws://${addr}`);

        App.socket.addEventListener("message", (event) => {
            let response = JSON.parse(event.data);
            if(response.first) {
                App.userId = response.id;
                return;
            }

            if(response.id == App.userId) $('.chat-body').append(rightBubble(response));
            else $('.chat-body').append(leftBubble(response));
        })
    }
});



