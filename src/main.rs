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
}
