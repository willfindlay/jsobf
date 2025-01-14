use clap::Parser;
use clap_derive::Parser;
use rand::{thread_rng, Rng as _};
use serde_json::{Map, Number, Value};
use std::{collections::HashMap, fs::File, io::stdin, path::PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Files containing JSON to obfuscate. Can be set to a single `-` to use STDIN.
    #[arg(value_name = "JSON_FILE", required = true)]
    files: Vec<PathBuf>,
    /// Should the output be pretty
    #[arg(long)]
    pretty: bool,
    /// Show key mapping in output
    #[arg(long = "show-keys")]
    show_keys: bool,
    /// Show value mapping in output
    #[arg(long = "show-values")]
    show_values: bool,
}

fn obfuscate(
    value: &Value,
    known_keys: &mut HashMap<String, String>,
    known_values: &mut HashMap<Value, Value>,
) -> Value {
    if let Some(v) = known_values.get(value) {
        return v.clone();
    }
    match value {
        Value::Null => value.clone(),
        Value::Bool(b) => Value::Bool(*b),
        Value::Number(number) => {
            let n = Value::Number(obfuscate_number(number));
            known_values.insert(value.clone(), n.clone());
            n
        }
        Value::String(s) => {
            let s = Value::String(obfuscate_string(s.len()));
            known_values.insert(value.clone(), s.clone());
            s
        }
        Value::Array(vec) => Value::Array(obfuscate_vec(vec, known_keys, known_values)),
        Value::Object(map) => Value::Object(obfuscate_obj(map, known_keys, known_values)),
    }
}

fn obfuscate_number(number: &Number) -> Number {
    if number.is_u64() {
        let n: u64 = rand::random();
        Number::from_u128(n as u128).expect("should convert")
    } else if number.is_i64() {
        let n: i64 = rand::random();
        Number::from_i128(n as i128).expect("should convert")
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
        .map(char::from)
        .collect()
}

fn obfuscate_vec(
    v: &Vec<Value>,
    known_keys: &mut HashMap<String, String>,
    known_values: &mut HashMap<Value, Value>,
) -> Vec<Value> {
    let mut out = Vec::with_capacity(v.capacity());
    for val in v {
        out.push(obfuscate(val, known_keys, known_values));
    }
    out
}

fn obfuscate_obj(
    m: &Map<String, Value>,
    known_keys: &mut HashMap<String, String>,
    known_values: &mut HashMap<Value, Value>,
) -> Map<String, Value> {
    let mut out = Map::new();
    for (k, v) in m {
        let nk = if let Some(k) = known_keys.get(k) {
            k.clone()
        } else {
            obfuscate_string(k.len())
        };
        known_keys.insert(k.clone(), nk.clone());
        let nv = obfuscate(v, known_keys, known_values);
        out.insert(nk, nv);
    }
    out
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let parsed: Result<Vec<Value>, _> =
        if cli.files.len() == 1 && cli.files[0].to_str() == Some("-") {
            let de = serde_json::Deserializer::from_reader(stdin());
            de.into_iter().collect()
        } else {
            let mut res = Vec::with_capacity(cli.files.len());
            for f in cli.files {
                res.push(serde_json::from_reader(File::open(f)?))
            }
            res.into_iter().collect()
        };
    let parsed = parsed?;

    let mut known_keys = HashMap::new();
    let mut known_values = HashMap::new();
    let out = obfuscate_vec(&parsed, &mut known_keys, &mut known_values);

    if cli.show_keys {
        for (k, v) in known_keys {
            println!("{:?}: {:?}", k, v);
        }
        println!("-----");
    }

    if cli.show_values {
        for (k, v) in known_values {
            println!("{:?}: {:?}", k, v);
        }
        println!("-----");
    }

    if cli.pretty {
        for v in out {
            println!("{}", serde_json::to_string_pretty(&v)?)
        }
    } else {
        for v in out {
            println!("{}", serde_json::to_string(&v)?)
        }
    }

    Ok(())
}
