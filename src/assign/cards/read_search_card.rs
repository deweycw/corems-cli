use std::io::{stdout, Read, Write};
use std::time::Duration;
use std::collections::HashMap;
use std::env::{set_current_dir, current_dir};


//pub mod common;
use crate::assign::cards::common_card::*;

pub fn read_search_card<'a>(content:&'a String) -> HashMap<i32, AssignParams<'a>> {

    let mut first_assign = true;
    let card_split: Vec<&str> = content.split("SEARCH").collect();
    let assign_card_grouped = card_split[1];

    let assign_cards: Vec<&str> = assign_card_grouped.split("ASSIGNMENT").collect();
    
    fn get_assign_card_vals<'a>(card:&'a str) -> AssignParams<'a> {
        let mut read_elements_card = false;
        let mut read_filters_card = true;
        let mut min_dbe = "0";
        let mut max_dbe = "20";
        let mut ion_charge = "1";
        let mut ion_type_selected = false;
        let mut is_radical = "False";
        let mut is_protonated = "True";
        let mut is_adduct = "False";
        let mut oc_filter = "0";
        let mut hc_filter = "0";
        let mut element_vec = Vec::new();
        let mut filters_vec = Vec::new();
        let mut element_return = false;
        let mut param_return = false;

        for line in card.lines() {
            let line_vec: Vec<&str> = line.split_whitespace().collect();
            if line_vec.len() == 0 {
                continue;
            } else if line.contains("\n") {
                continue;
            } else {    
                if line.contains("ELEMENTS") {
                    read_elements_card = true;
                    element_return = true;
                }
                if line.contains("FILTERS") {
                    read_filters_card = false;
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
                    param_return = false;
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
                if line.contains("(DE)PROTONATED") {
                    let vec: Vec<&str> = line.split_whitespace().collect();
                    let mut protonated_string = "True";
                    if vec.len() == 1 {
                        is_protonated = "True";
                    } else{
                        protonated_string = vec[1];
                    }
                    
                    if protonated_string.contains("True"){
                        is_protonated = "True";
                    } else if protonated_string.contains("False"){
                        is_protonated = "False"
                    } else {
                        println!("(DE)PROTONATED requires either True or False. Default value (True) will be used.");
                        is_protonated = "True";
                    }
                    ion_type_selected = true;
                }
                if line.contains("RADICAL") {
                    let vec: Vec<&str> = line.split_whitespace().collect();
                    let mut radical_string = "True";
                    if vec.len() == 1 {
                        is_radical = "True";
                    } else{
                        radical_string = vec[1];
                    }
                    
                    if radical_string.contains("True"){
                        is_radical = "True";
                    } else if radical_string.contains("False"){
                        is_radical = "False"
                    } else {
                        println!("RADICAL requires either True or False. Default value (True) will be used.");
                        is_radical = "True";
                    }
                    ion_type_selected = true;
                }
                if line.contains("ADDUCT") {
                    let vec: Vec<&str> = line.split_whitespace().collect();
                    let mut adduct_string = "True";
                    if vec.len() == 1 {
                        is_adduct = "True";
                    } else{
                        adduct_string = vec[1];
                    }
                    
                    if adduct_string.contains("True"){
                        is_adduct = "True";
                    } else if adduct_string.contains("False"){
                        is_adduct = "False"
                    } else {
                        println!("ADDUCT requires either True or False. Default value (True) will be used.");
                        is_adduct = "True";
                    }
                    is_adduct = "True";
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
                read_filters_card = false;
                if read_filters_card {
                    println!("yes");
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

        let mut elements_hash: Elements = HashMap::new();

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
        let card_vec: Vec<&str> = card.split_whitespace().collect();

        if card_vec.len() > 1{
            let card_vals = get_assign_card_vals(card);
            param_hash.insert(k, card_vals);
            k = k + 1;
        }
    };

    return param_hash;
}
