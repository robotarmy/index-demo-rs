use std::collections::BTreeMap;
use std::io::{self};
use std::time::{Instant};


use serde::{Serialize, Deserialize};
use std::fs::File;
use bincode::{serialize_into, deserialize_from};
use std::io::BufWriter;
use std::io::BufReader;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Index {
    tree: BTreeMap<Box<String>, Box<Vec::<Box<String>>>>,
}

fn times(n: usize) -> impl Iterator {
    std::iter::repeat(()).take(n)
}

fn main() {
    let mut index;
    let index_file = String::from("/tmp/index.tree");
    if Path::new(&index_file).is_file() {
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


    let start_save = Instant::now();
    let mut f = BufWriter::new(File::create("/tmp/index.tree").unwrap());
    serialize_into(&mut f, &index).unwrap();
    let duration_save = start_save.elapsed();
    println!("Time saving map is: {:?}", duration_save);



    for _ in times(5) {
        let start_look = Instant::now();
        let (key, vals) = if let Some((key, vals)) = index.tree.pop_last()  { (key, vals) } else { todo!() };
        let duration_look = start_look.elapsed();
        println!("Time searching map is: {:?}", duration_look);
        println!("KEY:: {key}");
        println!("STORED VALUE :: {:?}", vals);
    }
}



