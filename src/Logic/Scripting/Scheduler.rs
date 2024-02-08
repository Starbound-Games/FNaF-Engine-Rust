use crate::GameLoader::Code;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::thread::JoinHandle;


pub struct Scheduler {
    commands: Arc<Mutex<HashMap<String, Box<dyn FnMut() -> bool + Send>>>>,
    event_threads: HashMap<String, Vec<JoinHandle<()>>>,
    event_senders: HashMap<String, Sender<Vec<String>>>,
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            commands: Arc::new(Mutex::new(HashMap::new())),
            event_threads: HashMap::new(),
            event_senders: HashMap::new(),
        }
    }

    pub fn register_command<F>(&mut self, command: &str, func: F)
        where
            F: FnMut() -> bool + Send + 'static,
    {
        self.commands.lock().unwrap().insert(command.to_string(), Box::new(func));
    }

    pub fn register_commands<F>(&mut self, new_commands: HashMap<String, Box<dyn FnMut() -> bool + Send>>)
        where
            F: FnMut() -> bool + Send + 'static,
    {
        self.commands.lock().unwrap().extend(new_commands);
    }

    pub fn register_event(&mut self, event_name: &str, subcode: Vec<Code>) {
        let (sender, receiver) = channel();
        self.event_senders.insert(event_name.to_string(), sender);
        let commands = Arc::clone(&self.commands); // Clone Arc to use within the thread
        let handle = thread::spawn(move || {
            let commands = commands.lock().unwrap(); // Use the cloned commands within the thread
            for args in receiver {
                for action in &subcode {
                    run_block(&commands, action, &args);
                }
            }
        });
        self.event_threads.insert(event_name.to_string(), vec![handle]);
    }



    pub fn trigger_event(&self, event_name: &str, args: Vec<String>) {
        if let Some(sender) = self.event_senders.get(event_name) {
            sender.send(args.clone()).unwrap();
        }
    }


    pub fn stop_event(&mut self, event_name: &str) {
        if let Some(threads) = self.event_threads.get_mut(event_name) {
            for handle in threads {
                handle.thread().unpark(); // Unpark the thread to stop it
            }
        }
    }

    pub fn run_script(&mut self, script: Vec<Code>) {
        for event in script {
            self.register_event(&event.block, event.subcode);
        }
    }
}

fn run_block(
    commands: &HashMap<String, Box<dyn FnMut() -> bool + Send>>,
    code: &Code,
    args: &[String],
) {
    if commands.contains_key(&code.block){
        if commands.get(&code.block).is_some() {
            for subcode in &code.subcode {
                run_block(commands, subcode, args);
            }
        }
    }
}
