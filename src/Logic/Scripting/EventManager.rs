use std::cell::Ref;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use async_recursion::async_recursion;
use serde_json::Value;
use regex::Regex;


use crate::GameLoader::{Code, Menu};
include!("ScriptingAPI.rs");

type CodeBlockFunction = fn(&mut EngineData, &mut EventManager, &[Value]) -> bool;

pub struct EventManager {
    listeners: Arc<Mutex<Vec<(String, Vec<String>, Vec<Code>)>>>,
    code_blocks: HashMap<String, CodeBlockFunction>,
    variables: HashMap<String, String>,
    data_values: HashMap<String, String>,
    logger: Logger,
}

impl EventManager {
    fn new(logger: Logger) -> Self {
        EventManager {
            listeners: Arc::new(Mutex::new(Vec::new())),
            code_blocks: ScriptingAPI::init_api().unwrap().actions,
            variables: HashMap::new(),
            data_values: HashMap::new(),
            logger,
        }
    }

    pub fn register_listener(&mut self, event_name: String, args: Vec<String>, subcode: Vec<Code>)
    {
        self.logger.log("Event Manager",format!("Registering Event: {:0} With args: {:?}", event_name, args).as_str());
        let listener = (event_name, args, subcode);
        self.listeners.lock().unwrap().push(listener);
    }

    pub fn run_script(&mut self, script: &Vec<Code>) {
        for event in script {
            let args: Vec<String> = event.args.iter().map(|v| v.to_string().trim_matches('"').to_string()).collect();
            self.register_listener(event.block.clone(), args, event.subcode.clone());
        }
    }


    pub fn kill_listener(&mut self, event_name: &str, args: &[String]) {
        let mut listeners = self.listeners.lock().unwrap();
        listeners.retain(|(name, arguments, _)| name != event_name || arguments != args);
    }

    pub fn kill_all_listeners(&mut self) {

        let mut listeners = self.listeners.lock().unwrap();
        let count = listeners.iter().count();
        listeners.clear();
        self.logger.log("Event Manager",format!("Killed {} Listeners", count).as_str());
    }

    pub fn register_code_block(&mut self, block_name: &str, func: CodeBlockFunction) {
        self.code_blocks.insert(block_name.to_string(), func);
    }

    pub fn trigger_event(&mut self, event_name: &str, args: &[String], engine_data: &mut EngineData) {
        let listeners = self.listeners.lock().unwrap().clone();
        for listener in listeners.iter() {
            if listener.0 == event_name && listener.1 == args {
                let subcode = &listener.2;
                for code in subcode.iter() {
                    self.run_block(engine_data, code);
                }
            }
        }
    }

    pub fn run_block(&mut self, engine_data: &mut EngineData, code: &Code) {
        if let Some(func) = self.code_blocks.get(&code.block) {
            let args: Vec<Value> = code.args.clone();
            let result = func(engine_data, self, &args);
            if result {
                for subcode in code.subcode.iter() {
                    self.run_block(engine_data, subcode);
                }
            }
        } else {
            self.logger.log("Event Manager", format!("Function for code block '{}' not found", code.block).as_str());
        }
    }

    pub fn get_expr(&self, expression: &str, engine_data: &mut EngineData) -> String {
        match self.parse_expression(expression, engine_data) {
            Ok(result) => result,
            Err(e) => format!("Error: {}", e),
        }
    }

    fn parse_expression(&self, expression: &str, engine_data: &mut EngineData) -> Result<String, Box<dyn Error>> {
        if let Some(variable_name) = self.get_variable_name(expression) {
            return self.variables.get(&variable_name)
                .map_or_else(|| Err(format!("Variable '{}' not found.", variable_name).into()), |value| Ok(value.clone()));
        } else if let Some(variable_name) = self.get_data_value(expression) {
            return self.data_values.get(&variable_name)
                .map_or_else(|| Err(format!("Data Value '{}' not found.", variable_name).into()), |value| Ok(value.clone()));
        } else if expression.starts_with("%math(") {
            return Ok(self.evaluate_math_expression(expression)?);
        } else if expression.starts_with("%ai(") {
            return Ok(self.evaluate_ai_expression(expression, engine_data)?);
        }
        Ok(expression.to_string())
    }

    fn get_variable_name(&self, expression: &str) -> Option<String> {
        let regex = Regex::new(r"%var\((.*?)\)").unwrap();
        regex.captures(expression)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
    }

    fn get_data_value(&self, expression: &str) -> Option<String> {
        let regex = Regex::new(r"%data\((.*?)\)").unwrap();
        regex.captures(expression)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
    }

    fn evaluate_math_expression(&self, expression: &str) -> Result<String, Box<dyn std::error::Error>> {
        let regex = Regex::new(r"%math\((.*?)\)").unwrap();
        regex.captures(expression)
            .and_then(|caps| caps.get(1))
            .map(|m| self.evaluate_math(m.as_str()))
            .ok_or_else(|| "Invalid math expression.".into())
    }

    fn evaluate_ai_expression(&self, expression: &str, engine_data: &mut EngineData) -> Result<String, Box<dyn std::error::Error>> {
        let regex = Regex::new(r"%ai\((.*?)\)").unwrap();
        let args = regex.captures(expression)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
            .ok_or_else(|| "Invalid AI expression.")?;
        let arg_array: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
        if arg_array.len() != 1 {
            return Err("Invalid number of arguments for %ai().".into());
        }
        let animatronic = arg_array[0];
        return Err("Unimplemented.".into());
       // engine_data.game.animatronics.get(animatronic)
         //   .and_then(|animatronic_map| animatronic_map.get(&engine_data.game.night)) // TODO: Implement OfficeData
        //.map_or_else(|| Err(format!("Error accessing AI for Animatronic '{}' in game '{}'.", animatronic, engine_data.game.night).into()), |ai_value| Ok(ai_value.clone()))
    }

    fn evaluate_math(&self, expression: &str) -> String {
        let expression = expression.replace("sin", "f64::sin")
            .replace("cos", "f64::cos")
            .replace("tan", "f64::tan");

        expression.parse::<f64>()
            .map_or_else(|e| format!("Error evaluating math expression: {}", e), |result| result.to_string())
    }
}