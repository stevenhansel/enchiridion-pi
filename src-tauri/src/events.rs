pub enum ApplicationEvent {
    MediaUpdateStart,
    MediaUpdateEnd,
}

impl ApplicationEvent {
    pub fn tag(&self) -> &'static str {
        match self {
            ApplicationEvent::MediaUpdateStart => "media_update_start",
            ApplicationEvent::MediaUpdateEnd => "media_update_end",
        }
    }
}
