fn main() {
    let mut v = vec![1, 2, 3];
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{:?}", v);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];

    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("there is no third element"),
    }

    let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[10];  // panic !!!
    let does_not_exist = v.get(10);
    println!("{:?}", does_not_exist);

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    // v.push(6); // Error
    println!("The first element is: {}", first);

    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row[0]);
}
