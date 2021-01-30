var host = window.location.host;
const urlParams = new URLSearchParams(window.location.search);
const socket = new WebSocket(`wss://${host}/msg`);
vid = document.getElementById("thevideo");
var init = 0;

mkstatus = function (msg) {
    document.getElementById("status").innerHTML = msg;
};

vid.onplay = function() {
    if (init == 0) {
        init = 1;
        vid.pause();
        socket.send('ready');
        mkstatus("Ready!");
        // document.getElementById("status").childNodes[0].innerHTML = "Ready!";
    }
};


socket.addEventListener('open', function (event) {
    var joinReq = { Join: urlParams.get('key') };
    socket.send(JSON.stringify(joinReq));
});
socket.addEventListener('message', function (event) {
    console.log(event.data);
    cmd = JSON.parse(event.data);

    if ("Pause" == cmd) {
        vid.pause();
        mkstatus("Stream is paused.");
    } else if ("Play" in cmd) {
        var cmd_src = cmd.Play[0];
        var cmd_seek = cmd.Play[1];
        var cmd_time = cmd.Play[2];

        vid.pause();
        if (vid.src != cmd_src) {
            vid.src = cmd_src
        }
        vid.currentTime = cmd_seek;

        var diff = new Date(cmd_time) - Date.now();
        var timeout = setTimeout(function(){
            vid.play();
            mkstatus("Stream is playing.");
        }, diff);
        document.getElementById("status").innerHTML =
            "Sync start in "
            + Math.ceil(diff / 1000)
            + " seconds.";
    } else {
        document.getElementById("status").innerHTML =
            "Unknown command: " + JSON.stringify(cmd);
    }
});
socket.addEventListener('close', (event) => {
    document.getElementById("status").innerHTML =
        "⚠️ Connection closed.  Refresh page?"
});

setInterval(function() { socket.send("ping") }, 5000 );
