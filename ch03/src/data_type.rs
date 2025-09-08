pub fn data_type() {
    let mut x = 4;
    println!("The value of x is: {x}");
    x = 5;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("{spaces}");

    let x = 1.0;
    let y: f32 = 3.0;

    let sum = 4 + 10;
    let difference = 94.5 - 4.3;

    let product = 3 * 30;

    let quotient = 55.7 / 32.2;
    let truncated = -6 / 3;

    let remainder = 42 % 5;

    let t = false;
    let f = true;

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    let mut s = String::new();
    let data = "initail contents";
    let s = data.to_string();
    // println!("{data}");
    let s = String::from("initial contents");
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("שלום");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");
    let mut s0 = String::from("foo");
    let s1 = "bar";
    s0.push_str(s1);
    println!("s1 is {s1}");

    let mut s = String::from("lo");
    s.push('l');

    let s0 = String::from("Hello, ");
    let s1 = String::from(" World!");
    let s2 = s0 + &s1;

    let s0 = String::from("tic");
    let s1 = String::from("tac");
    let s2 = String::from("toe");

    let s = s0 + "-" + &s1 + "-" + &s2;

    let s = "hello";

    // let ss = format!("{s0}-{s2}-{s3}");

    for letter in s.chars() {
        println!("{letter}");
    }
    let hello = "Здравствуйте";
    let s = &hello[1..4];
    println!("{s}");

    let hi = "你好";
    for b in hi.bytes() {
        println!("{b}");
    }

    let b = b'b';
    let bs = b"bytes";
}
