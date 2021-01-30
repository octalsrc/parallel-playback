# Parallel Playback

This is a tool for synchronizing media playback (video or audio) on
multiple remote clients.  *Viewer* clients load media files from an
arbitrary external source, and then start and stop playback according
to signals from a *host* client.

This takes the form of an HTTP server which serves single-page
javascript clients for the viewers and host.  The clients in turn
stream media from any direct HTTP link, in any format that can be
played by a browser (`.webm`, `.opus`, `.mp4`, `.mp3`, etc. depending
on browser).

## Getting started

Building the `parallel-playback` server requires the [Rust
toolchain][install-rust].  From the root of this repository, run:

    $ cargo run -- 8000 ./test-config.json ./clients

You may now find the host client at
`https://localhost:8000/host/?key=zcmdzdx8` and the viewer client at
`https://localhost:8000/join/?key=pf9fflsk`.  Please note the requirement for
`https` ([details](#https-and-wss)).

The javascript clients (in `./clients`) are simple HTML pages and
scripts; they do not require any sort of compiling or processing.

### Command arguments and configuration

The `parallel-playback` server program takes three arguments: the port
to connect on, the configuration file path, and the path to static
client files.

The configuration file gives a list of distinct "parties", with keys
to connect as host (host) and as viewer (join).

    {
        "parties": [
            { "host": "zcmdzdx8", "join": "pf9fflsk" }
            { "host": "7g5vpd9r", "join": "8qq8fs2x" }
        ]
    }

The host clients of one party only interact with that party's
join/viewer clients.  Multiple host clients for a single party can be
open; Play and Pause signals sent from any host will have the same
effect.

Depending on your use case, you may want to share both the host and
join keys with party participants, or just the join key.

## Wait, so what is this thing?

The main purpose of this tool is to support a "watch-together" type
remote party, in which remote participants watch video or listen to
music while talking to each other over a video-call.  In particular,
its design solves three key problems:

1. The media comes from an external source, so the host participant's
   video-call stream is not squashed.  Media quality is also not
   limited by the video-call quality.
2. Because the media stream is distinct from the video call, viewers
   can control its volume separately.
3. "Play" signals schedule a start time several seconds in the future,
   which viewer clients match to their local clock.  This means that
   start times will not be skewed by large ping times between the host
   and viewers.

Another use-case is to play background music and effects for a
tabletop-style roleplaying game session.

## What to watch

Direct links to video and audio are not exactly common on the web, so
if you have a particular movie or soundtrack in mind to play using
this tool, you may need to set up an HTTP host for it yourself.

But if you want to use existing hosted content, there are options:

- If you're looking for music and effects for a game session, check
  out [OpenGameArt.org][oga-music].
- Podcasts must always directly host audio files, so if a podcast
  listening-party is your thing, your options are endless!  For
  example, use any download link on the [Libre Lounge archive
  page][lla].

## Troubleshooting

### HTTPS and WSS

This tool is intended to serve pages through HTTPS and connect to
clients using WSS; if you can't access `localhost` using HTTPS, you
will need to edit the `wss://` websocket links to `ws://` in the
[host](./clients/host/index.js) and [join](./clients/host/index.js)
clients for local testing (yuck, I know).  [This guide][localcert]
from Let's Encrypt has some suggestions for fixing this problem more
cleanly for your dev environment.

## License

This program is free software: you can redistribute it and/or modify
it under the terms of the [GNU General Public License][gpl3] as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the [GNU
General Public License][gpl3] for more details.

*Note: If you publish a modified version, you should edit the footers
of the [host](./clients/host/index.html) and
[join](./clients/join/index.html) clients to link to the repository of
your modified source code.*


[install-rust]: https://www.rust-lang.org/tools/install
[oga-music]: https://opengameart.org/art-search-advanced?keys=&field_art_type_tid%5B%5D=12&sort_by=count&sort_order=DESC
[lla]: https://librelounge.org/archive/
[localcert]: https://letsencrypt.org/docs/certificates-for-localhost/
[gpl3]: https://www.gnu.org/licenses/gpl-3.0.html
