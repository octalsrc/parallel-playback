use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum IntroMsg {
    Host(String),
    Join(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum HostMsg {
    /// Play(url, seek_pos ("min:sec"), start_offset (seconds))
    Play(String, String, usize),
    Pause,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum JoinerMsg {
    /// Play(url, seek_pos (seconds), start_time (milliseconds unix))
    Play(String, usize, u128),
    Pause,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intromsg_rep() {
        assert_eq!(
            serde_json::to_string(&IntroMsg::Host("asdf".to_string())).unwrap(),
            "{\"Host\":\"asdf\"}",
        );
        assert_eq!(
            serde_json::to_string(&IntroMsg::Join("asdf".to_string())).unwrap(),
            "{\"Join\":\"asdf\"}",
        );
    }

    #[test]
    fn hostmsg_rep() {
        assert_eq!(
            serde_json::to_string(&HostMsg::Play("http://asdf".to_string(), "1:24:30".to_string(), 5)).unwrap(),
            "{\"Play\":[\"http://asdf\",\"1:24:30\",5]}"
        );
        assert_eq!(
            serde_json::to_string(&HostMsg::Pause).unwrap(),
            "\"Pause\""
        );
    }
    #[test]
    fn joinmsg_rep() {
        assert_eq!(
            serde_json::to_string(&JoinerMsg::Play("http://asdf".to_string(), 120, 5000)).unwrap(),
            "{\"Play\":[\"http://asdf\",120,5000]}"
        );
        assert_eq!(
            serde_json::to_string(&JoinerMsg::Pause).unwrap(),
            "\"Pause\""
        );
    }
}
