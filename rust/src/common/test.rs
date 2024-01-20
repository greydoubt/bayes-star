use std::{collections::HashMap, error::Error, io, rc::Rc};

use crate::{
    common::{
        graph::InferenceGraph, model::InferenceModel, proposition_db::{RedisBeliefTable, EmptyBeliefTable, HashMapBeliefTable},
        train::TrainingPlan,
    },
    inference::{
        graph::PropositionGraph,
        inference::{inference_compute_marginals, Inferencer},
        table::PropositionNode,
    },
    model::{
        exponential::ExponentialModel,
        objects::{Proposition, EXISTENCE_FUNCTION},
    },
    print_blue, print_green, print_red, print_yellow,
};

use super::{interface::BeliefTable, resources::FactoryResources, setup::ConfigurationOptions};

struct ReplState {
    inferencer: Box<Inferencer>,
    fact_memory: Rc<dyn BeliefTable>,
    /// Relative set by the `print_ordering` last time it serialized an ordering.
    question_index: HashMap<u64, PropositionNode>,
}

impl ReplState {
    pub fn new(mut inferencer: Box<Inferencer>) -> ReplState {
        let fact_memory = HashMapBeliefTable::new();
        inferencer.fact_memory = fact_memory.clone();
        ReplState {
            inferencer,
            fact_memory,
            question_index: HashMap::new(),
        }
    }
    fn do_repl_loop(&mut self)  -> Result<(), Box<dyn Error>> {
        loop {
            self.inferencer.data.print_debug();
            self.inferencer.update_marginals()?;
            self.print_ordering()?;
            let tokens = get_input_tokens_from_user();
            println!("tokens {:?}", tokens);
            let function = &tokens[0];
            match function.as_str() {
                "set" => {
                    self.handle_set(&tokens);
                },
                "reinit" => {
                    self.inferencer.initialize_chart()?;
                },
                "pass" => {
                    self.inferencer.do_full_forward_and_backward()?;
                },
                "quit" => break,
                _ => println!("Command not recognized."),
            };
        }
        Ok(())
    }

    fn handle_set(&mut self, tokens: &Vec<String>) {
        let select_index = tokens[1].parse::<u64>().unwrap();
        let new_prob = tokens[2].parse::<f64>().unwrap();
        let node = self.question_index.get(&select_index).unwrap();
        let prop = node.extract_single();
        self.fact_memory.store_proposition_probability(&prop, new_prob).unwrap();
        self.inferencer.do_fan_out_from_node(&node).unwrap();
    }

    fn print_ordering(&mut self) -> Result<(), Box<dyn Error>> {
        let bfs = self.inferencer.proposition_graph.get_bfs_order();
        self.question_index.clear();
        for (index, node) in bfs.iter().enumerate() {
            if node.is_single() {
                let single = node.extract_single();
                let probability = self.fact_memory.get_proposition_probability(&single)?;
                info!("node {} {:?} {:?}", index, &node, probability);
                self.question_index.insert(index as u64, node.clone());
            } else {
                // print_green!("node {} {:?} *", index, &node);
            }
        }
        Ok(())
    }
}

pub fn get_input_tokens_from_user() -> Vec<String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let trimmed = input.trim();
    let tokens: Vec<String> = trimmed.split_whitespace().map(|s| s.to_string()).collect();
    tokens
}

pub fn interactive_inference_example(
    config: &ConfigurationOptions,
    resources: &FactoryResources,
) -> Result<(), Box<dyn Error>> {
    let plan = TrainingPlan::new(&resources.redis)?;
    let graphical_model = InferenceModel::new_shared(&resources)?;
    info!("do_training - Getting all implications");
    let plan = TrainingPlan::new(&resources.redis)?;
    let model = InferenceModel::new_shared(&resources).unwrap();
    // test
    let test_questions = plan.get_test_questions().unwrap();
    let target = &test_questions[config.test_example.unwrap() as usize];
    info!("testing proposition {:?}", &target.hash_string());
    let fact_memory = EmptyBeliefTable::new_shared(&resources.redis)?;
    let proposition_graph = PropositionGraph::new_shared(model.graph.clone(), target)?;
    proposition_graph.visualize();
    let mut inferencer =
        Inferencer::new_mutable(model.clone(), proposition_graph.clone(), fact_memory)?;
    inferencer.initialize_chart()?;
    let mut repl = ReplState::new(inferencer);
    repl.do_repl_loop()?;
    info!("done");
    Ok(())
}

pub fn summarize_examples(
    config: &ConfigurationOptions,
    resources: &FactoryResources,
) -> Result<(), Box<dyn Error>> {
    let plan = TrainingPlan::new(&resources.redis)?;
    let graphical_model = InferenceModel::new_shared(&resources)?;
    let model = InferenceModel::new_shared(&resources).unwrap();
    // test
    let test_questions = plan.get_test_questions().unwrap();
    for (index, proposition) in test_questions.iter().enumerate() {
        info!("testing proposition {:?}", &proposition.hash_string());
    }
    Ok(())
}
