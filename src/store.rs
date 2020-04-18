use bincode::{deserialize, serialize};
use domain::optimized::OptimizedEvent;
use domain::Event;
use rocksdb::{DBCompressionType, Direction, IteratorMode, Options, DB};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

const STORE_PATH: &str = "sware_store";
const EVENT_THRESHOLD_MICROS: u128 = 1000 * 1000 * 60 * 60; // 1 hr

pub struct Store {
    db: DB,
    mutex: Mutex<usize>,
}

impl Store {
    pub fn new() -> Store {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.enable_statistics();
        opts.set_compression_type(DBCompressionType::Lz4hc);
        let db = DB::open(&opts, STORE_PATH)
            .expect(&format!("Unable to open store at path: {:?}", STORE_PATH));
        let mutex = Mutex::new(0);

        Store { db, mutex }
    }

    pub fn put_event(&self, event: &mut Event) {
        let key = self.get_key();
        event.ingest_ts = key;
        match serialize(event) {
            Ok(value) => match self.db.put(&key.to_be_bytes(), &value) {
                Ok(_) => (),
                Err(e) => error!("Unable to put event: {}", e)
            },
            Err(e) => error!("Unable to serialize event: {}", e)
        };
    }

    pub fn get_events(&self, key: Option<u128>) -> Vec<OptimizedEvent> {
        let key = if key.is_none() {
            get_system_micros() - EVENT_THRESHOLD_MICROS
        } else {
            key.unwrap() + 1 // Skip the key passed in
        };

        self
            .db
            .iterator(IteratorMode::From(&key.to_be_bytes(), Direction::Forward))
            .map(|(_, value)| {
                match deserialize(&*value) {
                    Ok(value) => Some(value),
                    Err(e) => {
                        error!("Unable to deserialize event with key {}: {}", key, e);
                        None
                    }
                }
            })
            .filter_map(Option::unwrap)
            .collect()
    }

    fn get_key(&self) -> u128 {
        let _guard = self.mutex.lock().expect("Unable to acquire lock");
        let mut value = Ok(Some(vec![]));
        let mut key: u128 = 0;

        while value.is_err() || (value.is_ok() && value.unwrap().is_some()) {
            key = get_system_micros();
            value = self.db.get(&key.to_be_bytes());
        }

        key
    }
}

fn get_system_micros() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Unable to get system time");
    since_the_epoch.as_micros()
}
