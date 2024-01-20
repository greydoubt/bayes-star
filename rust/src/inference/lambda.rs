use super::{
    inference::{compute_each_combination, groups_from_backlinks, Inferencer},
    table::{GenericNodeType, PropositionNode},
};
use crate::{
    inference::inference::build_factor_context_for_assignment,
    model::{objects::EXISTENCE_FUNCTION, weights::CLASS_LABELS},
    print_blue, print_green, print_red, print_yellow,
};
use std::error::Error;

impl Inferencer {
    pub fn do_lambda_traversal(&mut self) -> Result<(), Box<dyn Error>> {
        let mut bfs_order = self.bfs_order.clone();
        bfs_order.reverse();
        trace!("send_lambda_messages bfs_order: {:?}", &bfs_order);
        for node in &bfs_order {
            print_yellow!("send pi bfs selects {:?}", node);
            self.lambda_visit_node(node)?;
        }
        Ok(())
    }
    pub fn initialize_lambda(&mut self) -> Result<(), Box<dyn Error>> {
        trace!("initialize_lambda: proposition");
        for node in &self.proposition_graph.all_nodes {
            trace!("initializing: {}", node.debug_string());
            for outcome in CLASS_LABELS {
                self.data.set_lambda_value(node, outcome, 1f64);
            }
            for parent in &self.proposition_graph.get_all_backward(node) {
                trace!(
                    "initializing lambda link from {} to {}",
                    node.debug_string(),
                    parent.debug_string()
                );
                for outcome in CLASS_LABELS {
                    self.data.set_lambda_message(node, parent, outcome, 1f64);
                }
            }
        }
        Ok(())
    }

    pub fn lambda_set_from_evidence(
        &mut self,
        node: &PropositionNode,
    ) -> Result<(), Box<dyn Error>> {
        let as_single = node.extract_single();
        let probability = self
            .fact_memory
            .get_proposition_probability(&as_single)?
            .unwrap();
        print_red!("set from evidence {:?} {}", node, probability);
        self.data.set_lambda_value(node, 1, probability);
        self.data.set_lambda_value(node, 0, 1f64 - probability);
        self.data.print_debug();
        panic!();
        Ok(())
    }

    pub fn lambda_compute_generic(
        &mut self,
        from_node: &PropositionNode,
    ) -> Result<(), Box<dyn Error>> {
        let is_observed = self.is_observed(from_node)?;
        assert!(!is_observed);
        let children = self.proposition_graph.get_all_forward(from_node);
        for class_label in &CLASS_LABELS {
            let mut product = 1f64;
            for (_child_index, child_node) in children.iter().enumerate() {
                let child_lambda = self
                    .data
                    .get_lambda_value(&child_node, *class_label)
                    .unwrap();
                product *= child_lambda;
            }
            self.data
                .set_lambda_value(&from_node, *class_label, product);
        }
        Ok(())
    }

    pub fn lambda_visit_node(&mut self, from_node: &PropositionNode) -> Result<(), Box<dyn Error>> {
        let is_observed = self.is_observed(from_node)?;
        print_yellow!("lambda_visit_node {:?} is_observed {}", from_node, is_observed);
        if is_observed {
            self.lambda_set_from_evidence(from_node)?;
        } else {
            self.lambda_compute_generic(&from_node)?;
        }
        self.lambda_send_generic(from_node)?;
        Ok(())
    }

    pub fn lambda_send_generic(&mut self, node: &PropositionNode) -> Result<(), Box<dyn Error>> {
        let parent_nodes = self.proposition_graph.get_all_backward(node);
        let all_combinations = compute_each_combination(&parent_nodes);
        let mut sum_true = 0f64;
        let mut sum_false = 0f64;
        for combination in &all_combinations {
            let mut product = 1f64;
            for (index, parent_node) in parent_nodes.iter().enumerate() {
                let boolean_outcome = combination.get(parent_node).unwrap();
                let usize_outcome = if *boolean_outcome { 1 } else { 0 };
                let pi_x_z = self
                    .data
                    .get_pi_message(parent_node, node, usize_outcome)
                    .unwrap();
                trace!(
                    "getting pi message parent_node {:?}, node {:?}, usize_outcome {}, pi_x_z {}",
                    &parent_node,
                    &node,
                    usize_outcome,
                    pi_x_z,
                );
                product *= pi_x_z;
            }
            let true_marginal =
                self.score_factor_assignment(&parent_nodes, combination, node)?;
            let false_marginal = 1f64 - true_marginal;
            let false_marginal = 1f64 - true_marginal;
            sum_true += true_marginal * product;
            sum_false += false_marginal * product;
        }
        self.data.set_lambda_value(node, 1, sum_true);
        self.data.set_lambda_value(node, 0, sum_false);
        Ok(())
    }
}
