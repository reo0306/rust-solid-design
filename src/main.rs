use rust_solid_design::solid_principle::{
    single_responsibility::SingleMain,
    open_closed::OpenMain,
    liskov_substitution::LiskovMain,
    interface_segregation::InterfaceMain,
    dependency_inversion::DependMain,
};
use rust_solid_design::design_pattern_part1::{
    template_method::TemplateMain,
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
}
