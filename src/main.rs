use bollard::container::{
    AttachContainerOptions, AttachContainerResults, Config, CreateContainerOptions,
    DownloadFromContainerOptions, InspectContainerOptions, KillContainerOptions,
    ListContainersOptions, LogsOptions, PruneContainersOptions, RemoveContainerOptions,
    RenameContainerOptions, ResizeContainerTtyOptions, RestartContainerOptions, StatsOptions,
    TopOptions, UpdateContainerOptions, UploadToContainerOptions, WaitContainerOptions,
};
use bollard::Docker;

use bollard::exec::{CreateExecOptions, ResizeExecOptions, StartExecResults};
use bollard::image::{CreateImageOptions, PushImageOptions, TagImageOptions};
use bollard::models::*;

use futures_util::{StreamExt, TryStreamExt};
use std::io::{stdout, Read, Write};
use std::time::Duration;
use std::collections::HashMap;

#[cfg(not(windows))]
use termion::raw::IntoRawMode;
#[cfg(not(windows))]
use termion::{async_stdin, terminal_size};
use tokio::io::AsyncWriteExt;
use tokio::task::spawn;
use tokio::time::sleep;

use clap::Parser;

#[macro_use]
pub mod common;
use crate::common::*;

pub mod write_py;



const IMAGE: &str = "deweycw/debian";

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    
    find_cards(&content);

    let docker = Docker::connect_with_socket_defaults().unwrap();

    let mut port_bindings = ::std::collections::HashMap::new();
    port_bindings.insert(

        String::from("8080/tcp"),
        Some(vec![PortBinding {
            host_ip: Some(String::from("127.0.0.1")),
            host_port: Some(String::from("8080")),
        }]),
    );


    let host_config = HostConfig {
        mounts: Some(vec![Mount {
            target: Some(if cfg!(windows) {
                String::from("C:\\Windows\\Temp")
            } else {
                String::from("/CoreMS/usrdata")
            }),
            source: Some(if cfg!(windows) {
                String::from("C:\\Windows\\Temp")
            } else {
                String::from("/Users/christiandewey/corems-docker/corems/rawfiles")
            }),
            typ: Some(MountTypeEnum::BIND),
            consistency: Some(String::from("default")),
            ..Default::default()
        }]),
        port_bindings: Some(port_bindings),
        ..Default::default()
    };

    docker
        .create_image(
            Some(CreateImageOptions {
                from_image: IMAGE,
                ..Default::default()
            }),
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await?;
        

    let id = &docker
        .create_container(
            Some(CreateContainerOptions {
                name: "test2",
                platform: None,
            }),
            Config {
                image:Some(IMAGE),
                tty: Some(true),
                host_config: Some(host_config),
                ..Default::default()
            },
        )
        .await?
        .id;

    docker.start_container::<String>(&id, None).await?;

    // non interactive
    let exec = docker
        .create_exec(
            &id,
            CreateExecOptions {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                cmd: Some(vec!["python3", "/CoreMS/usrdata/corems_input.py"]),
                ..Default::default()
            },
        )
        .await?
        .id;
    if let StartExecResults::Attached { mut output, .. } = docker.start_exec(&exec, None).await? {
        while let Some(Ok(msg)) = output.next().await {
            print!("{}", msg);
        }
    } else {
        unreachable!();
    }

    docker
        .remove_container(
            &id,
            Some(RemoveContainerOptions {
                force: true,
                ..Default::default()
            }),
        )
        .await?;

    Ok(())
}



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
    if !rawfiles_card {
        println!("RAWFILES card must be defined to proceed!");
    } else {
        read_rawfiles_card(&content);
    }

    if !time_binning_card {
        println!("TIME_BINNING card not found; all scans will be averaged.");
    } else {
        read_time_binning_card(&content);
    }

    if !search_card {
        println!("SEARCH card must be defined to proceed!");
    } 

    if !calibration_card {
        println!("CALIBRATION card must be defined to proceed!");
    } else {
        read_calibration_card(&content);
    }

    
    let mut global_params: Parameters<'a> = read_global_settings_card(&content);
    let mut assign_params_hash = read_search_card(&content);
    println!("{:?}",assign_params_hash.keys());
    let first_search = assign_params_hash.get(&2).unwrap();
    let first_search_params = &first_search.params;
    let first_elements = &first_search.elements;
    println!("{:?}", &first_search_params);
    let cal_params = read_calibration_card(&content);
    let time_params = read_time_binning_card(&content);

    let num_assign = assign_params_hash.keys().len();

    write_py::header();
    write_py::assign_func_header();
    write_py::global_params(global_params);
    write_py::assign_chunk();
    write_py::first_search_params(first_search_params);
    write_py::calibration_chunk(cal_params);
    write_py::first_elements(first_elements);
    let mut first_hit:&str = "False";
    write_py::run_search(&first_hit);
    if num_assign > 1 {

        let it: i32 = num_assign as i32;
        for a in 2..it {
            first_hit = "True";
            let search = assign_params_hash.get(&a).unwrap();
            let search_params = &search.params;
            let elements = &search.elements;
            write_py::next_search_params(search_params);
            write_py::next_elements(elements); 
            write_py::run_search(&first_hit); 
        } 
    }
    write_py::search_chunk();
    write_py::search_return();
    write_py::py_main(time_params);
}


