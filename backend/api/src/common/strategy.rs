use std::time::Duration;

pub enum FetchStrategy {
    NoCache,
    Cached(Option<CacheStrategy>),
    ForceCache,
}

pub struct CacheStrategy {
    pub max_age: Duration,
}
