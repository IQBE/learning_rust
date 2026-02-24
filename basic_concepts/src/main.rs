// This is a comment
fn main() {
    const MY_CONST: u32 = 55;

    println!("My const (!= immutable!) is {}", MY_CONST);

    let x = 11;
    println!("Value of immutable x = {}", x);
    let x = "Other value";
    println!("Shadowed! Now it's '{}'", x);

    let mut a = 0;
    for value in 1..4 {
        println!("a: {}!", value);
        a += value;
    }

    let mut counter = 0;
    let b = loop {
        // this can also be done with a while loop
        counter += 1;

        if counter == 10 {
            println!("This is the end: {}!", counter);
            break counter;
        } else if counter == 9 {
            println!("Last count coming up: {}!", counter);
        } else {
            println!("counter: {}!", counter)
        }
    };

    println!("My function returned: {}", my_function(a, b));
}

/*
    Multi line comment!
*/
fn my_function(x: u32, y: u32) -> u32 {
    println!("X = {}", x);
    println!("Y = {}", y);
    x + y
}
