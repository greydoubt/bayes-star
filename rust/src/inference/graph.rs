use std::{
    collections::{HashMap, HashSet},
    error::Error,
    rc::Rc,
};

use serde::{Deserialize, Serialize};

use crate::{
    common::{graph::InferenceGraph, redis::RedisManager},
    model::{
        choose::{compute_search_predicates, extract_backimplications_from_proposition},
        objects::{GroupRoleMap, PredicateInferenceFactor, Proposition, PropositionGroup},
    },
};

fn proposition_implication_from(
    implication: &PredicateInferenceFactor,
    proposition: &Proposition,
) -> Result<PropositionInferenceFactor, Box<dyn Error>> {
    todo!()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PropositionInferenceFactor {
    pub premise: PropositionGroup,
    pub conclusion: Proposition,
    pub inference: PredicateInferenceFactor,
}

pub struct PropositionGraph {
    pub predicate_graph: Rc<InferenceGraph>,
    pub single_forward: HashMap<Proposition, PropositionGroup>,
    pub single_backward: HashMap<Proposition, PropositionGroup>,
    pub group_forward: HashMap<PropositionGroup, Proposition>,
    pub roots: HashSet<Proposition>,
}

fn initialize_visit_single(
    graph: &mut PropositionGraph,
    single: &Proposition,
) -> Result<(), Box<dyn Error>> {
    let inference_factors =
        extract_backimplications_from_proposition(&graph.predicate_graph, single)?;
    if inference_factors.len() == 0 {
        graph.roots.insert(single.clone());
    } else {
        for inference_factor in &inference_factors {
            graph.single_backward.insert(
                inference_factor.conclusion.clone(),
                inference_factor.premise.clone(),
            );
            graph.group_forward.insert(
                inference_factor.premise.clone(),
                inference_factor.conclusion.clone(),
            );
            for term in &inference_factor.premise.terms {
                graph
                    .single_forward
                    .insert(term.clone(), inference_factor.premise.clone());
                initialize_visit_single(graph, term)?;
            }
        }
    }
    Ok(())
}

impl PropositionGraph {
    pub fn new_shared(
        predicate_graph: Rc<InferenceGraph>,
        target: &Proposition,
    ) -> Result<Rc<PropositionGraph>, Box<dyn Error>> {
        let mut graph = PropositionGraph {
            predicate_graph,
            single_forward: HashMap::new(),
            single_backward: HashMap::new(),
            group_forward: HashMap::new(),
            roots: HashSet::new(),
        };
        initialize_visit_single(&mut graph, target)?;
        Ok(Rc::new(graph))
    }
}

pub fn compute_forward_graph(
    predicate_graph: Rc<InferenceGraph>,
    proposition: &Proposition,
) -> Result<PropositionGraph, Box<dyn Error>> {
    todo!()
}

pub fn compute_backward_graph(
    predicate_graph: Rc<InferenceGraph>,
    proposition: &Proposition,
) -> Result<PropositionGraph, Box<dyn Error>> {
    todo!()
}
