use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::thread::JoinHandle;
use serde_json::Value;

use crate::GameLoader::Code;

include!("ScriptingAPI.rs");

pub struct Scheduler {
    commands: Arc<Mutex<HashMap<String, Box<dyn Fna(Vec<Value>, &GameState) -> bool + Send>>>>,
    event_threads: HashMap<String, Vec<JoinHandle<()>>>,
    event_senders: HashMap<String, Sender<Vec<String>>>,
    game_state: GameState,
}

impl Scheduler {
    pub fn new(game_state: GameState) -> Self {
        Scheduler {
            commands: Arc::new(Mutex::new(ScriptingAPI::init_api(&game_state).unwrap().actions)),
            event_threads: HashMap::new(),
            event_senders: HashMap::new(),
            game_state: game_state,
        }
    }

    pub fn register_command<F>(&mut self, command: &str, func: F)
        where
            F: Fn(Vec<Value>, &GameState) -> bool + Send + 'static,
    {
        self.commands.lock().unwrap().insert(command.to_string(), Box::new(func));
    }

    pub fn register_commands(&mut self, new_commands: HashMap<String, Box<dyn Fn(Vec<Value>, &GameState) -> bool + Send>>) {
        self.commands.lock().unwrap().extend(new_commands);
    }

    pub fn register_event(&mut self, event_name: &str, event_args: Vec<Value>, subcode: Vec<Code>) {
        let (sender, receiver) = channel();
        self.event_senders.insert(event_name.to_string(), sender);
        let commands = Arc::clone(&self.commands);
        let event_receiver = Arc::new(Mutex::new(receiver)); // I know wrappers are ugly, just bear with me
        let state = &self.game_state;
        let handle = thread::spawn(move || {
            // Use the cloned commands within the thread
            let commands = commands.lock().unwrap();
            // Lock the receiver, I have a feeling this will make scripts extremely slow :foreshadowing:
            let receiver = event_receiver.lock().unwrap();
            loop {
                match receiver.recv() {
                    Ok(args) => {
                        if (args == event_args) { // check event args
                            for action in &subcode {
                                run_block(&commands, action, state.clone());
                            }
                        }
                    }
                    Err(_) => break, // channel closed rip
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
            self.register_event(&event.block, event.args, event.subcode);
        }
    }

}

fn run_block(
    commands: &HashMap<String, Box<dyn Fn(Vec<Value>, &GameState) -> bool + Send>>,
    code: &Code,
    game_state: &GameState
) {
    if commands.contains_key(&code.block) {
        if let Some(command) = commands.get(&code.block) {
            let _ = command(code.args.clone(), game_state);
            for subcode in &code.subcode {
                run_block(commands, subcode, &game_state);
            }
        }
    }
}

