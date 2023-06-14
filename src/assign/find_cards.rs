use std::io::{stdout, Read, Write};
use std::collections::HashMap;

#[macro_use]
use crate::write_py::*;

use crate::cards::*;
use crate::find_cards::read_search_card::*;
use crate::find_cards::read_global_settings_card::*;
use crate::find_cards::read_calibration_card::*;
use crate::find_cards::read_time_binning_card::*;
use crate::find_cards::common_card::*;

fn find_cards<'a>(content:&'a String) {
    
    let mut global_settings_card = false;
    let mut calibration_card = false;
    let mut rawfiles_card = false;
    let mut time_binning_card = false;
    let mut search_card = false;
    let mut output_card = false;

    for line in content.lines() {
        if line.contains("GLOBAL_SETTINGS") {
            global_settings_card = true;
        } else if line.contains("CALIBRATION") {
            calibration_card = true;
        } else if line.contains("RAWFILES") {
            rawfiles_card = true;
        } else if line.contains("SEARCH") {
            search_card = true;
        } else if line.contains("TIME_BINNING") {
            time_binning_card = true;
        } else if line.contains("OUTPUT") {
            output_card = true;
        }
    }
    
    if !global_settings_card {
        println!("GLOBAL_SETTINGS card must be defined to proceed!");
    }

    if !time_binning_card {
        println!("TIME_BINNING card not found; all scans will be averaged.");
    } else {
        read_time_binning_card::read_time_binning_card(&content);
    }

    if !search_card {
        println!("SEARCH card must be defined to proceed!");
    } 

    if !calibration_card {
        println!("CALIBRATION card must be defined to proceed!");
    } else {
        read_calibration_card::read_calibration_card(&content);
    }

    
    let mut global_params: common_card::Parameters<'a> = read_global_settings_card::read_global_settings_card(&content);
    let mut assign_params_hash = read_search_card::read_search_card(&content);
    let first_search = assign_params_hash.get(&1).unwrap();
    let first_search_params = &first_search.params;
    let first_elements = &first_search.elements;
    
    let cal_params = read_calibration_card::read_calibration_card(&content);
    let time_params = read_time_binning_card::read_time_binning_card(&content);

    let num_assign = assign_params_hash.keys().len();

    write_header();
    write_assign_func_header();
    write_global_params(global_params);
    write_assign_chunk();
    write_first_search_params(first_search_params);
    write_first_elements(first_elements);
    write_calibration_chunk(cal_params);
    let mut first_hit:&str = "False";
    write_run_search(&first_hit);
    write_search_chunk();
    if num_assign > 1 {
        let it: i32 = num_assign as i32;
        for a in 1..it {
            first_hit = "True";
            let i = a + 1;
            let search = assign_params_hash.get(&i).unwrap();
            let search_params = &search.params;
            let elements = &search.elements;
            write_next_search_params(search_params);
            write_next_elements(elements); 
            write_run_search(&first_hit); 
            write_search_chunk();
        } 
    }
    write_search_return();
    write_py_main(time_params);
}