use std::error::Error;
use std::sync::{Arc, Mutex};
use libc::rand;
use rand::{random, Rng, thread_rng};
use regex::Regex;
use serde_json::Value;

use crate::GameLoader::Code;

include!("MathEvaluator.rs");
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
        self.logger.log("Event Manager", format!("Registering Event: {:0} With args: {:?}", event_name, args).as_str());
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
        self.logger.log("Event Manager", format!("Killed {} Listeners", count).as_str());
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
        if let Some(func) = self.code_blocks.get(&code.block.to_lowercase()) {
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

    pub fn parse_expression(&self, expression: &str, engine_data: &mut EngineData) -> Result<String, Box<dyn Error>> {
        let mut result = expression.to_string();

        let re_random = Regex::new(r"%random\(([^,]+),([^)]+)\)").unwrap();
        let mut rng = rand::thread_rng();

        while re_random.is_match(&result) {
            result = re_random.replace_all(&result, |caps: &regex::Captures| {
                let a = self.get_expr(caps.get(1).unwrap().as_str(), engine_data).trim().parse::<i32>().unwrap();
                let b = self.get_expr(caps.get(2).unwrap().as_str(), engine_data).trim().parse::<i32>().unwrap();
                rng.gen_range(a..b).to_string()
            }).to_string();
        }

        let patterns = [
            ("var", r"%var\((.*?)\)"),
            ("data", r"%data\((.*?)\)"),
            ("math", r"%math\((.*?)\)"),
            ("ai", r"%ai\((.*?)\)"),
            ("mouse", r"%mouse\((.*?)\)"),
            ("game", r"%game\((.*?)\)")
        ];

        for (expr_type, pattern) in &patterns {
            let re = Regex::new(pattern).unwrap();

            while re.is_match(&result) {
                result = re.replace_all(&result, |caps: &regex::Captures| {
                    let content = caps.get(1).unwrap().as_str();
                    let replacement = match *expr_type {
                        "var" => self.variables.get(content).cloned().ok_or_else(|| format!("Variable '{}' not found.", content)),
                        "data" => self.data_values.get(content).cloned().ok_or_else(|| format!("Data Value '{}' not found.", content)),
                        "math" => Ok(self.evaluate_math_expression(content, engine_data).unwrap()),
                        "ai" => Ok(self.evaluate_ai_expression(content, engine_data).unwrap().to_string()),
                        "mouse" => Err("Mouse expressions are not implemented".to_string()),
                        "game" => Err("Game expressions are not implemented".to_string()),
                        _ => Err(format!("Unknown expression type: {}", expr_type)),
                    };

                    replacement.unwrap_or_else(|err| err.to_string())
                }).to_string();
            }
        }

        Ok(result)
    }

    fn set_variable_value(&mut self, name: String, data: String) {
        self.variables.insert(name, data);
    }

    fn set_data_value(&mut self, name: String, data: String) {
        self.data_values.insert(name, data);
    }

    pub fn evaluate_math_expression(&self, expression: &str, engine_data: &mut EngineData) -> Result<String, Box<dyn Error>> {
        match MathEvaluator::evaluate(expression) {
            Ok(result) => Ok(result.to_string()),
            Err(e) => {
                engine_data.logger.log_error("EventManager", format!("Unknown math expression: {}", expression).as_str());
                Ok("MathErr".to_string())
            },
        }
    }

    fn evaluate_ai_expression(&self, expression: &str, engine_data: &mut EngineData) -> Result<i32, Box<dyn std::error::Error>> {
  //      let regex = Regex::new(r"%ai\((.*?)\)").unwrap();
  //      let args = regex.captures(expression)
  //          .and_then(|caps| caps.get(1))
   //         .map(|m| m.as_str())
    //        .ok_or_else(|| "Invalid AI expression.").unwrap();
    //    let arg_array: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
    //    if arg_array.len() != 1 {
   //         //  Err("Invalid number of arguments for %ai().".into()).expect("etghedtshwjhewtrsh");
  //      }
   //     let animatronic = arg_array[0];
        return Ok(0);
        // TODO: make compiler happy
        // engine_data.game.animatronics.get(animatronic)
        //      .and_then(|animatronic_map| animatronic_map.AI.clone().expect("Error accessing AI vector for Animatronic").get(engine_data.officemgr.Office.night as usize))
        //      .map_or_else(|| Err(format!("Error accessing AI value for Animatronic '{}' for night '{}'.", animatronic, &engine_data.officemgr.Office.night).into()), |ai_value: &i32| Ok(ai_value.clone()))
    }

    fn evaluate_math(&self, expression: &str) -> String {
        let expression = expression.replace("sin", "f64::sin")
            .replace("cos", "f64::cos")
            .replace("tan", "f64::tan");

        expression.parse::<f64>()
            .map_or_else(|e| format!("Error evaluating math expression: {}", e), |result| result.to_string())
    }
}