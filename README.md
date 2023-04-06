# build

## with rust installed

cargo build

# run
## generate a file with 100000 entries
### 17MB of data
./target/debug/index-rs -g 1000000
### 170MB of data
./target/debug/index-rs -g 10000000

---
Note: using 1 hundred million will create 1.7GB of data
and take a few minutes to generate.

## execute query against dataset via stdin
### get values from top 3 index entries from stdin
cat input_gen-100000 | ./target/debug/index-rs -x 3

### generate a BTree and cache it to disk for fast loading
cat input_gen-100000 | ./target/debug/index-rs -w

### read a cached BTree and get top 5 inputs
cat input_gen-100000 | ./target/debug/index-rs -u -x 5


Usage: index-rs [OPTIONS]

Options:
  -x, --x <X>                [default: 1]
  -w, --write-index          
  -u, --use-cache            
  -g, --generate <GENERATE>  [default: 0]
  -h, --help                 Print help
  -V, --version              Print version



