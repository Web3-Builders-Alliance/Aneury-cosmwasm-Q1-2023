

pub fn run(){
    let x = 1;   // i32
    let y = 2.5; // f64
    let z: i64 = 435343434434344; // i64
    
    let is_active: bool = true;

    let is_greater : bool = 1>2;

    let a1 = 'a';

    let face = '\u{1F600}';

    println!("Max i32 {}", std::i32::MAX );
    println!("Max i32 {}", std::i64::MAX );


    println!("{:?}",(x,y,z,is_active,is_greater,face));
}