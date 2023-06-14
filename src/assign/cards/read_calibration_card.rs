use std::io::{stdout, Read, Write};
use std::time::Duration;
use std::collections::HashMap;
use std::env::{set_current_dir, current_dir};


use crate::assign::cards::common_card::*;

pub fn read_calibration_card(content:&String) -> HashMap<&str,String> {
    
    ///set default values
    
    let mut ref_mass_list = "".to_string();
    let mut cal_ppm_threshold = "(-3,3)".to_string();
    let mut cal_snr_threshold = "3".to_string();
    let mut holder = "";
    let mut min_error = "-3";
    let mut max_error = "3";
    let mut string_bldr: Vec<String> = Vec::new();
    let mut calfile = "";
    
    let card_split: Vec<&str> = content.split("CALIBRATION_SETTINGS").collect();
    let cal_settings_card = card_split[1];
    
    let mut read_cal_card = false;
    let mut snr_threshold_input = true;
    let mut min_error_input = true;
    let mut max_error_input = true;
    let mut file_input = true;

    for line in cal_settings_card.lines() {
        if line.contains("CALIBRATION_SETTINGS") {
            read_cal_card = true;
        }

        if line.contains("SNR_THRESHOLD") && snr_threshold_input{
            let vec: Vec<&str> = line.split_whitespace().collect();
            if vec.len() == 1 {
                println!("SNR_THRESHOLD not defined. Using default setting.");   
            } else {
                cal_snr_threshold = vec[1].to_string();
            }
            snr_threshold_input = false;
        }

        if line.contains("MIN_PPM_ERROR") && min_error_input {
            let vec: Vec<&str> = line.split_whitespace().collect();
            if vec.len() == 1 {
                println!("MIN_PPM_ERROR not defined. Using default setting.");
            } else {
                min_error = vec[1];
            }
            min_error_input = false;
        }

        if line.contains("MAX_PPM_ERROR") && max_error_input {
            let vec: Vec<&str> = line.split_whitespace().collect();
            if vec.len() == 1 {
                println!("MAX_PPM_ERROR not defined. Using default setting.");
            } else {
                max_error = vec[1];
            }
            max_error_input = false;
        }

        if !min_error_input && !max_error_input {
            let min = min_error.to_owned();
            let max = max_error.to_owned();
            string_bldr.push(min);
            string_bldr.push(max);            
        }

        if line.contains("CAL_FILE") && file_input {
            let vec: Vec<&str> = line.split_whitespace().collect();
            if vec.len() == 1 {
                println!("CAL_FILE not defined. Using default setting.");
            } else {
                calfile = vec[1];
            }
            file_input = false;
        }

    }

    

    let mut min = string_bldr[0].to_string();
    let mut max = string_bldr[1].to_string();
    let mut cal_range = "(".to_string();
    cal_range.push_str(&min);
    cal_range.push_str(",");
    cal_range.push_str(&max);
    cal_range.push_str(")");

    let mut calfile_bldr: String = "'".to_owned();

    calfile_bldr.push_str(calfile);
    calfile_bldr.push_str("'");

    let mut param_vec = vec![
        ("calib_ppm_error_threshold",cal_range),
        ("calib_snr_thrshold",cal_snr_threshold),
        ("ref_mass_list",calfile_bldr),
    ];
    

    let mut cal_params_hash: HashMap<_, _> = param_vec.into_iter().collect();

    return cal_params_hash;

}
