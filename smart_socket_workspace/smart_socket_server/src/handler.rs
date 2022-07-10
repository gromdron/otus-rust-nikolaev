use crate::smart_socket::SmartSocket;
use std::str::Split;

pub struct Request<'a>(Split<'a, &'a str>);

impl<'a> Request<'a> {
    pub fn new(s: &'a str) -> Self {
        Self(s.split("|||"))
    }

    pub fn next(&mut self) -> &'a str {
        self.0.next().unwrap_or("")
    }
}

pub struct RequestHandler {
    smart_socket: SmartSocket,
}

impl RequestHandler {
    pub fn new(smart_socket: SmartSocket) -> Self {
        Self { smart_socket }
    }

    pub fn handle(&mut self, mut request: Request) -> String {
        let command = request.next();
        match command {
            "get_report" => self.get_report(request),
            "get_state" => self.get_state(request),
            "turn_on" => self.turn_on(request),
            "turn_off" => self.turn_off(request),
            _ => "Bad command".into(),
        }
    }

    fn get_report(&self, _request: Request) -> String {
        self.smart_socket.get_report()
    }

    fn get_state(&self, _request: Request) -> String {
        format!("{:?}", self.smart_socket.get_state())
    }

    fn turn_on(&mut self, _request: Request) -> String {
        self.smart_socket.turn_on();
        "Turned on".to_string()
    }

    fn turn_off(&mut self, _request: Request) -> String {
        self.smart_socket.turn_off();
        "Turned off".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Request, RequestHandler, SmartSocket};

    #[test]
    fn append_fetch() {
        let smart_socket = SmartSocket::default();
        let mut handler = RequestHandler::new(smart_socket);

        let req_str = "get_report|||".to_string();
        let req = Request::new(&req_str);
        assert_eq!(
            handler.handle(req),
            "SmartSocket state: Off, current power: 0.0".to_string()
        );

        let req_str = "turn_on|||".to_string();
        let req = Request::new(&req_str);
        assert_eq!(handler.handle(req), "Turned on".to_string());

        let req_str = "get_report|||".to_string();
        let req = Request::new(&req_str);
        assert_eq!(
            handler.handle(req),
            "SmartSocket state: On, current power: 0.0".to_string()
        );

        let req_str = "turn_off|||".to_string();
        let req = Request::new(&req_str);
        assert_eq!(handler.handle(req), "Turned off".to_string());

        let req_str = "get_report|||".to_string();
        let req = Request::new(&req_str);
        assert_eq!(
            handler.handle(req),
            "SmartSocket state: Off, current power: 0.0".to_string()
        );
    }
}
