use std::sync::{atomic::{AtomicBool, Ordering}};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

struct TickManager {
    current_tick: Arc<Mutex<u64>>,
    ticked: Arc<Mutex<bool>>,
    stop_signal: Arc<AtomicBool>,
    callbacks: Arc<Mutex<Vec<Box<dyn Fn() + Send + 'static>>>>,
}

impl TickManager {
    fn new() -> Self {
        TickManager {
            current_tick: Arc::new(Mutex::new(0)),
            ticked: Arc::new(Mutex::new(false)),
            stop_signal: Arc::new(AtomicBool::new(false)),
            callbacks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn increment(&self) {
        let mut tick = self.current_tick.lock().unwrap();
        *tick += 1;
        self.trigger_callbacks();
    }

    fn current_tick_equals(&self, tick: u64) -> bool {
        let current_tick = self.current_tick.lock().unwrap();
        *current_tick == tick
    }

    fn get_current_tick(&self) -> u64 {
        let tick = self.current_tick.lock().unwrap();
        *tick
    }

    fn reset(&self) {
        let mut tick = self.current_tick.lock().unwrap();
        *tick = 0;
    }

    fn start(self: Arc<Self>) {
        let stop_signal = Arc::clone(&self.stop_signal);
        let ticked = Arc::clone(&self.ticked);
        let current_tick = Arc::clone(&self.current_tick);

        thread::spawn(move || {
            while !stop_signal.load(Ordering::Relaxed) {
                {
                    let mut tick = current_tick.lock().unwrap();
                    *tick += 1;
                    *ticked.lock().unwrap() = true;
                }
                self.trigger_callbacks();
                thread::sleep(Duration::from_millis(50));
            }
            stop_signal.store(false, Ordering::Relaxed)
        });
    }

    fn stop(&self) {
        self.stop_signal.store(true, Ordering::Relaxed);
    }

    fn on_tick<F>(&self, callback: F)
    where
        F: Fn() + Send + 'static,
    {
        let mut callbacks = self.callbacks.lock().unwrap();
        callbacks.push(Box::new(callback));
    }

    fn trigger_callbacks(&self) {
        let callbacks = self.callbacks.lock().unwrap();
        for callback in callbacks.iter() {
            callback();
        }
    }
}