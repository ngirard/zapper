#[macro_use]
extern crate zapper;

use zapper::compile;

use std::io::stdout;

#[derive(ZapperRunner)]
#[filter = "sqrt/0n"]
#[filter = "round/1n"]
#[filter = "toupper/0s"]
struct Person {
    id: u64,
    name: String,
    age: u32,
    weight: f64,
}

#[derive(ZapperEnv)]
#[runner = "Person"]
struct Provider {
    provider: String,
    provider_code: u32,
}

fn sqrt(_data: &Person, _args: &[f64], input: f64) -> f64 {
    input.sqrt()
}

fn round(_data: &Person, args: &[f64], input: f64) -> f64 {
    let digits = args[0];
    if digits > 10.0 {
        return input;
    }
    let factor = 10u32.pow(digits as u32) as f64;
    let value = (input * factor).round() as f64;
    value / factor
}

fn toupper(_data: &Person, _args: &[f64], input: &str, buffer: &mut String) {
    for c in input.as_bytes() {
        buffer.push(c.to_ascii_uppercase() as char)
    }
}

fn main() {
    let template = "{{provider}} {{provider_code + 4}} {{id}} {{name | toupper}} {{age | sqrt}} {{weight / 2.2 | round 2}}kg\n";

    let env = Provider {
        provider: "john doe".to_string(),
        provider_code: 31,
    };

    let mut bytecode = match compile(template, &env) {
        Ok(bc) => bc,
        Err(err) => {
            eprintln!("error compiling template: {}", err);
            return;
        }
    };

    // println!("bytecode: {:#?}", bytecode);

    // build up a group of 100 (similar) people
    let mut group = vec![];
    for i in 0..100 {
        group.push(Person {
            id: 12 + i,
            name: "Bob".to_string(),
            age: 49,
            weight: 170.3 + i as f64,
        });
    }

    let stdout = stdout();
    let mut stdout_lock = stdout.lock();

    for person in group {
        bytecode.render(&person, &mut stdout_lock).unwrap();
    }
}
