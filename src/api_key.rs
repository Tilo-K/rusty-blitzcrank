use std::collections::HashMap;
use std::time::SystemTime;

pub struct RateLimiter {
    requests: HashMap<String, Vec<u64>>,
    limit1: u16,
    limit1per: u16,

    limit2: u16,
    limit2per: u16,
}

impl RateLimiter {
    pub fn new() -> RateLimiter {
        RateLimiter {
            requests: HashMap::new(),

            limit1: 20,
            limit1per: 1,
            limit2: 100,
            limit2per: 120,
        }
    }

    pub fn new_custom(limit1: u16, limit1per: u16, limit2: u16, limit2per: u16) -> RateLimiter {
        RateLimiter {
            requests: HashMap::new(),

            limit1,
            limit1per,
            limit2,
            limit2per,
        }
    }

    pub fn delete_old(&mut self, endpoint: String) {
        let req = self.requests.get(&endpoint).unwrap();
        let now = SystemTime::now();

        let seconds = match now.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("🛑Systemtime is fucked up my guy 🔥🔥🥵"),
        };

        let mut new: Vec<u64> = req
            .into_iter()
            .filter(|time| (seconds - time.clone()) < self.limit2 as u64)
            .cloned()
            .collect();

        new.push(seconds);
        self.requests.insert(endpoint, new);
    }

    pub fn add_call(&mut self, endpoint: String) {
        if !self.requests.contains_key(&endpoint) {
            self.requests.insert(
                endpoint.clone(),
                Vec::with_capacity(usize::try_from(self.limit2).unwrap()),
            );
        }

        self.delete_old(endpoint.clone());

        let mut req = self.requests.get(&endpoint).unwrap().clone();
        let now = SystemTime::now();

        let seconds = match now.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("🛑Systemtime is fucked up my guy 🔥🔥🥵"),
        };

        req.push(seconds);
        self.requests.insert(endpoint, req);
    }

    pub fn is_limited(&mut self, endpoint: String) -> bool {
        self.delete_old(endpoint.clone());

        let now = SystemTime::now();

        let seconds = match now.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => panic!("🛑Systemtime is fucked up my guy 🔥🔥🥵"),
        };

        let reqs = match self.requests.get(&endpoint) {
            Some(n) => n,
            None => return false,
        };

        let count = reqs.iter().count();
        if count >= usize::try_from(self.limit2).unwrap() {
            return true;
        }

        let small_count = reqs
            .iter()
            .filter(|v| v.clone() > &(seconds - self.limit1per as u64))
            .count();

        if small_count >= usize::try_from(self.limit1).unwrap() {
            return true;
        }

        return false;
    }
}

pub struct ApiKey {
    pub key: String,
    pub ratelimiter: RateLimiter,
}

impl ApiKey {
    pub fn new(key: String) -> ApiKey {
        let limiter = RateLimiter::new();

        return ApiKey {
            key: key,
            ratelimiter: limiter,
        };
    }

    pub fn new_custom_rate(
        key: String,
        limit1: u16,
        limit1per: u16,
        limit2: u16,
        limit2per: u16,
    ) -> ApiKey {
        let limiter = RateLimiter::new_custom(limit1, limit1per, limit2, limit2per);

        return ApiKey {
            key: key,
            ratelimiter: limiter,
        };
    }
}
