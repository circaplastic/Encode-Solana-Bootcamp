fn fizz_buzz() {
    let mut counter = 0;

    for i in 1..=301 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("{i}: fizz buzz");
            counter +=1
        } else if i % 3 == 0 {
            println!("{i}: fizz");    
        } else if i % 5 == 0 {
            println!("{i}: buzz");
        } else {
            println!("{i}");
        }
    }
    println!("fizz buzz occurred {counter} times");
}
