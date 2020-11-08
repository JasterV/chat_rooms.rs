require('./main.css');
const net = require('net');
const $ = require('jquery');

$(() => {

    $('#join-form').on('submit', (e) => {
        e.preventDefault();
    })

    $('#join-btn').on('click', () => {
        $("#choose-room, #join-section").toggle();
    })

    $("#back-btn").on('click', () => {
        $("#choose-room, #join-section").toggle();
    });

});

// const client = new net.Socket();
// connector.connect(PORT, HOST, function () {
//     console.log('CONNECTED TO: ' + HOST + ':' + PORT);
//     connector.write()
// });
// connector.on('data', function (addr) {
//     console.log('DATA: ' + data);
// });
