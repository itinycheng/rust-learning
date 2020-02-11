struct Wrap {
    i: i32,
}

impl Wrap {
    fn show(&self) {
        println!("Wrap show item={}", self.i);
    }

    fn add_one(&mut self) {
        self.i += 1;
    }

    fn add_two(&mut self) {
        self.i += 2;
    }
}

pub fn life_time() {
    let mut wrap = Wrap{i: 1};
    wrap.add_one();
    wrap.add_two();
    wrap.show();

}