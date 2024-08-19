fn run() -> Box<dyn FnOnce(Vec<String>, Receiver<SessionMessage>, Sender<SessionMessage>)> {
    Box::new(move |args, _r, sender| {
        sender
            .send(SessionMessage::Output(OutputMessage::PushLine(
                "CASH v 0.0.0".into(),
            )))
            .unwrap();
        sender
            .send(SessionMessage::Output(OutputMessage::PushLine(
                "Cold's Awful SHell".into(),
            )))
            .unwrap();
        sender
            .send(SessionMessage::Output(OutputMessage::PushLine(
                "No help to provide...".into(),
            )))
            .unwrap();

        println!("Forcing update");
        sender
            .send(SessionMessage::Output(OutputMessage::ForceUpdate))
            .unwrap();
    })
}
