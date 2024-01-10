use std::cell::RefCell;
use std::error::Error;

use redis::{Client, Connection};
use crate::common::model::GraphicalModel;
use crate::common::redis::RedisClient;
use super::interface::{FactorModel, ScenarioMaker};
use super::model::Graph;
use crate::model::maxent::ExponentialModel;
use crate::model::{inference::marginalized_inference_probability};

pub fn do_training(
    graph: &Graph,
    model: &mut Box<dyn FactorModel>,
) -> Result<(), Box<dyn Error>> {
    let storage = graph;
    trace!("do_training - Getting all implications");
    let implications = storage.get_all_implications()?;
    for implication in implications {
        trace!("do_training - Processing implication: {:?}", implication);
        model.initialize_connection(&implication)?;
    }

    trace!("do_training - Getting all propositions");
    let propositions = storage.get_training_questions()?;
    trace!(
        "do_training - Processing propositions: {}",
        propositions.len()
    );

    let mut examples_processed = 0;
    for proposition in propositions {
        match model.train(storage, &proposition) {
            Ok(_) => trace!(
                "do_training - Successfully trained on proposition: {:?}",
                proposition
            ),
            Err(e) => {
                panic!(
                    "do_training - Error in train_on_example for proposition {} {:?}: {:?}",
                    examples_processed, proposition, e
                )
            }
        }
        examples_processed += 1;
    }

    trace!(
        "do_training - Training complete: examples processed {}",
        examples_processed
    );
    Ok(())
}

fn run_test_loop(model: &mut GraphicalModel) -> Result<(), Box<dyn Error>> {
    let test_questions = model.graph
        .get_test_questions()
        .expect("Couldn't get test questions.");
    for test_question in &test_questions {
        println!("\n\n\n\n\n\n");
        info!("test_question {:?}", &test_question.search_string());
        let inference_result = marginalized_inference_probability(model, &test_question);
        trace!("inference_result {:?}", &inference_result);
    }

    Ok(())
}

pub fn train_and_test(scenario_maker: &dyn ScenarioMaker) -> Result<(), Box<dyn Error>> {
    let redis_client = RedisClient::new()?;
    let model_spec = "dummy_model_spec".to_string();
    let mut model = GraphicalModel::new(
        &model_spec, &redis_client).expect("Couldn't make storage");
    let result = scenario_maker.setup_scenario(&mut model);
    info!("scenario result: {:?}", result);
    let train_result = do_training(&model.graph, &mut model.model);
    info!("train result: {:?}", train_result);
    run_test_loop(&mut model)?;
    std::mem::drop(model);

    Ok(())
}