fn read_calibration_card(content:&String) -> HashMap<&str,String> {
    
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

    println!("{:?}", cal_params_hash);


    return cal_params_hash;

}

#[derive(Debug)]
struct AssignParams<'a>{
    pub params: common::Parameters<'a>,
    pub elements: common::Elements,
}

fn read_search_card<'a>(content:&'a String) -> HashMap<i32, AssignParams<'_>> {

    let mut first_assign = true;
    let card_split: Vec<&str> = content.split("SEARCH").collect();
    let assign_card_grouped = card_split[1];

    let assign_cards: Vec<&str> = assign_card_grouped.split("ASSIGNMENT").collect();
    println!("{:?}",assign_cards.len());

    
    fn get_assign_card_vals(card:&str) ->AssignParams {
        let mut read_elements_card = false;
        let mut read_filters_card = true;
        let mut min_dbe = "0";
        let mut max_dbe = "20";
        let mut ion_charge = "1";
        let mut ion_type_selected = false;
        let mut is_radical = "False";
        let mut is_protonated = "False";
        let mut is_adduct = "False";
        let mut oc_filter = "1.0";
        let mut hc_filter = "2.0";
        let mut element_vec = Vec::new();
        let mut filters_vec = Vec::new();

        for line in card.lines() {
            let line_vec: Vec<&str> = line.split_whitespace().collect();
            if line_vec.len() == 0 {
                continue;
            } else {    
        
                if line.contains("ELEMENTS") {
                    println!("tests");
                    read_elements_card = true;
                }
                if line.contains("FILTERS") {
                    read_filters_card = true;
                }
                if line.contains("DBE") {
                    let vec: Vec<&str> = line.split_whitespace().collect();
                    if vec.len() == 1 {
                        println!("DBE min and max not defined. Default values (min = 0, max = 20) will be used.");
                    } else if vec.len() == 2{
                        println!("Search will consider DBE between 0 and {}.",vec[1]);
                        max_dbe = vec[1];
                    } else {
                        min_dbe = vec[1];
                        max_dbe = vec[2];
                    }
                }
                if line.contains("ION_CHARGE") {
                    let vec: Vec<&str> = line.split_whitespace().collect();
                    let temp = vec[1];
                    let str_charge = temp.to_string();
                    let int_charge = str_charge.parse::<i32>().unwrap();
                    let abs_charge = int_charge.abs();
                    if abs_charge > 3 {
                        println!("Please re-define ION_CHARGE. Only values of +/-1 or +/-2 are valid.");
                    } else {
                        ion_charge = vec[1];
                    }
                }
                if line.contains("PROTONATED") {
                    is_protonated = "True";
                    ion_type_selected = true;
                }
                if line.contains("RADICAL") {
                    is_radical = "True";
                    ion_type_selected = true;
                }
                if line.contains("ADDUCT") {
                    is_adduct = "True";
                    ion_type_selected = true;
                }
                if read_elements_card {
                    let vec_el: Vec<&str> = card.split("ELEMENTS").collect();
                    let temp = vec_el[1];
                    let elements_grp = temp.to_string();
                    for l in elements_grp.lines() {
                        let vec_el_2: Vec<&str> = l.split_whitespace().collect();
                        if vec_el_2.len() < 2 && vec_el_2.len() > 1 {
                            println!("Max and min number of each element must be specified!");
                            continue;
                        } else if !l.contains("/") && vec_el_2.len() > 0 {
                            let element = vec_el_2[0].to_owned();
                            let element_min = vec_el_2[1].to_owned();
                            let element_max = vec_el_2[2].to_owned();
                            let mut element_holder = Vec::new();
                            element_holder.push(element);
                            element_holder.push(element_min);
                            element_holder.push(element_max);
                            element_vec.push(element_holder);
                        } else if l.contains("/") {
                            read_elements_card = false;
                            break;
                        } else {
                            continue;
                        }
                    }
                    
                }
                
                if read_filters_card {
                    let vec_f: Vec<&str> = card.split("FILTERS").collect();
                    let tempf = vec_f[1];
                    let filters_grp = tempf.to_string();
                    for f in tempf.lines() {
                        
                        let vec_f2: Vec<&str> = f.split_whitespace().collect();

                        if vec_f2.len() < 1 {
                            continue;
                        } else if f.contains("/") {
                            read_filters_card = false;
                            break;
                        } else if vec_f2.len() == 1{
                            let mut filter_holder = vec_f2[0].to_owned();
                            let temp = vec_f2[0].to_owned();
                            filters_vec.push(temp);
                            filters_vec.push(filter_holder);
                        } else {
                            let mut filter_holder = vec_f2[1].to_owned();
                            filters_vec.push(filter_holder);
                        }
                    }
                    
                }
            }
        }

        let mut param_vec = vec![
            ("min_dbe",min_dbe),
            ("max_dbe",max_dbe),
            ("ion_charge",ion_charge),
            ("isRadical",is_radical),
            ("isAdduct",is_adduct),
            ("isProtonated",is_protonated),
        ];

        let search_params_hash = make_param_hash(&param_vec);

        let mut elements_hash: common::Elements = HashMap::new();

        let mut n: i32 = 0;

        for e in element_vec {
            let mut min = e[1].to_string();
            let mut max = e[2].to_string();
            let mut element_range = "(".to_string();
            element_range.push_str(&min);
            element_range.push_str(",");
            element_range.push_str(&max);
            element_range.push_str(")");

            let mut py_element = "usedAtoms['".to_string();
            py_element.push_str(&e[0]);
            py_element.push_str("']");

            let element_index = ElementIndex {
                i: n,
            };            

            let element_value = ElementValue {
                symbol: py_element,
                range: element_range,
            };

            elements_hash.insert(element_index, element_value);
            n = n + 1;
            
        }

        let param_set = AssignParams {
            params: search_params_hash,
            elements: elements_hash,
        };       
        
        return param_set
    }
    
    let mut param_hash: HashMap<i32,AssignParams> = HashMap::new();

    let mut k: i32 = 1;
    for card in assign_cards {

        let card_vals = get_assign_card_vals(card);
        param_hash.insert(k, card_vals);
        k = k + 1;
    };

    return param_hash;
}

