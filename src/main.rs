fn main() {
    println!("Hello, world!");
    let life: i32 =42;
    println!("The answer to the Ultimate Question of Life, The Universe, and Everything is: {}", life);
    println!("{:?}", life);

    println!("{life}");
    println!("{life:?}");

    //if else practice
    let number = 70;
    if number < 50 {
        println!("The number is less than 50");
    } else {
        println!("The number is greater than 50");
}

//nested if else practice
    let number = 210;
    if number > 100{
      if number > 200 {
        println!("The number is huge");
      }else {
        println!("The number is big");
      } 
    } else {
        println!("The number is small");
    }
//else if practice
    let a = 99;
    if a > 200 {
        println!("The number is huge");
    } else if a > 99 {
        println!("The number is big");
    } else {
        println!("The number is small");
    }
    //the code that will not work
    if a > 99 {
        println!("The number is big");
    } else if a > 200 {
        println!("The number is huge");
    } else {
        println!("The number is small");
    }
//match practice
    let  number = 3;
    match number {
        1 => println!("The number is one"),
        2 => println!("The number is two"),
        3 => println!("The number is three"),
        _ => println!("The number is something else"),
    }
//ownership practice
    let s1 = String::from("Hello");
    let s2 = s1; //s1 is moved to s2, s1 is no longer valid
    println!("{}", s2);
}
