use std::io::{stdout, Read, Write};
use std::time::Duration;
use std::collections::HashMap;
use std::env::{set_current_dir, current_dir};


#[macro_use]
//pub mod cards;
use crate::assign::cards::common_card::*;

pub fn read_global_settings_card<'a>(content:&'a String) -> Parameters<'a> {
    
    ///set default values
    
    let mut error_method = "'s2n'";
    let mut min_ppm_error = "UNDEFINED";
    let mut max_ppm_error = "UNDEFINED";
    let mut threshold_method = "'signal_noise'";
    let mut s2n_threshold = "3";
    let mut min_prominence = "0.1";
    let mut score_method = "'prob_score'";
    let mut output_score_method = "'prob_score'";
    
    let card_split: Vec<&str> = content.split("GLOBAL_SETTINGS").collect();
    let global_settings_card = card_split[1];
    
    let mut read_error_card = false;
    let mut read_thresholding_card = false;
    
    let mut score_method_input = true;
    let mut output_score_method_input = true;

    for line in global_settings_card.lines() {
        if line.contains("ERROR_SETTINGS") {
            read_error_card = true;
        }
        
        if line.contains("THRESHOLDING") {
            read_thresholding_card = true;
        }
        
        if line.contains("SCORE_METHOD") && score_method_input {
            let vec: Vec<&str> = line.split_whitespace().collect();
            if vec.len() == 1 {
                println!("SCORE_METHOD not defined. Using default setting.");   
            } else {
                score_method = vec[1];
            }
            score_method_input = false;
        }

        if line.contains("OUTPUT_SCORE_METHOD") && output_score_method_input {
            let vec: Vec<&str> = line.split_whitespace().collect();
            if vec.len() == 1 {
                println!("OUTPUT_SCORE_METHOD not defined. Using default setting.");
            } else {
                output_score_method = vec[1];
            }
            output_score_method_input = false;
        }

        if read_error_card {
            if line.contains("ERROR_METHOD"){
                let vec: Vec<&str> = line.split_whitespace().collect();
                if vec.len() == 1 {
                    println!("ERROR_METHOD not defined. Using default setting.");
                } else {
                    error_method = vec[1];
                }
            }
            if line.contains("MIN_PPM_ERROR"){
                let vec: Vec<&str> = line.split_whitespace().collect();
                if vec.len() == 1 {
                    println!("MIN_PPM_ERROR not defined. This parameter must be defined to perform serach.")
                } else{
                    min_ppm_error = vec[1]; 
                }
            }
            if line.contains("MAX_PPM_ERROR") {
                let vec: Vec<&str> = line.split_whitespace().collect();
                if vec.len() == 1 {
                    println!("MAX_PPM_ERROR not defined. Using default setting.")
                } else {
                    max_ppm_error = vec[1];
                }
            }
            if line.contains("/") {
                read_error_card = false;
            }
        }

        if read_thresholding_card {
            if line.contains("THRESHOLD_METHOD") {
                let vec: Vec<&str> = line.split_whitespace().collect();
                if vec.len() == 1 {
                    println!("THRESHOLD_METHOD not defined. Using default setting.");
                } else {
                    threshold_method = vec[1];
                }
            }
            if line.contains("S2N_THRESHOLD") {
                let vec: Vec<&str> = line.split_whitespace().collect();
                if vec.len() == 1 {
                    println!("S2N_THRESHOLD not defined. Using default setting.");
                } else {
                    s2n_threshold = vec[1];
                }
            }
            if line.contains("MIN_PROMINENCE") {
                let vec: Vec<&str> = line.split_whitespace().collect();
                if vec.len() == 1 {
                    println!("MIN_PROMINENCE not defined. Using default setting.");
                } else {
                    min_prominence = vec[1]
                }
            }
            if line.contains("/") {
                read_thresholding_card = false;
            }
        }
    }

    let mut param_vec = vec![
        ("error_method",error_method),
        ("min_ppm_error",min_ppm_error),
        ("max_ppm_error",max_ppm_error),
        ("threshold_method",threshold_method),
        ("s2n_threshold",s2n_threshold),
        ("score_method",score_method),
        ("output_score_method",output_score_method)
    ];

    let global_params_hash = make_param_hash(&param_vec);

    return global_params_hash;

}