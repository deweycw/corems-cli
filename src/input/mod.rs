

pub fn find_cards(content:&String) {
    
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
    write_py::header();
    if !global_settings_card {
        println!("GLOBAL_SETTINGS card must be defined to proceed!");
    } else {
        let mut global_params = read_global_settings_card(&content);
        write_py::global_params(global_params);
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
    } else{
        let mut return_tuple = read_search_card(&content);
        let search_params = return_tuple.0;
        let elements = return_tuple.1;

        write_py::search_params(search_params);
        write_py::elements(elements);
    }
}


pub fn read_search_card(content:&String) -> (HashMap<String,String>, HashMap<String,String>) {

    let mut multiple_assignments = false;
    let card_split: Vec<&str> = content.split("SEARCH").collect();
    let assign_card_grouped = card_split[1];

    let assign_cards: Vec<&str> = assign_card_grouped.split("ASSIGN").collect();
    
    let mut read_elements_card = false;
    let mut read_filters_card = false;

    let mut min_dbe = "0";
    let mut max_dbe = "20";
    let mut ion_charge = "1";
    let mut ion_type_selected = false;
    let mut is_radical = "False";
    let mut is_protonated = "True";
    let mut is_adduct = "False";
    let mut oc_filter = "1.0";
    let mut hc_filter = "2.0";
    let mut element_vec = Vec::new();
    let mut filters_vec = Vec::new();

    for card in assign_cards {
        for line in card.lines() {
            let line_vec: Vec<&str> = line.split_whitespace().collect();
            if line_vec.len() == 0 {
                continue;
            } else {            
                if line.contains("ELEMENTS") {
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
    }
    let mut search_params_hash = HashMap::from([
        ("min_dbe".to_string(),min_dbe.to_string()),
        ("max_dbe".to_string(),max_dbe.to_string()),
        ("ion_charge".to_string(),ion_charge.to_string()),
        ("isRadical".to_string(),is_radical.to_string()),
        ("isAdduct".to_string(),is_adduct.to_string()),
        ("isProtonated".to_string(),is_protonated.to_string()),
    ]);

    let mut elements_hash = HashMap::new();
    
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
        elements_hash.insert(py_element,element_range);
    }

    return (search_params_hash, elements_hash);
}
