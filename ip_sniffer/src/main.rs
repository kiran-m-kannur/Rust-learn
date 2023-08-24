use std::env;
// use std::io;
use std::fs;

fn parse_config(args:&[String])-> (&str,&str){
    let query_string = &args[1];
    let file_path = &args[2];

    (query_string,file_path)
}

fn main() {
    
    
    let args:Vec<String>=env::args().collect();

    let (query_string,file_path) = parse_config(&args);

    let data = fs::read_to_string(file_path).expect("Could not parse the data in the above path");

    let mut num_occurances = 0;

    for words in data.split(" ") {


        if words == query_string {
            num_occurances +=1;
        }
        
    }

    println!("The word {0} is present in the following text {1} times.",query_string,num_occurances);





}   
