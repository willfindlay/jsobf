use clap::Parser;
use clap_derive::Parser;
use rand::{thread_rng, Rng as _};
use serde_json::{Map, Number, Value};
use std::{collections::HashMap, fs::File, io::stdin, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// File containing JSON to obfuscate. Can be set to `-` to use STDIN.
    #[arg(value_name = "JSON_FILE")]
    file: PathBuf,
    /// Should the output be pretty
    #[arg(long)]
    pretty: bool,
    /// Show key mapping in output
    #[arg(long = "show-keys")]
    show_keys: bool,
}

fn obfuscate(value: &Value, known_keys: &mut HashMap<String, String>) -> Value {
    match value {
        Value::Null => value.clone(),
        Value::Bool(_) => Value::Bool(obfuscate_bool()),
        Value::Number(number) => Value::Number(obfuscate_number(number)),
        Value::String(s) => Value::String(obfuscate_string(s.len())),
        Value::Array(vec) => Value::Array(obfuscate_vec(vec, known_keys)),
        Value::Object(map) => Value::Object(obfuscate_obj(map, known_keys)),
    }
}

fn obfuscate_bool() -> bool {
    rand::random()
}

fn obfuscate_number(number: &Number) -> Number {
    if number.is_u64() {
        let n: u128 = rand::random();
        Number::from_u128(n).expect("should convert")
    } else if number.is_i64() {
        let n: i128 = rand::random();
        Number::from_i128(n).expect("should convert")
    } else if number.is_f64() {
        let n: f64 = rand::random();
        Number::from_f64(n).expect("should convert")
    } else {
        unreachable!()
    }
}

fn obfuscate_string(n: usize) -> String {
    thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(n)
        .map(char::from) // From link above, this is needed in later versions
        .collect()
}

fn obfuscate_vec(v: &Vec<Value>, known_keys: &mut HashMap<String, String>) -> Vec<Value> {
    let mut out = Vec::with_capacity(v.capacity());
    for val in v {
        out.push(obfuscate(val, known_keys));
    }
    out
}

fn obfuscate_obj(
    m: &Map<String, Value>,
    known_keys: &mut HashMap<String, String>,
) -> Map<String, Value> {
    let mut out = Map::new();
    for (k, v) in m {
        let nk = if let Some(k) = known_keys.get(k) {
            k.clone()
        } else {
            obfuscate_string(k.len())
        };
        known_keys.insert(k.clone(), nk.clone());
        let nv = obfuscate(v, known_keys);
        out.insert(nk, nv);
    }
    out
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::try_parse()?;

    let parsed: Result<Value, _> = if cli.file.to_str() == Some("-") {
        serde_json::from_reader(stdin())
    } else {
        serde_json::from_reader(File::open(cli.file)?)
    };

    let mut known_keys = HashMap::new();
    let out = obfuscate(&parsed?, &mut known_keys);

    if cli.show_keys {
        for (k, v) in known_keys {
            println!("{:?}: {:?}", k, v);
        }
        println!("-----");
    }

    if cli.pretty {
        println!("{}", serde_json::to_string_pretty(&out)?)
    } else {
        println!("{}", serde_json::to_string(&out)?)
    }

    Ok(())
}
