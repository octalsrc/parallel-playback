var host = window.location.host;
const urlParams = new URLSearchParams(window.location.search);
const socket = new WebSocket(`wss://${host}/msg`);

socket.addEventListener('open', function (event) {
    var hostReq = { Host: urlParams.get('key') };
    socket.send(JSON.stringify(hostReq));
    document.getElementById("status").innerHTML = "Connected."
});

document.getElementById("play-button").addEventListener('click', function () {
    var vtt = document.getElementById("vtt-url").value;
    if (vtt == "") {
        vtt = null;
    }
    var playReq = { Play: [
        document.getElementById("stream-url").value,
        vtt,
        document.getElementById("seek-str").value,
        Number(document.getElementById("offset-secs").value)
    ]};
    socket.send(JSON.stringify(playReq));
});

document.getElementById("pause-button").addEventListener('click', function () {
    var pauseReq = "Pause";
    socket.send(JSON.stringify(pauseReq));
});
socket.addEventListener('close', (event) => {
    document.getElementById("status").innerHTML =
        "⚠️ Connection closed.  Refresh page?"
});

setInterval(function() { socket.send("ping") }, 5000 );
