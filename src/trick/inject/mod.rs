use self::context::Data;

pub mod context;
pub mod factory;

#[derive(Default, Debug, Clone)]
struct Id {
    id: u32,
}

#[derive(Default, Debug, Clone)]
struct Name<'a> {
    name: &'a str,
}

fn print_args(id: Data<Id>, name: Data<Option<Name>>) {
    println!("id: {:?}, name: {:?}", id, name);
}

fn print_no_args() {
    println!("{:?}", "no args")
}

#[cfg(test)]
mod tests {
    use crate::trick::inject::{
        context::AppContext, factory::Factory, print_args, print_no_args, Id,
    };

    #[test]
    fn test_inject_data() {
        let mut ctx = AppContext::new();
        ctx.store(Id { id: 1 });
        // ctx.store(Name { name: "tiny" });

        print_args.call_with_context(&ctx);
        print_no_args.call_with_context(&ctx);

        let a_fn = print_args as *const ();
        let b_fn = print_args.clone() as *const ();
        println!("{:p}, {:p}", a_fn, b_fn);
    }
}
