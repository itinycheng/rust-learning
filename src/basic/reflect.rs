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
    if let Some(cfg) = value.downcast_ref::<String>() {
        cfgs.push(cfg.clone())
    }
    if let Some(vec) = value.downcast_ref::<Vec<String>>() {
        cfgs.extend_from_slice(vec)
    }
    cfgs
}
