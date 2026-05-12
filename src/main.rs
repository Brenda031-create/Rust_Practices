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
    println!("s2: {}", s2);

    let s3 = String::from("Hello");
    let s4 = &s3; //s3 is borrowed by s4, s3 is still valid
    println!("s4: {}", s4);
    println!("s3: {}", s3);

    let s5 = String::from("Hello");
    let s6 = s5.clone(); //s5 is cloned to s6, s5 is still valid
    println!("s5: {}", s5);
    println!("s6: {}", s6);

    //ownership practice from the video
    /* enum Light {
        Bright,
        Dull,
    }
    fn display_light(light: Light) {
        match light {
            Light::Bright => println!("The light is bright"),
            Light::Dull => println!("The light is dull"),
        }
    }
    fn main() {
        let dull = Light::Dull;
        display_light(dull); //dull is moved to display_light, dull is no longer valid
        display_light(dull); //error: use of moved value: `dull`
    }*/

//ownership 2 borrwing practice
    enum Light {
        Bright,
        Dull,
    }
    fn display_light(light: &Light) {
        match light {
            Light::Bright => println!("The light is bright"),
            Light::Dull => println!("The light is dull"),
        }
    }
    
        let dull = Light::Dull;
        display_light(&dull); //dull is borrowed by display_light, dull is still valid
        display_light(&dull); //dull is borrowed by display_light, dull is still valid
    
    //vector practice
   /* let my_numbers = vec![1, 2, 3];
    let mut my_numbers = vec::new();
    my_numbers.push(1);
    my_numbers.push(2);
    my_numbers.push(3);
    my_numbers.pop();
    my_numbers.len();
    let two = my_numbers[1];*/
   // let mut my_numbers = Vec![1, 2, 3];
    //for num in my_numbers.iter() {
      //  println!("{:?}", num);

    //}
// string and &str practice
fn print_it(data: &str) {
    println!("{:?}", data);
}
print_it("a string slice");
let ownned_string = "owned string".to_owned();//makes the string owned by the variable, it can be modified and passed to functions that take ownership
let another_owned = String::from("another");
print_it(&ownned_string);
print_it(&another_owned);

//will not work
struct Employee {
    //name: &str,
    name: String,
}
//let emp_name = "Jayson";
let emp_name = "Jayson".to_owned();
let emp_name = String::from("Jayson");
let emp = Employee {
    name: emp_name
};
//type annotation practice
let numbers: Vec<i32> = vec![1, 2, 3];
let letters: Vec<char> = vec!['a', 'b'];
//let clicks:Vec<Mouse> = vec![Mouse::Left, Mouse::Right];
//enums revisit
enum PromoDiscount {
    NewUser,
    Holiday(String),
}
enum Discount {
    Percent(f64),
    Flat(i32),
    Promo(PromoDiscount),
    Custom(String),
}
//Data options practice
struct Custmer {
    age:Option<u32>,
    email:String,
}
let mark = Custmer {
    age: Some(22),
    email: "mark@example.com".to_owned(),
};
let becky = Custmer {
    age: None,
    email: "becky@example.com".to_owned(),
};
match becky.age {
    Some(age) => println!("Becky's age is {}", age),
    None => println!("Becky's age is not provided"),
}
}
