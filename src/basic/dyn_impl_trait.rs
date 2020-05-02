pub fn proc_dyn_impl_trait() {
    let plus_two = plus(2);
    println!("1 plus 2 eq {}", plus_two(1));

    let minus_two = minus(2);
    println!("3 minus 2 eq {}", minus_two(3));

    let multi_two = multi(2);
    println!("2 multi 2 eq {}", multi_two.multi(2));

    let div_two = div(2);
    println!("4 div 2 eq {}", div_two.div(4));

    let trait_object = Div::trait_object(4, 2);
    println!("trait object {}", trait_object);

    let number = Number(3);
    println!("operate_1(3, 2) eq {}", operate_1(number, 2));
    println!("operate_2(3, 2) eq {}", operate_2(number, 2));

    // specify type of return value to `Number`
    let number = new_multi_object::<Number>(3);
    // coding diff between operate_1 and operate_2;
    println!(
        "operate_2::<Number>(3, 2) eq {}",
        operate_2::<Number>(number, 2)
    );
    // pub fn parse<F>(&self) -> Result<F, <F as FromStr>::Err> where F: FromStr,

    let nums: [&dyn Multiply; 2] = [&Number(1), &Digit(2)];
    for (i, num) in nums.iter().enumerate() {
        println!("nums #{}, multi 2 eq {} ", i, num.multi(2));
    }
}

#[derive(Debug, Copy, Clone)]
struct Number(i32);

#[derive(Debug, Copy, Clone)]
struct Digit(i32);

trait Multiply {
    fn multi(&self, num: i32) -> i32;

    /// must Sized if need dyn trait
    fn new(num: i32) -> Self
    where
        Self: Sized;
}

trait Div {
    fn div(&self, num: i32) -> i32;
}

trait Mod {
    fn mod_func(num1: i32, num2: i32) -> i32 {
        num1 % num2
    }
}

impl Mod for Number {}

impl Multiply for Number {
    fn multi(&self, num: i32) -> i32 {
        self.0 * num
    }

    fn new(num: i32) -> Self {
        Number(num)
    }
}

// vtable ?
impl dyn Div {
    fn trait_object(num1: i32, num2: i32) -> i32 {
        num1 / num2
    }
}

impl Div for Number {
    fn div(&self, num: i32) -> i32 {
        num / self.0
    }
}

impl Multiply for Digit {
    fn multi(&self, num: i32) -> i32 {
        self.0 * num
    }

    fn new(num: i32) -> Self {
        Digit(num)
    }
}

fn plus(num: i32) -> impl Fn(i32) -> i32 {
    move |x: i32| x + num
}

fn minus(num: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x: i32| x - num)
}

fn multi(num: i32) -> impl Multiply {
    Number(num)
}

fn div(num: i32) -> Number {
    Number(num)
}

fn operate_1(opr: impl Multiply, num: i32) -> i32 {
    opr.multi(num)
}

fn operate_2<T: Multiply>(opr: T, num: i32) -> i32 {
    opr.multi(num)
}

// not work
fn new_multi_object<T>(num: i32) -> T
where
    T: Multiply,
{
    T::new(num)
}
