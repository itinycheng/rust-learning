use std::any::Any;
use std::fmt::Debug;

pub fn proc_reflect() {
    println!("load string config {:?}", load_config(&"str".to_string()));
    println!(
        "load string config {:?}",
        load_config(&vec!["str".to_string()])
    );
}

fn load_config<T: Any + Debug>(value: &T) -> Vec<String> {
    let mut cfgs: Vec<String> = vec![];
    let value = value as &dyn Any;
    match value.downcast_ref::<String>() {
        Some(cfg) => cfgs.push(cfg.clone()),
        None => (),
    }
    match value.downcast_ref::<Vec<String>>() {
        Some(vec) => cfgs.extend_from_slice(vec),
        None => {}
    }
    cfgs
}
