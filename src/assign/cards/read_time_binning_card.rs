use std::collections::HashMap;

pub fn read_time_binning_card(content:&String) -> HashMap<&str,&str> {
    
    let mut interval = "2";
    let mut time_min = "0";
    let mut time_max = "2";
    let mut time_range_check = false; 

    let card_split: Vec<&str> = content.split("TIME_BINNING").collect();
    let time_binning_card = card_split[1];
    
    for line in time_binning_card.lines() {
        if line.contains("INTERVAL") {
            let vec: Vec<&str> = line.split_whitespace().collect();
            if vec.len() == 1 {
                println!("INTERVAL is not set. Using default value of 2.0 min.")
            } else{
                interval = vec[1];   
            }
        }
        if line.contains("TIME_RANGE") {
            time_range_check = true;
            let vec: Vec<&str> = line.split_whitespace().collect();
            if vec.len() == 1 {
                println!("TIME_RANGE is not defined. Please define TIME_RANGE before proceeding.")
            } else {
                time_min = vec[1];
                time_max = vec[2];
            }
        }
    }


    let param_vec = vec![
        ("time_min",time_min),
        ("time_max",time_max),
        ("interval",interval),
    ];
    
    let time_params_hash: HashMap<_, _> = param_vec.into_iter().collect();

    return time_params_hash;

}