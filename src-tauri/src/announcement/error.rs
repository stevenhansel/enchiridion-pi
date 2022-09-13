pub enum DeleteAnnouncementMediaError {
    ApplicationError,
}

impl std::fmt::Display for DeleteAnnouncementMediaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeleteAnnouncementMediaError::ApplicationError => write!(f, "Application Error"),
        }
    }
}

pub enum ResyncAnnouncementsError {
    ApplicationError,
}

impl std::fmt::Display for ResyncAnnouncementsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResyncAnnouncementsError::ApplicationError => write!(f, "Application Error"),
        }
    }
}

#[derive(PartialEq)]
pub enum ParseAnnouncementConsumerDataError {
    EmptyStream(&'static str),
    ApplicationError(String),
}

impl std::fmt::Display for ParseAnnouncementConsumerDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseAnnouncementConsumerDataError::EmptyStream(message) => write!(f, "{}", message),
            ParseAnnouncementConsumerDataError::ApplicationError(message) => {
                write!(f, "{}", message)
            }
        }
    }
}
