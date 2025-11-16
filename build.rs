use std::fs;

fn main() {
    embuild::espidf::sysenv::output();

    let content = fs::read_to_string(".env").expect("Couldn't read .env");
    let vars:Vec<(String, String)> = content.lines()
                    .filter(|line| {
                            let trimmed = line.trim();
                            !trimmed.is_empty() && !trimmed.starts_with('#')
                        })
                    .filter_map(|line| line.split_once('='))
                    .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
                    .collect();


    //compiler uses stdout to catch variables apparently 
    //using env::set_var didn't work             
    //Thanks AI 
    for (k, v) in vars {
        println!("cargo:rustc-env={k}={v}");
    }
    
}
