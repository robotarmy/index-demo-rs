use std::collections::BTreeMap;
use std::io::{self};
use std::time::{Instant};


use serde::{Serialize, Deserialize};
use std::fs::File;
use bincode::{serialize_into, deserialize_from};
use std::io::BufWriter;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

use std::process;
use rand::{thread_rng, Rng};
use clap::Parser;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    x: usize,
    
    #[arg(short, long, default_value_t = true)]
    write_index: bool,

    #[arg(short, long, default_value_t = false)]
    use_cache: bool,

    #[arg(short, long, default_value_t = 0)]
    generate: u32
}

#[derive(Serialize, Deserialize)]
struct Index {
    tree: BTreeMap<Box<String>, Box<Vec::<Box<String>>>>,
}



fn times(n: usize) -> impl Iterator {
    std::iter::repeat(()).take(n)
}

fn main() {
    let mut index;

    let args = Args::parse();

    if args.generate > 0 {
        let mut rng = thread_rng();

        let filename = format!("input_gen-{0}", args.generate);
        let mut f = BufWriter::new(File::create(filename).unwrap());
        for _ in 0..args.generate {
            let key : i64 = rng.gen_range(1000000..9999999999);
            let value : i32 = rng.gen_range(1..999999);
            match write!(f, "{1} {0}\n", key, value) {
                Ok(_) => 1,
                Err(thing) => {
                    println!("Unable to write to file -- {:?}", thing);
                    process::exit(10);
                },
            };
        }
        match f.flush() {
            Ok(_) =>{
                println!("Generated #{0} inputs", args.generate);
                process::exit(0);
            },
            Err(thing) => {
                println!("Unable to flush generate buffer, {:?}", thing);
                process::exit(1);
            },
        };
    }


    let index_file = String::from("/tmp/index.tree");
    if args.use_cache && Path::new(&index_file).is_file() {
        let start_load = Instant::now();
        let mut f = BufReader::new(File::open(index_file).unwrap());
        let index2: Index = deserialize_from(&mut f).unwrap();
        let duration_load = start_load.elapsed();
        println!("Time loading index is: {:?}", duration_load);
        index = index2;
    } else {
        index = Index{
            tree: BTreeMap::new()
        };
        let start = Instant::now();
        for line in io::stdin().lines()  {
            if let Ok(input) = line {
                let mut v = input.split(' ').collect::<Vec::<&str>>();
                let key = Box::new(v.pop().unwrap().to_string());
                let value = Box::new(v.pop().unwrap().to_string());

                if index.tree.contains_key(&key) {
                    let resp : Option<&mut Box<Vec::<Box<String>>>> = index.tree.get_mut(&key);
                    match resp {
                        Some(values) => {
                            values.push(value);
                        }
                        None => {
                            println!("ERROR");
                        }
                    }
                } else {
                    let mut values = Box::new(Vec::<Box<String>>::new());
                    values.push(value);
                    index.tree.insert(key, values);
                }
            }
        }
        let duration = start.elapsed();
        println!("Time elapsed building map is: {:?}", duration);
    }

    if args.write_index {
        let start_save = Instant::now();
        let mut f = BufWriter::new(File::create("/tmp/index.tree").unwrap());
        serialize_into(&mut f, &index).unwrap();
        let duration_save = start_save.elapsed();
        println!("Time saving map is: {:?}", duration_save);
    }




    for _ in times(args.x) {
        let start_look = Instant::now();
        let (key, vals) = if let Some((key, vals)) = index.tree.pop_last()  { (key, vals) } else { todo!() };
        let duration_look = start_look.elapsed();
        println!("Time searching map is: {:?}", duration_look);
        println!("KEY:: {key}");
        println!("STORED VALUE :: {:?}", vals);
    }
}



