use std::time::Duration;

pub enum FetchStrategy {
    NoCache,
    Cached(Option<CacheStrategy>),
    ForceCache,
}

impl Default for FetchStrategy {
    fn default() -> Self {
        Self::Cached(None)
    }
}

pub struct CacheStrategy {
    pub max_age: Duration,
}
