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
    let preamble = "\tMSParameters.molecular_search.";
    let mut file = OpenOptions::new()
        .append(true)
        .open("./corems_input.py")
        .unwrap();
   
    writeln!(&file,"\n");
    writeln!(&file,"\t#global search settings");
    _write_to_file(preamble,global_params_hash);
}

pub fn search_params<'a>(search_params_hash:Parameters<'a>) {
    let preamble = "\t\tMSParameters.molecular_search.";
    let mut file = OpenOptions::new()
        .append(true)
        .open("./corems_input.py")
        .unwrap();
   
    writeln!(&file,"\n");
    writeln!(&file,"\t\t#first search settings");
    _write_to_file(preamble,search_params_hash);
}

pub fn elements(elements_hash:Elements) {
    
    let preamble = "\t\tMSParameters.molecular_search.";
    let mut file = OpenOptions::new()
        .append(true)
        .open("./corems_input.py")
        .unwrap();
    writeln!(&file,"\t\t#first search elements");

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

pub fn assign_func_header() {
    let preamble = "\n\ndef assign_formula(esifile, times, cal_ppm_threshold=(-1,1), refmasslist=None):";
    
    let mut file = OpenOptions::new()
        .append(true)
        .open("./corems_input.py")
        .unwrap();
    
    let mut newline = preamble.to_owned();

    writeln!(&file,"{newline}");
}


pub fn assign_chunk() {
    let preamble = "\n\n\tparser = rawFileReader.ImportMassSpectraThermoMSFileReader(esifile)\n\n\ttic=parser.get_tic(ms_type='MS')[0]\n\ttic_df=pd.DataFrame({'time': tic.time,'scan': tic.scans})\n\tresults = []\n\n\tfor timestart in times:\n\n\t\tscans=tic_df[tic_df.time.between(timestart,timestart+interval)].scan.tolist()\n\t\tmass_spectrum = parser.get_average_mass_spectrum_by_scanlist(scans) ";
    
    let mut file = OpenOptions::new()
        .append(true)
        .open("./corems_input.py")
        .unwrap();
    
    let mut newline = preamble.to_owned();

    writeln!(&file,"{newline}");
}

pub fn search_chunk() {

    let preamble = "\n\t\tSearchMolecularFormulas(mass_spectrum,first_hit=False).run_worker_mass_spectrum()\n\n\t\tmass_spectrum.percentile_assigned(report_error=True)\n\t\tassignments=mass_spectrum.to_dataframe()\n\t\tassignments['Time']=timestart\n\t\tresults.append(assignments)";

    let mut file = OpenOptions::new()
        .append(true)
        .open("./corems_input.py")
        .unwrap();

    let mut newline = preamble.to_owned();

    writeln!(&file,"{newline}");
}

pub fn search_return() {

    let preamble = "\n\n\tresults=pd.concat(results,ignore_index=True)\n\n\treturn(results)";

    let mut file = OpenOptions::new()
        .append(true)
        .open("./corems_input.py")
        .unwrap();

    let mut newline = preamble.to_owned();

    writeln!(&file,"{newline}");
}


pub fn calibration_chunk() {


}




pub fn py_main() {

    let func_body = "\n\n\nif __name__ -- '__main__':\n\n\tdata_dir = '/CoreMS/usrdata/'\n\tmzref = data_dir + 'mz_ref.db'\n\n\tinterval = 2\n\ttime_range = [7,11]\n\n\tresults = []\n\ttimes = list(range(time_range[0],time_range[1],interval))\n\n\tflist = os.listdir(data_dir)\n\tf_raw = [f for f in flist if '.raw' in f]\n\tos.chdir(data_dir)\n\ti=1\n\n\tfor i in f_raw:\n\t\toutput = assign_formula(esifile = f, times = times, cal_ppm_threshold=(-1,1), refmasslist = mzref)\n\t\toutput['file'] = f\n\t\tresults.append(output)\n\t\ti = i + 1 \n\n\tdf = pd.concat(results)\n\tdf.to_csv(data_dir+fname)";

    let mut file = OpenOptions::new()
        .append(true)
        .open("./corems_input.py")
        .unwrap();

    let mut newline = func_body.to_owned();

    writeln!(&file,"{newline}");
}