use std::collections::HashMap;

pub fn proc_collection() {
    vec_type();
    hash_map();
}

pub fn proc_iterator() {
    // 惰性计算
    let seq = (1..).into_iter();
    let collected = seq
        .take_while(|x| *x < 10)
        .map(|x| x * x)
        .filter(|x| x % 2 == 0)
        .collect::<Vec<i32>>()
        .iter()
        .fold(0, |x, y| x + *y);
    println!("iterator collected {:?}", collected);
    let sum = (1..=4).fold(0, |x, y| x + y);
    println!("iterator sum = {}", sum);

    let names = vec!["tiny", "bill", "free"];
    let mut scores = vec![10, 20, 30];
    let map: HashMap<_, _> = names.iter().zip(scores.iter()).collect();
    println!("iterator map result {:?}", map);
    names
        .iter()
        .enumerate()
        .filter(|&(index, _)| index % 2 == 0)
        .map(|(_, &item)| item)
        .take(2)
        .for_each(|x| println!("iterator name {}", x));

    scores.reverse();
    println!("{:?}", scores);
}

fn hash_map() {
    let mut map: HashMap<&str, i32> = HashMap::with_capacity(5);
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    map.remove("a");
    let value = map.get("b").unwrap_or(&0);
    println!("map value {}", value);
    for (key, value) in &map {
        println!("map key={}, value={}", key, value);
    }
    let mut map = HashMap::new();
    for c in "dfasfasfsfsdf".chars() {
        let value = map.entry(c).or_insert(0);
        *value += 1
    }
    println!("{:?}", map);
}

fn vec_type() {
    let mut v1: Vec<f64> = Vec::new();
    v1.push(3 as f64);
    println!("v1 {:?}", v1);
    let v2: Vec<i32> = vec![];
    println!("v2 {:?}", v2);
    let v3 = vec![1, 2, 3];
    println!("{:?}", v3);
    let v4 = vec![2; 10];
    println!("v4 {:?}", v4);
    let mut v5: Vec<i32> = (1..=5).collect();
    println!("v5 {:?}", v5);
    // 不推荐通过下标访问，容易导致越界使得线程panic
    let item0 = v5[0];
    // 使用get可以做到安全访问
    let item1 = v5.get(1).unwrap();
    println!("item0= {} {}", item0, item1);
    for x in &mut v5 {
        println!("for mut vec {}", x);
    }
    // 预分配空间，减少数组扩容带来的数据拷贝，类似java ArrayList
    let mut v6 = vec![1];
    v6.reserve(1000);
    println!("v6 len={}, capacity={}", v6.len(), v6.capacity());
}
