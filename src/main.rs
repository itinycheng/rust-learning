use std::cell::Cell;

#[allow(dead_code)]
fn main() {
    let name = "tiny";
    println!("Hello, {}.", name);

    // example 1
    let a1: i32 = 5;
    let a2: i32 = 5;
    let b1: u32 = 5;
    assert_eq!(a1,a2);
    assert_eq!(a1 as u32, b1);

    let (a3, mut b3) = (true, false);
    println!("a3={}, b3={}", a3, b3);
    b3 = true;
    let b3 = b3;
    println!("b3={}", b3);

    println!("{}",name.trim());

    let tuple = (1, "str");
    println!("{}", tuple.0);

    let arr = [1,3,5,7];
    println!("arr.first = {}", arr.first().unwrap());

    let middle = &arr[1..3];
    assert_eq!([3,5], middle);
    for item in middle {
        println!("item = {}", item);
    }

    let mut vec1 = vec![1, 2, 3];
    println!("vec pop: {}", vec1.pop().unwrap());
    for item1 in vec1 {
        println!("vec1 item = {}", item1);
    }

    let like = "like";
    let mut wrap = String::from(like);
    println!("like={}, wrap={}", like, wrap);
    wrap.push_str(",");
    wrap.push_str(like);
    println!("wrap={}", wrap);


    let point = Point{x: 0, y: 0};
    println!("point x: {},, y: {}", point.x, point.y);
    let gray = Color(0x11,0x22,0x33);
    let Color(color_x,color_y,color_z) = gray;
    println!("gray x={}, y={}, z={}", color_x, color_y, color_z);
    let digits = vec![1,2,3];
    let vec_digit: Vec<Digit> = digits.into_iter().map(Digit).collect();
    for item in vec_digit {
        println!("vec_digit item = {}", item.0);
    }

    let origin = Point3D::default();
    let point3d = Point3D{y: 1, ..origin};
    println!("point3d x={}, y={}, z={}", point3d.x, point3d.y, point3d.z);
    let Point3D{x: x0, y:y0, ..} = point3d;
    println!("point3d x0={}, y0={}", x0, y0);

    let point2d = Point2D{x:1, y: Cell::from(2)};
    point2d.y.set(3);
    println!("point2d x={}, y={}", point2d.x ,point2d.y.get());

    graph::inside_fn();
    let graph_point = graph::Point::default();
    println!("graph point x={}", graph_point.x);

    let pos = Message::Move{x:2, y:1};
    let write = Message::Write(String::from("hello"));
    let quit = Message::Quit;
    match_message(pos);
    match_message(write);
    match_message(quit);

    let elem: i32 = if a1 == 5 {10} else {3};
    println!("if statement elem = {}", elem);

    for i in 1..5 {
        println!("for statement i = {}", i);
    }

    'outer :loop {
        println!("outer loop");
        'inner: loop{
            println!("inner loop");
            break;
        }
        break;
    }

}

fn match_message(msg: Message) -> () {
    match msg {
        Message::Move{x: m_x, y: m_y} => println!("message move: x={}, y={}", m_x, m_y),
        Message::Write(m_msg) => println!("message write={}", m_msg),
        Message::Quit => println!("message quit"),
    }
}
// struct Empty;

struct Digit(i32);

struct Color(u8,u8,u8);

struct Point{
    x:i32,
    y:i32
}

#[derive(Default)]
struct Point3D{
    x: i32,
    y: i32,
    z: i32
}

struct Point2D{
    x: i32,
    y: Cell<i32>
}

mod graph{

    #[derive(Default)]
    pub struct Point{
        pub x: i32,
        y: i32
    }

    pub fn inside_fn(){
        let point = Point{x: 2, y: 3};
        println!("inside_fn x={}, y={}", point.x, point.y);
    }

}

enum Message{
    Quit,
    Move{ x: i32, y: i32},
    Write(String)
}