fn read_time_binning_card(content:&String) -> HashMap<&str,&str> {
    
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


    let mut param_vec = vec![
        ("time_min",time_min),
        ("time_max",time_max),
        ("interval",interval),
    ];
    
    let mut time_params_hash: HashMap<_, _> = param_vec.into_iter().collect();

    return time_params_hash;

}


fn read_rawfiles_card(content:&String) {
    
    let mut read_card = false;
    
    for line in content.lines() {
        if line.contains("RAWFILES") {
            read_card = true;
            let vec: Vec<&str> = line.split_whitespace().collect();
            if vec.len() == 1 {
                println!("Location of raw data files is not set. Please set with RAWFILES card.")
            } else {
                let rawfiles_dir = vec[1];
            }
        }
    }

    if !read_card {
        println!("Location of raw data files is not set. Please set with RAWFILES card.")
    }
}


fn read_global_settings_card<'a>(content:&'a String) -> Parameters<'a> {
    
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

fn make_param_hash<'a>(param_vector: &Vec<(&'a str, &'a str)>) -> common::Parameters<'a> {

    let mut params_hash: common::Parameters = HashMap::new();

    let mut i: i32 = 0;

    for tuple in param_vector.iter() {
        let pname: &str = tuple.0;
        let pvalue: &str = tuple.1;
        let line_order = i;

        let param_index = ParamIndex {
            i: line_order,
        };            

        let param_value = ParamValue {
            py_param: pname,
            value: pvalue,
        };

        params_hash.insert(param_index, param_value);
        i = i + 1;

    }

    return params_hash;
}