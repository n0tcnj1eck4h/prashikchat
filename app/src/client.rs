use common::ClientMessage;
use common::ServerMessage;
use ewebsock::Options;
use ewebsock::WsEvent;
use ewebsock::WsMessage;
use ewebsock::WsReceiver;
use ewebsock::WsSender;
use ewebsock::connect;

pub enum ClientState {
    Connecting,
    Opened,
    Closed,
    WsError(String),
}

pub struct Client {
    recv: WsReceiver,
    sender: WsSender,
    state: ClientState,
}

pub enum RecvError {
    Decoding,
    WsError(String),
}

pub enum RecvResult {
    Ok(ServerMessage),
    OkNone,
    Connecting,
    Disconnected,
    Error(RecvError),
}

impl Client {
    pub fn new(url: impl Into<String>) -> Result<Self, String> {
        let (sender, recv) = connect(url.into(), Options::default())?;
        let state = ClientState::Connecting;
        Ok(Self {
            recv,
            sender,
            state,
        })
    }

    pub fn recieve(&mut self) -> RecvResult {
        match &self.state {
            ClientState::Connecting => RecvResult::Connecting,
            ClientState::Closed => RecvResult::Disconnected,
            ClientState::WsError(err) => RecvResult::Error(RecvError::WsError(err.clone())),
            ClientState::Opened => {
                while let Some(msg) = self.recv.try_recv() {
                    match msg {
                        WsEvent::Opened => self.state = ClientState::Opened,
                        WsEvent::Error(err) => self.state = ClientState::WsError(err),
                        WsEvent::Closed => self.state = ClientState::Closed,
                        WsEvent::Message(ws_message) => {
                            if let WsMessage::Binary(data) = ws_message {
                                let Ok(msg) = ServerMessage::decode(&data) else {
                                    return RecvResult::Error(RecvError::Decoding);
                                };
                                return RecvResult::Ok(msg);
                            }
                        }
                    }
                }
                RecvResult::OkNone
            }
        }
    }

    pub fn send(&mut self, msg: ClientMessage) {
        let bytes = msg.encode().expect("encoding error");
        self.sender.send(WsMessage::Binary(bytes));
    }
}
