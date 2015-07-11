use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread::{spawn, sleep_ms};
use metric::{Metric, Kind};

pub struct MetricStore {
    pub data: Arc<Mutex<HashMap<String, i64>>>,
    pub sender: Sender<Metric>
}

impl MetricStore {
    pub fn new() -> MetricStore {
    	let (tx, rx) = mpsc::channel();
        let data = Arc::new(Mutex::new(HashMap::new()));

    	let s = MetricStore{
            data: data.clone(),
        	sender: tx
        };

    	s.recieve(data.clone(), rx);
        return s
    }

    fn recieve(&self, data: Arc<Mutex<HashMap<String, i64>>>, rx: Receiver<Metric>) {
        spawn(move || {
            let timer = timer_periodic(2000);
            let mut v = data.lock().unwrap();

            loop {
                // loop endlessly, selecting over both the timer and
                // the metric receiver  
                select!(
                    r = rx.recv() => match r {
                        Ok(metric) => {
                            debug!("incoming: {:?}", metric);
                            // let name = metric.name.to_owned();
                            match metric.kind {
                                Kind::Guage => {
                                    v.insert(metric.name, metric.value);
                                    ()
                                },
                                Kind::Counter => {
                                    if v.contains_key(&metric.name) {
                                        let val = v[&metric.name];
                                        v.insert(metric.name, val + metric.value);
                                    } else {
                                        v.insert(metric.name, metric.value);
                                    }
                                    ()
                                }
                                _ => println!("other")
                            }
                        },
                        Err(e) => println!("{:?}", e)
                    },
                    _ = timer.recv() => {
                        // copy the data hashmap
                        // and then send it out to an endpoint
                        let mut d: HashMap<String, i64> = HashMap::new();
                        for (k, v) in v.drain() {
                            d.insert(k, v);
                        }
                        dump_data(&d)
                    }
                );
            };
        });
    }
}

fn dump_data(data: &HashMap<String, i64>) {
    if data.len() == 0 {
        return
    }
    println!("dumping data: ");
    for k in data.keys() {
        println!("{}: {} ", k, data[k]);
    }
}

// send an Ok down the channel periodically
fn timer_periodic(ms: u32) -> Receiver<()> {
    let (tx, rx) = mpsc::channel();
    spawn(move || {
        loop {
            sleep_ms(ms);
            if tx.send(()).is_err() {
                break;
            }
        }
    });
    rx
}