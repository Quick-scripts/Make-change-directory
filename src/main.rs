const HELPSTR: &str = 
    "Usage: <insert usage here>";


fn main() {
    let argv: Vec<String> = std::env::args().collect();

    if argv.len() == 1 {
        println!("{}", &HELPSTR);
        return;
    }

    /*
    let arg_argnumber_iterator = {
        argv[1..]
        .iter()
        .zip(1..)
    };
    */
    
    let mut first_successful_name: String = String::new();
    let mut first_successful_assigned: bool = false;
    let mut unsuccessful_dir: Vec<String> = Vec::new();


    for argument in argv.iter() {
        let res = std::process::Command::new("sh")
            .args(["mkdir", argument])
            .output();
            
        match res {
            Err(_) => { unsuccessful_dir.push(argument.clone()); },
            Ok(_) => {
                if first_successful_assigned == false {
                    first_successful_name = argument.clone();
                    first_successful_assigned = true;
                }
            }
        }
    }
    if first_successful_assigned == false {
        println!("Failed to create all directories");
        return;
    }

    if unsuccessful_dir.len() != 0 {
        println!("Unable to make directories: {}", unsuccessful_dir.join(", "));
        
    }

    std::process::Command::new("sh")
        .args(["cd", &first_successful_name[..]]);

    println!("Hello, world");
}

