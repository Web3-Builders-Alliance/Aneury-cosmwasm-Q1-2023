

pub fn run(){
    /// printing without argument
    println!("Hello from print.rs ");

    /// Basic formarter
    println!("print a single argument Value = {}", 1);

  
    /// Positional Argument
    println!("Position {1} win over {0} and {2}", "Jeff","Joao", "Aneury");


    /// Named Argument
    println!("print a single argument Value = {id} {number}", number=1, id=123);

    /// placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal {:o} ", 16,16,16);

    /// Placeholder for Debug trait
    println!("{:?}",(13,true,"Hello"));
}