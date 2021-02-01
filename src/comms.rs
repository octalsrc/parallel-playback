mod message;
use message::{IntroMsg,HostMsg,JoinerMsg};

use crate::config::Party;
use crate::timing;

use std::{
    collections::HashMap,
    sync::Arc,
};

use futures::{FutureExt, StreamExt};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

type Tx = mpsc::UnboundedSender<Result<Message, warp::Error>>;

#[derive(Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Debug)]
enum Role {
    Host(usize),
    Joiner(usize),
}

pub struct State {
    next_id: usize,
    conns: HashMap<usize, Tx>,
    parties: Vec<Party>,
    p_map: HashMap<usize,Role>,
}

pub type StateH = Arc<RwLock<State>>;

impl State {
    pub fn new(parties: Vec<Party>) -> StateH {
        StateH::new(RwLock::new(
            State{
                next_id: 0,
                conns: HashMap::new(),
                parties,
                p_map: HashMap::new(),
            }
        ))
    }
    pub fn insert(&mut self, tx: Tx) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.conns.insert(id, tx);
        id
    }
    /// Upgrade connection to host of party.  Ok(()) on success,
    /// Err(()) when key matches no party's host-key.
    pub fn host(&mut self, id: usize, key: String) -> Result<usize,()> {
        for (p,ks) in self.parties.iter().enumerate() {
            if key == ks.host {
                self.p_map.insert(id, Role::Host(p));
                return Ok(p)
            }
        }
        Err(())
    }
    /// Upgrade connection to joiner of party.  Ok(()) on success,
    /// Err(()) when key matches no party's join-key.
    pub fn join(&mut self, id: usize, key: String) -> Result<usize,()> {
        for (p,ks) in self.parties.iter().enumerate() {
            if key == ks.join {
                self.p_map.insert(id, Role::Joiner(p));
                return Ok(p)
            }
        }
        Err(())
    }
    /// Get role (if any) of connection.
    fn role(&self, id: usize) -> Option<Role> {
        self.p_map.get(&id).map(|r| r.clone())
    }
    /// Get Vec of channels for joiners of given party_id
    pub fn pdests(&self, party_id: usize) -> Vec<Tx> {
        let mut out = Vec::new();
        for c in self.conns.iter() {
            match self.p_map.get(&c.0) {
                Some(Role::Joiner(p)) if party_id == *p =>
                    out.push(c.1.clone()),
                _ => {},
            }
        }
        out
    }
    pub fn remove(&mut self, id: usize) {
        self.conns.remove(&id);
        self.p_map.remove(&id);
    }
}

pub async fn broadcast(state: &StateH, msg: &str, party_id: usize) {
    let dests = &state.read().await.pdests(party_id);

    for tx in dests {
        match tx.send(Ok(Message::text(msg.to_string()))) {
            Ok(_) => {},
            Err(_) => {}, // tx is disconnected, no need to send
        }
    }
}

pub async fn handle_connection(
    state: StateH,
    ws: WebSocket,
) {
    let (ws_tx, mut ws_rx) = ws.split();
    let (tx,rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);
    tokio::task::spawn(rx.forward(ws_tx).map(|result| {
        match result {
            Ok(_) => {},
            Err(e) => println!("Websocket send error: {}", e),
        }
    }));
    let conn_id = state.write().await.insert(tx);
    println!("New conn {}.", conn_id);

    while let Some(result) = ws_rx.next().await {
        let msg = match result {
            Ok(m) => match m.to_str() {
                Ok(s) => s.to_string(),
                Err(e) => {
                    println!("Msg not text? {:?}", e);
                    break;
                },
            },
            Err(e) => {
                println!("Websocket error(id={}): {}", conn_id, e);
                break;
            },
        };
        if msg != "ping" {
            let r = state.read().await.role(conn_id);
            match r {
                Some(Role::Host(p)) => match serde_json::from_str(&msg) {
                    Ok(HostMsg::Play(url,opt_vtt,seekstr,offset_secs)) =>
                       match timing::from_seekstr(&seekstr) {
                           Ok(seek_secs) => {
                               let m = serde_json::to_string(
                                   &JoinerMsg::Play(
                                       url,
                                       opt_vtt,
                                       seek_secs,
                                       timing::get_playtime(offset_secs)
                                   )
                               ).unwrap();
                               broadcast(&state, &m, p).await;
                           },
                           Err(e) => println!("Bad seekstr: {}", e),
                       },
                    Ok(HostMsg::Pause) => {
                        let m = serde_json::to_string(&JoinerMsg::Pause).unwrap();
                        broadcast(&state, &m, p).await;
                    },
                    Err(e) => println!("Could not decode as HostMsg. {}", e),
                },
                Some(Role::Joiner(_p)) => {
                    // Joiners don't send messages
                },
                None => match serde_json::from_str(&msg) {
                    Ok(IntroMsg::Host(key)) =>
                        match state.write().await.host(conn_id, key.clone()) {
                            Ok(p) => println!("{} is hosting {}.", conn_id, p),
                            Err(()) => println!("No match for key {}.", key),
                        },
                    Ok(IntroMsg::Join(key)) =>
                        match state.write().await.join(conn_id, key.clone()) {
                            Ok(p) => println!("{} is joining {}.", conn_id, p),
                            Err(()) => println!("No match for key {}.", key),
                        },
                    Err(e) => println!("Coud not decode as IntroMsg. {}", e),
                },
            }
        }
    }

    state.write().await.remove(conn_id);
    println!("Disconnect: {}", conn_id);
}
