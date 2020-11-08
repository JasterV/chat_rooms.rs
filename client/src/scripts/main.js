import '../styles/main.css';
import 'regenerator-runtime/runtime'
import {ROOMS_API} from './config';
import $ from 'jquery';
import net from 'net';

$(() => {

    $('#join-form').on('submit', async (e) => {
        e.preventDefault();
        let id = $('#room-id').val().trim();
        if(id.length > 0) {
            try {
                await fetchRoomInfo(id);
            } catch(err) {
                alert(err.message);
            }
        }
    })

    $('#join-btn').on('click', () => {
        $("#choose-room, #join-section").toggle();
    })

    $("#back-btn").on('click', () => {
        $("#choose-room, #join-section").toggle();
    });
});

async function fetchRoomInfo(id) {
    let response = await fetch(ROOMS_API + '/rooms/' + id);
    console.log(response);
}

// const client = new net.Socket();
// connector.connect(PORT, HOST, function () {
//     console.log('CONNECTED TO: ' + HOST + ':' + PORT);
//     connector.write()
// });
// connector.on('data', function (addr) {
//     console.log('DATA: ' + data);
// });
