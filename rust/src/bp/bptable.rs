use std::{collections::HashMap, error::Error};

use redis::Connection;

use crate::model::{objects::Proposition, weights::CLASS_LABELS};

pub struct BeliefPropagationData {
    pi_values: HashMap<String, f64>,
    lambda_values: HashMap<(String, usize), f64>,
}

impl BeliefPropagationData {
    // Constructor to create a new instance
    pub fn new() -> Self {
        BeliefPropagationData {
            pi_values: HashMap::new(),
            lambda_values: HashMap::new(),
        }
    }

    // Getter for pi values
    pub fn get_pi_value(&self, key: &Proposition) -> Option<f64> {
        self.pi_values.get(&key.search_string()).cloned()
    }

    // Setter for pi values
    pub fn set_pi_value(&mut self, key: Proposition, value: f64) {
        self.pi_values.insert(key.search_string(), value);
    }

    // Getter for lambda values
    pub fn get_lambda_value(&self, node:&Proposition, outcome:usize) -> Option<f64> {
        let key = (node.search_string(), outcome);
        self.lambda_values.get(&key).cloned()
    }

    // Setter for lambda values
    pub fn set_lambda_value(&mut self, node:&Proposition, outcome:usize, value: f64) {
        let key = (node.search_string(), outcome);
        self.lambda_values.insert(key, value);
    }
}

pub struct BeliefPropagator {
    data: BeliefPropagationData,
}

impl Drop for BeliefPropagator {
    fn drop(&mut self) {
        // The Drop trait for Arc<Client> will automatically be called here,
        // reducing the reference count. If this Storage instance holds the last
        // reference to the client, the client will be dropped and its resources
        // (like network connections) will be cleaned up.
    }
}

impl BeliefPropagator {
    // Initialize new Storage with a Redis connection
    pub fn new() -> Result<Self, redis::RedisError> {
        Ok(BeliefPropagator {
            data: BeliefPropagationData::new(),
        })
    }

    pub fn initialize(&mut self) -> Result<(), Box<dyn Error>> {
        let all_propositions = self.get_all_propositions()?;

        // Initialize lambda values
        for x in all_propositions.iter() {
            for x_value in CLASS_LABELS {
                self.data.set_lambda_value(x, x_value, 1.0);
            }

            // Initialize parent lambda values
            if let Some(parent) = self.find_parent(x)? {
                for z_value in CLASS_LABELS {
                    self.data.set_lambda_value(&parent, z_value, 1.0);
                }
            }
        }

        // Initialize pi values for the root
        let root = self.find_root()?;
        let probability = self.get_proposition_probability(&root)?;
        self.data.set_pi_value(root.clone(), probability);

        // Send pi messages to children of the root
        for child in self.find_children(&root)? {
            self.send_pi_msg(&root, &child)?;
        }

        Ok(())
    }

    // have to depend on graph, and Propositions -> values
    pub fn update_tree(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    // A stub implementation for `send_pi_msg`.
    pub fn send_pi_msg(
        &mut self,  // Changed to mutable reference
        from: &Proposition,
        to: &Proposition,
    ) -> Result<(), Box<dyn Error>> {
        // Get the pi value for the 'from' Proposition using the new interface
        let from_pi = self.data.get_pi_value(from).expect("Value not found in map");

        // Get the conditional probability of 'to' given 'from'
        let conditional_probability = self.get_conditional_probability(from, to)?;

        // Calculate the new pi value for 'to'
        let to_pi = from_pi * conditional_probability;

        // Update the pi value for 'to' using the new interface
        self.data.set_pi_value(to.clone(), to_pi);  // Assuming Proposition is Cloneable

        // Update lambda values for 'to'
        for value_index in CLASS_LABELS {
            // Fetch the current lambda value, defaulting to 1 if not present
            let current_lambda = self.data.get_lambda_value(to, value_index).unwrap_or(1.0);

            // Combine the existing lambda value with the new pi value
            let new_lambda = current_lambda * to_pi;

            // Set the new lambda value
            self.data.set_lambda_value(to, value_index, new_lambda);
        }

        Ok(())
    }

    // A stub implementation for `send_lambda_message`.
    pub fn send_lambda_message(
        &self,
        from: &Proposition,
        to: &Proposition,
        lambda_values: &mut HashMap<(String, usize), f64>,
        pi_values: &mut HashMap<String, f64>,
    ) -> Result<(), Box<dyn Error>> {
        // Get the lambda value for the 'from' Proposition.
        let mut from_lambda = 1.0;
        for value_index in CLASS_LABELS.iter() {
            let lambda_key = (from.search_string(), *value_index);
            from_lambda *= lambda_values
                .get(&lambda_key)
                .ok_or_else(|| "Lambda value for 'from' Proposition not found")?;
        }

        // Get the conditional probability of 'from' given 'to'.
        // This function `get_conditional_probability` is assumed to be defined elsewhere.
        let conditional_probability = self.get_conditional_probability(to, from)?;

        // Calculate the new lambda value for 'to'.
        // The lambda value is a product of the lambda value from 'from' and the conditional probability.
        let to_lambda = from_lambda * conditional_probability;

        // Update the lambda value for 'to' in `lambda_values`.
        // This is a simplified version, assuming binary propositions.
        for value_index in CLASS_LABELS.iter() {
            let lambda_key = (to.search_string(), *value_index);
            if let Some(lambda) = lambda_values.get_mut(&lambda_key) {
                // Combine the existing lambda value with the new lambda value.
                // This is a placeholder for the actual combination logic, which will depend on your specific use case.
                *lambda *= to_lambda;
            } else {
                // If there is no lambda value for 'to', insert a new one.
                lambda_values.insert(lambda_key, to_lambda);
            }
        }

        // Assuming pi_values need to be updated with new lambda values.
        // Here we adjust the pi_values for 'to' Proposition, though the specific update rule will depend on your use case.
        let to_pi = pi_values
            .get(&to.search_string())
            .ok_or_else(|| "Pi value for 'to' Proposition not found")?;
        pi_values.insert(to.search_string(), to_pi * to_lambda);

        Ok(())
    }
    pub fn get_all_propositions(&self) -> Result<Vec<Proposition>, Box<dyn Error>> {
        // Your implementation here
        todo!()
    }

    pub fn get_proposition_probability(&self, proposition: &Proposition) -> Result<f64, Box<dyn Error>> {
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