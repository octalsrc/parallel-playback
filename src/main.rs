mod comms;
use comms::State;
mod config;
use config::Config;
mod timing;

use std::env;
use std::path::Path;

use warp::Filter;

#[tokio::main]
async fn main() {

    let args = env::args().collect::<Vec<String>>();
    if args.len() != 4 {
        panic!("Requires 3 arguments, got {}.", args.len() - 1);
    }
    let port = args[1].parse::<u16>().expect("Couldn't parse port.");
    let c = match Config::load(Path::new(&args[2])) {
        Ok(c) => c,
        Err(e) => panic!(e),
    };
    let static_dir = Path::new(&args[3]);
    let state = State::new(c.parties);
    let state2 = state.clone();

    let state_f = warp::any().map(move || state2.clone());

    let msg = warp::path("msg")
        .and(warp::ws())
        .and(state_f)
        .map(|ws: warp::ws::Ws, state| {
            ws.on_upgrade(move |socket|
                          comms::handle_connection(state,socket))
        });

    let common = warp::path("common")
        .and(warp::fs::dir(static_dir.join("common")));
    let join = warp::path("join")
        .and(warp::fs::dir(static_dir.join("join")));
    let host = warp::path("host")
        .and(warp::fs::dir(static_dir.join("host")));

    let routes = common.or(join).or(host).or(msg);

    warp::serve(routes).run(([127, 0, 0, 1], port)).await;
}
