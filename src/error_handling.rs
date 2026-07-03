

use super::{*};

type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug)]
pub struct ErrorWithRetryAfter{
   pub error: Error,
   pub retry_after_s: u64
}

#[derive(Debug)]
pub struct RetryAfter{
   pub s: u64
}

#[derive(Debug)]
pub enum SnotifyError {
    ClientError(ErrorWithRetryAfter),
    UnsupportedItemType((PlayableItem, RetryAfter)),
    NoPlayableItem(RetryAfter),
    MissingStringId((Song, RetryAfter)),
    NoCurrentlyPlayingContext,
    PathNotExistent(String),
    FailedSerializeToJson,
    FailedDeserializeFromJson,
    FileIOFailure(Error),
    Unknown,
}

// #todo use macros
impl fmt::Display for SnotifyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ClientError(error_with_retry) => {
                write!(f, "Client error: \n {}", error_with_retry.error)?;
                write!(f, "Retry after {} seconds", error_with_retry.retry_after_s)
            },
            Self::UnsupportedItemType((item, retry_after)) => {
                write!(f, "The handling of playable item {:#?} is not implemented.", item)?;
                write!(f, "Retry after {} seconds", retry_after.s)
            },
            Self::NoPlayableItem(retry_after_s) => {
                write!(f, "Failed to fetch playable item.")?;
                write!(f, "Retry after {} seconds",retry_after_s.s)
            },
            Self::MissingStringId((song, retry_after_s)) => {
                write!(f, "Missing id on song {} ", song)?;
                write!(f, "Retry after {} seconds", retry_after_s.s)
            },
            Self::NoCurrentlyPlayingContext => {
                write!(f, "Failed to fetch currently playing context.")
            },
            Self::PathNotExistent(path) => {
                write!(f, "Path at {} does not exist", path)
            },
            Self::FailedSerializeToJson => {
                write!(f, "Failed to serialize to json.")
            },
            Self::FailedDeserializeFromJson => {
                write!(f, "Failed to deserialize from json.")
            },
            Self::FileIOFailure(error) => {
                write!(f, "File IO error: {error}")
            },
            Self::Unknown => {
                write!(f, "Encountered an unknown error.")
            }
        }
    }
}

impl SnotifyError {
    pub fn retry_after_s(&self) -> Option<u64> {
        match self {
            Self::ClientError(error_with_retry) => {
                Some(error_with_retry.retry_after_s)
            },
            Self::UnsupportedItemType((_, retry_after_s)) => {
                Some(retry_after_s.s)
            },
            Self::NoPlayableItem(retry_after_s) => {
                Some(retry_after_s.s)
            },
            Self::MissingStringId((_, retry_after_s)) => {
                Some(retry_after_s.s)
            },
            _  => {
                None
            },
        }
    } 
}
