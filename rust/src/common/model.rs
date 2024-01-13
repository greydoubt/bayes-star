use crate::{
    common::interface::FactDB,
    model::{
        self,
        maxent::ExponentialModel,
        objects::{PredicateConjunction, Domain, Entity, PredicateImplication, Predicate, ImplicationInstance, Proposition},
    },
};
use redis::{Commands, Connection};
use std::{cell::RefCell, error::Error};

use super::{
    interface::{PredictStatistics, TrainStatistics},
    redis::RedisManager, graph::PredicateGraph, fact_db::RedisFactDB, resources::FactoryResources,
};

pub struct GraphicalModel {
    pub graph: PredicateGraph,
    pub model: Box<dyn FactorModel>,
    pub fact_db: Box<dyn FactDB>,
}

impl GraphicalModel {
    pub fn new(resources: &FactoryResources) -> Result<Self, Box<dyn Error>> {
        let graph = PredicateGraph::new(&resources.redis)?;
        let model = ExponentialModel::new(&resources)?;
        let fact_db = RedisFactDB::new(&resources.redis)?;
        Ok(GraphicalModel {
            graph,
            model,
            fact_db,
        })
    }
}

#[derive(Debug)]
pub struct  Factor {
    pub conjuncts: Vec<ImplicationInstance>,
    pub conclusion: Proposition,
}

#[derive(Debug)]
pub struct  FactorContext{
    pub factor: Factor,
    pub conjunct_probabilities: Vec<f64>,
}

pub trait FactorModel {
    fn initialize_connection(&mut self, implication: &PredicateImplication) -> Result<(), Box<dyn Error>>;
    fn train(
        &mut self,
        factor: &FactorContext,
        probability: f64,
    ) -> Result<TrainStatistics, Box<dyn Error>>;
    fn predict(&self, factor: &FactorContext) -> Result<PredictStatistics, Box<dyn Error>>;
}