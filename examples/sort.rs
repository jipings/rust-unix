#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Person {
            name,
            age,
        }
    }
}

fn main() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];
    // 自然排序
    people.sort();
    println!("{:?}", people);

    people.sort_by(|a, b| b.age.cmp(&a.age));
    println!("{:?}", people);

}