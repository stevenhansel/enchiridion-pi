pub enum ApplicationEvent {
    MediaUpdate,
}

impl ApplicationEvent {
    pub fn tag(&self) -> &'static str {
        match self {
            ApplicationEvent::MediaUpdate => "media_update",
        }
    }
}
