use clap::{Arg, App, ArgMatches};
use rand::Rng;
use proceed::{proceed};
use std::io;
use std::cmp::{Ordering};

const DEFAULT_LENGTH: u32 = 12;

struct PassSpec {
    symbols: bool,
    numbers: bool,
    upper: bool,
    lower: bool,
    length: u32
}

fn generate_pass_spec(args: &ArgMatches) -> PassSpec {
    let symbols: bool = args.is_present("symbols");
    let numbers: bool = args.is_present("numbers");
    let upper: bool = args.is_present("upper");
    let lower: bool = args.is_present("lower");
    
    let length: u32 = args.value_of_t("length").unwrap_or(DEFAULT_LENGTH);

    if !symbols && !numbers && !upper && !lower {
        PassSpec { 
            symbols: true, 
            numbers: true, 
            upper: true, 
            lower: true, 
            length: length }
    }
    else {
        PassSpec { 
            symbols: symbols, 
            numbers: numbers, 
            upper: upper, 
            lower: lower, 
            length: length }
    }
}

fn generate_char_pool(spec: &PassSpec) -> Vec<char> {
    let mut pool: Vec<char> = vec![];

    if spec.symbols {
        pool.extend(('!'..'0').chain(':'..'A').chain('['..'a').chain('{'..'~')); 
    }

    if spec.numbers {
        pool.extend('0'..':');
    }

    if spec.upper {
        pool.extend('A'..'[');
    }

    if spec.lower {
        pool.extend('a'..'{');
    }

    pool
}

fn generate_pass(spec: &PassSpec, rng: &mut rand::rngs::ThreadRng) -> String {
    let pool = generate_char_pool(&spec);
    let mut pass = String::new();

    for _i in 0..spec.length {
        let rnd_index = rng.gen_range(0..pool.len());
        pass.push(pool[rnd_index]);
    }

    pass
}

fn generate_pass_loop(spec: &PassSpec, rng: &mut rand::rngs::ThreadRng) -> String {
    loop {
        let pass = generate_pass(&spec, rng);
        println!("Generated password: {}", pass);
        println!("Use password? [y/N]");

        if proceed() {
            return pass
        }
    }
}

fn practice_pass(pass: &String) {
    println!("Password: {}", pass);
    let mut line = String::new();

    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
}

fn practice_loop(times: u32, pass: &String) {
    println!("Let's practice your password {} times", times);

    for i in 0..times {
        println!();
        println!("Practice {}/{}", i + 1, times);
        practice_pass(&pass);
    }
}

fn pass_compare(correct: &String, attempt: &String) -> bool {
    if correct == attempt {
        println!("You got it!");
        true
    }
    else {
        match correct.len().cmp(&attempt.len()) {
            Ordering::Less => println!("Too long!"),
            Ordering::Greater => println!("Too short!"),
            _ => println!("Errors in your password!")
        }
        false
    }
}

fn test_pass(pass: &String) -> bool {
    print!("{}[2J", 27 as char);
    println!("Now try and remember your password!");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    pass_compare(&pass, &String::from(line.trim()))
}


fn learn_pass_loop(pass: &String) {
    loop {
        practice_loop(3, &pass);
        let _correct = test_pass(&pass);
    }
}

fn main() {
    let matches = App::new("pass-tutor")
        .version("0.0.1")
        .author("Jackson C <jacksonc@alerik.de>")
        .about("Helps you generate and learn passwords")
        .arg(Arg::new("symbols")
            .short('s')
            .long("symbols")
            .takes_value(false)
            .about("Allow symbols in generated passwords"))
        .arg(Arg::new("numbers")
            .short('n')
            .long("numbers")
            .takes_value(false)
            .about("Allow numbers in generated passwords"))
        .arg(Arg::new("upper")
            .short('U')
            .long("upper")
            .about("Allow upper-case letters in generated passwords"))
        .arg(Arg::new("lower")
            .short('L')
            .long("lower")
            .takes_value(false)
            .about("Allow lower-case in generated passwords"))
        .arg(Arg::new("length")
            .short('l')
            .long("length")
            .takes_value(true)
            .about("Length of the password. Default is 12"))
        .get_matches();

    let mut rng = rand::thread_rng();
    let spec = generate_pass_spec(&matches);
    let pass = generate_pass_loop(&spec, &mut rng);
    learn_pass_loop(&pass);
}
