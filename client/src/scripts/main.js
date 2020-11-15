import '../styles/main.css';
import 'regenerator-runtime/runtime'
import { fetchRoomInfo, createRoom } from './controller.js';
import {leftBubble, rightBubble, newUserText, userGoneText} from './templates';

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

    $("#create-btn").on('click', async () => {
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

    function closeRoom() {
        $("#chat-room").hide();
        $("#choose-room").show();
    }

    /** SOCKETS LOGIC */
    function openSocket(addr) {
        App.socket = new WebSocket(`ws://${addr}`);

        App.socket.addEventListener("message", (e) => {
            let response = JSON.parse(e.data);
            let event = response.event;

            if(event == 'open') {
                App.userId = response.id;
            } else if (event == 'new_user') {
                if(response.id != App.userId)
                    $('.chat-body').append(newUserText(response));
            } else if (event == 'user_gone') {
                $('.chat-body').append(userGoneText());
            } else if (event == 'message') {
                if(response.id == App.userId) $('.chat-body').append(rightBubble(response));
                else $('.chat-body').append(leftBubble(response));
            }
        })

        App.socket.addEventListener("close", () => {
            alert("The room has been closed for innactivity");
            closeRoom();
        });
    }
});



