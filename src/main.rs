use rust_solid_design::solid_principle::{
    single_responsibility::SingleMain,
    open_closed::OpenMain,
    liskov_substitution::LiskovMain,
    interface_segregation::InterfaceMain,
    dependency_inversion::DependMain,
};
use rust_solid_design::design_pattern_part1::{
    template_method::TemplateMain,
    iterator::IteratorMain,
    facade::FacadeMain,
    //factory_method::FactoryMain,
    adapter_delegation::AdapterDeleMain,
};
use rust_solid_design::design_pattern_part2::{
    strategy::StrategyMain,
    state::StateMain,
};

fn main() {
    println!("--- SingleMain ---");
    SingleMain::index();
    println!("---OpenMain ---");
    OpenMain::index();
    println!("--- LiskovMain ---");
    LiskovMain::index();
    println!("--- InterfaceMain ---");
    InterfaceMain::index();
    println!("--- DependMain ---");
    DependMain::index();
    println!("--- TemplateMain ---");
    TemplateMain::index();
    println!("--- IteratorMain ---");
    IteratorMain::index();
    println!("--- FacadeMain ---");
    FacadeMain::index();
    //println!("--- FactoryMain ---");
    //FactoryMain::index();
    println!("--- AdapterDeleMain ---");
    AdapterDeleMain::index();
    println!("--- StrategyMain ---");
    StrategyMain::index();
    println!("--- StateMain ---");
    StateMain::index();
}
