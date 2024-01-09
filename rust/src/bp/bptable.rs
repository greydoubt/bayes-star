use std::{collections::HashMap, error::Error};

use redis::Connection;

use crate::model::{objects::Proposition, weights::CLASS_LABELS, storage::PropositionProbability};

pub struct BeliefPropagationData {
    pi_values: HashMap<(String, usize), f64>,
    lambda_values: HashMap<(String, usize), f64>,
    pi_messages: HashMap<(String, String, usize), f64>,
    lambda_messages: HashMap<(String, String, usize), f64>,
}

impl BeliefPropagationData {
    // Constructor to create a new instance
    pub fn new() -> Self {
        BeliefPropagationData {
            pi_values: HashMap::new(),
            lambda_values: HashMap::new(),
            pi_messages: HashMap::new(),
            lambda_messages: HashMap::new(),
        }
    }

    // Getter for pi values
    pub fn get_pi_value(&self, node: &Proposition, outcome: usize) -> Option<f64> {
        let key = (node.search_string(), outcome);
        self.pi_values.get(&key).cloned()
    }

    // Setter for pi values
    pub fn set_pi_value(&mut self, node: &Proposition, outcome: usize, value: f64) {
        let key = (node.search_string(), outcome);
        self.pi_values.insert(key, value);
    }

    // Getter for lambda values
    pub fn get_lambda_value(&self, node: &Proposition, outcome: usize) -> Option<f64> {
        let key = (node.search_string(), outcome);
        self.lambda_values.get(&key).cloned()
    }

    // Setter for lambda values
    pub fn set_lambda_value(&mut self, node: &Proposition, outcome: usize, value: f64) {
        let key = (node.search_string(), outcome);
        self.lambda_values.insert(key, value);
    }

    // Getter for pi messages
    pub fn get_pi_message(
        &self,
        from: &Proposition,
        to: &Proposition,
        outcome: usize,
    ) -> Option<f64> {
        let key = (from.search_string(), to.search_string(), outcome);
        self.pi_messages.get(&key).cloned()
    }

    // Setter for pi messages
    pub fn set_pi_message(
        &mut self,
        from: &Proposition,
        to: &Proposition,
        outcome: usize,
        value: f64,
    ) {
        let key = (from.search_string(), to.search_string(), outcome);
        self.pi_messages.insert(key, value);
    }

    // Getter for lambda messages
    pub fn get_lambda_message(
        &self,
        from: &Proposition,
        to: &Proposition,
        outcome: usize,
    ) -> Option<f64> {
        let key = (from.search_string(), to.search_string(), outcome);
        self.lambda_messages.get(&key).cloned()
    }

    // Setter for lambda messages
    pub fn set_lambda_message(
        &mut self,
        from: &Proposition,
        to: &Proposition,
        outcome: usize,
        value: f64,
    ) {
        let key = (from.search_string(), to.search_string(), outcome);
        self.lambda_messages.insert(key, value);
    }
}

pub struct BeliefPropagator {
    data: BeliefPropagationData,
}

impl BeliefPropagator {
    // Initialize new Storage with a Redis connection
    pub fn new() -> Result<Self, redis::RedisError> {
        Ok(BeliefPropagator {
            data: BeliefPropagationData::new(),
        })
    }

    pub fn initialize(&mut self, evidence:&dyn PropositionProbability) -> Result<(), Box<dyn Error>> {
        self.initialize_lambda(evidence)?;
        self.initialize_lambda(evidence)?;
        Ok(())
    }

    pub fn initialize_pi(&mut self, evidence:&dyn PropositionProbability) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn initialize_lambda(&mut self, evidence:&dyn PropositionProbability) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub fn get_all_propositions(&self) -> Result<Vec<Proposition>, Box<dyn Error>> {
        // Your implementation here
        todo!()
    }

    pub fn get_proposition_probability(
        &self,
        proposition: &Proposition,
    ) -> Result<f64, Box<dyn Error>> {
        // Your implementation here
        todo!()
    }

    pub fn get_conditional_probability(
        &self,
        conclusion: &Proposition,
        premise: &Proposition,
    ) -> Result<f64, Box<dyn Error>> {
        // Your implementation here
        todo!()
    }

    fn find_parent(&self, x: &Proposition) -> Result<Option<Proposition>, Box<dyn Error>> {
        // Your implementation here
        Ok(None) // Placeholder
    }

    fn find_root(&self) -> Result<Proposition, Box<dyn Error>> {
        // Your implementation here
        todo!()
    }

    fn find_children(&self, root: &Proposition) -> Result<Vec<Proposition>, Box<dyn Error>> {
        // Your implementation here
        Ok(Vec::new()) // Placeholder
    }
}
