use std::fs;
use std::io::Write;
use std::fs::OpenOptions;
use std::collections::HashMap;

#[macro_use]

use crate::common::*;

fn _write_to_file<'a>(preamble:&str,params_hash:Parameters<'a>) {
    let mut file = OpenOptions::new()
        .append(true)
        .open("./corems_input.py")
        .unwrap();

    let size = params_hash.keys().len();
    let nparams = size as i32;

    println!("{} params found", nparams);

    for n in 0..nparams {

        let nIndex = ParamIndex {
            i: n,
        };
        let param_value = params_hash.get(&nIndex).unwrap();
        //println!("{:?}", param_value.py_param);
        let mut newline = preamble.to_owned();

        newline.push_str(&param_value.py_param);
        newline.push_str(" = ");
        newline.push_str(&param_value.value);
        writeln!(&file,"{newline}");
    }

}



pub fn header() {
    let header = "import os\nfrom tempfile import tempdir\nimport time\nimport numpy as np\nimport warnings\nfrom datetime import date, datetime\nimport pandas as pd\n\nwarnings.filterwarnings('ignore')\nfrom pathlib import Path\nimport sys\nsys.path.append('./')\n\nos.chdir('/CoreMS')\nfrom corems.mass_spectra.input import rawFileReader\nfrom corems.molecular_id.factory.classification import HeteroatomsClassification, Labels\nfrom corems.molecular_id.search.priorityAssignment import OxygenPriorityAssignment\nfrom corems.molecular_id.search.molecularFormulaSearch import SearchMolecularFormulas\nfrom corems.encapsulation.factory.parameters import MSParameters\nfrom corems.encapsulation.constant import Atoms\nfrom corems.mass_spectrum.calc.Calibration import MzDomainCalibration\nimport corems.lc_icpms_ftms.calc.lc_icrms_qc_assign as icrms\nimport corems.lc_icpms_ftms.calc.lc_icrms_helpers as lcmsfns\nos.chdir('/CoreMS/usrdata')\n";
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("./corems_input.py");
    fs::write("./corems_input.py",header).expect("File not written");
 }

pub fn global_params<'a>(global_params_hash:Parameters<'a>) {
    let preamble = "MSParameters.molecular_search.";
    let mut file = OpenOptions::new()
        .append(true)
        .open("./corems_input.py")
        .unwrap();
   
    writeln!(&file,"\n");
    writeln!(&file,"#global search settings");
    _write_to_file(preamble,global_params_hash);
}

pub fn search_params<'a>(search_params_hash:Parameters<'a>) {
    let preamble = "MSParameters.molecular_search.";
    let mut file = OpenOptions::new()
        .append(true)
        .open("./corems_input.py")
        .unwrap();
   
    writeln!(&file,"\n");
    writeln!(&file,"#first search settings");
    _write_to_file(preamble,search_params_hash);
}

pub fn elements(elements_hash:Elements) {
    
    let preamble = "MSParameters.molecular_search.";
    let mut file = OpenOptions::new()
        .append(true)
        .open("./corems_input.py")
        .unwrap();
    writeln!(&file,"#first search elements");

    let size = elements_hash.keys().len();
    let nelements = size as i32;

    println!("{} elements found", nelements);

    for n in 0..nelements {

        let nIndex = ElementIndex {
            i: n,
        };
        let element_value = elements_hash.get(&nIndex).unwrap();
        let mut newline = preamble.to_owned();

        newline.push_str(&element_value.symbol);
        newline.push_str(" = ");
        newline.push_str(&element_value.range);
        writeln!(&file,"{newline}");
    }

}

pub fn assign_func(elements_hash:HashMap<String,String>) {
    let preamble = "def assign_formula(esifile, times, cal_ppm_threshold=(-1,1), refmasslist=None):\nMSParameters.mass_spectrum.threshold_method = 'signal_noise'\nMSParameters.mass_spectrum.s2n_threshold = 3\nparser = rawFileReader.ImportMassSpectraThermoMSFileReader(esifile)\n";
}
