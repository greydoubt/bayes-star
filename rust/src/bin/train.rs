use bayes_star::common::setup::common_setup;
use bayes_star::common::{run::setup_and_train, resources::FactoryResources};
use bayes_star::scenarios::dating_simple::SimpleDating;

#[macro_use]
extern crate log;

fn main() {
    let config = common_setup();
    let resources = FactoryResources::new(&config).expect("Couldn't create resources.");
    let scenario_maker = SimpleDating {};
    setup_and_train(&resources, &scenario_maker).expect("Error in training.");
    warn!("program done");
}
