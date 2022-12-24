use std::env::args;
use std::collections::HashMap;
mod file;
mod execute;
mod check_null;
mod keywords;

fn main() { 
    let mut integers : HashMap<String, i32> = HashMap::new();
    
    let args : Vec<String> = args().collect();
    
    let mut arg_one : bool = false;
    
    if args.len() == 2 {
        arg_one = true;
    }

    if !arg_one {
        println!("No cift file provided");
        return;
    }

    let file_path : String = args[1].clone();

    if !file_path.ends_with(".cift") {
        println!("Given file is not a valid cift file");
        return;
    }

    integers.extend(execute::set_all_ints("main.cift".to_string()));

    keywords::get_and_execute(file::get_line(2, "main.cift".to_string()), integers);
}
