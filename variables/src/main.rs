fn main() {
    // Constants are always immutable and are defined with const instead of let
    // Constants can only be assigned values from constant expressions (e.g., 10 * 6)
    // Constants are named using all caps with underscores between words
    // Constants must be explicitly typed using : <type>
    const MY_NUM: u8 = 5;
    // Variables are immutable by default, but can be set to mutable by adding mut
    // Variables can only be used within functions while constants can exist at the global scope
    let mut x = 5;
    println!("The value of x is {x} and my const is {MY_NUM}");
    x = 6;
    println!("Th value of x is {x}");

    let y = 5;

    // Overshadows the initial declaration of y
    // Could use mut however this approach allows you to redefine y while maintaining immutability
    let y = y + 1;

    {
        // Overshadows the previous definition of y in the outer scope but ends as soon as this scope ends
        // Shadowing also allows use to change the type, while even with mut you cannot change the type
        let y = "Hello";
        println!("Inner scope {y}");
    }

    println!("Outer scope {y}");
}
