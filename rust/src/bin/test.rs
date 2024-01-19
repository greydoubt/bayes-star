use bayes_star::common::setup::parse_configuration_options;
use bayes_star::common::resources::FactoryResources;
use bayes_star::common::test::do_testing;

#[macro_use]
extern crate log;

fn main() {
    let config: bayes_star::common::setup::ConfigurationOptions = parse_configuration_options();
    let resources = FactoryResources::new(&config).expect("Couldn't create resources.");
    let test_example = config.test_example;
    do_testing(&resources).expect("Testing failed.");
    warn!("program done");
}
