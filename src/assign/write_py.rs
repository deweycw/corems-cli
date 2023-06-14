use std::fs;
use std::io::Write;
use std::fs::OpenOptions;
use std::collections::HashMap;
use std::env;
use std::path::Path;



use crate::assign::cards::common_card::*;
//const WORKING_DIR = get_current_working_dir();


const INPUT_FILE: &str = "corems_input.py";




pub fn _write_to_file<'a>(preamble:&str,params_hash:&Parameters<'a>) {
    let mut file = OpenOptions::new()
        .append(true)
        .open(INPUT_FILE)
        .unwrap();

    let size = params_hash.keys().len();
    let nparams = size as i32;

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



pub fn write_header() {
    let header = "import os\nfrom tempfile import tempdir\nimport time\nimport numpy as np\nimport warnings\nfrom datetime import date, datetime\nimport pandas as pd\n\nwarnings.filterwarnings('ignore')\nfrom pathlib import Path\nimport sys\nsys.path.append('./')\n\nos.chdir('/CoreMS')\nfrom corems.mass_spectra.input import rawFileReader\nfrom corems.molecular_id.factory.classification import HeteroatomsClassification, Labels\nfrom corems.molecular_id.search.priorityAssignment import OxygenPriorityAssignment\nfrom corems.molecular_id.search.molecularFormulaSearch import SearchMolecularFormulas\nfrom corems.encapsulation.factory.parameters import MSParameters\nfrom corems.encapsulation.constant import Atoms\nfrom corems.mass_spectrum.calc.Calibration import MzDomainCalibration\n#import corems.lc_icpms_ftms.calc.lc_icrms_qc_assign as icrms\n#import corems.lc_icpms_ftms.calc.lc_icrms_helpers as lcmsfns\nos.chdir('/CoreMS/usrdata')\n";
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(INPUT_FILE);
    fs::write(INPUT_FILE,header).expect("File not written");
 }

pub fn write_global_params<'a>(global_params_hash:Parameters<'a>) {
    let preamble = "\tMSParameters.molecular_search.";
    let mut file = OpenOptions::new()
        .append(true)
        .open(INPUT_FILE)
        .unwrap();
   
    writeln!(&file,"\n");
    writeln!(&file,"\t#global search settings\n\tMSParameters.molecular_search.url_database = 'postgresql+psycopg2://coremsappdb:coremsapppnnl@corems-cli-molformdb-1:5432/coremsapp'");
    _write_to_file(preamble,&global_params_hash);
}

pub fn write_first_search_params<'a>(search_params_hash:&Parameters<'a>) {
    let preamble = "\t\tMSParameters.molecular_search.";
    let mut file = OpenOptions::new()
        .append(true)
        .open(INPUT_FILE)
        .unwrap();
   
    writeln!(&file,"\n");
    writeln!(&file,"\t\t#first search settings");
    _write_to_file(preamble,&search_params_hash);
}

pub fn write_next_search_params<'a>(search_params_hash:&Parameters<'a>) {
    let preamble = "\t\tMSParameters.molecular_search.";
    let mut file = OpenOptions::new()
        .append(true)
        .open(INPUT_FILE)
        .unwrap();
   
    writeln!(&file,"\n");
    writeln!(&file,"\t\t#next search settings");
    _write_to_file(preamble,&search_params_hash);
}



pub fn write_first_elements(elements_hash:&Elements) {
    
    let preamble = "\t\tMSParameters.molecular_search.";
    let mut file = OpenOptions::new()
        .append(true)
        .open(INPUT_FILE)
        .unwrap();
    writeln!(&file,"\n\t\t#first search elements");

    let size = elements_hash.keys().len();
    let nelements = size as i32;

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


pub fn write_next_elements(elements_hash:&Elements) {
    
    let preamble = "\t\tMSParameters.molecular_search.";
    let mut file = OpenOptions::new()
        .append(true)
        .open(INPUT_FILE)
        .unwrap();
    writeln!(&file,"\n\t\t#next search elements");

    let size = elements_hash.keys().len();
    let nelements = size as i32;

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


pub fn write_assign_func_header() {
    let preamble = "\n\ndef assign_formula(esifile, times):";
    
    let mut file = OpenOptions::new()
        .append(true)
        .open(INPUT_FILE)
        .unwrap();
    
    let mut newline = preamble.to_owned();

    writeln!(&file,"{newline}");
}

pub fn write_run_search(first_hit: &str) {
    let preamble = "\n\t\tSearchMolecularFormulas(mass_spectrum,first_hit = ";
    
    let mut file = OpenOptions::new()
        .append(true)
        .open(INPUT_FILE)
        .unwrap();

    let mut newline = preamble.to_owned();
    newline.push_str(&first_hit);
    newline.push_str(").run_worker_mass_spectrum()");
    writeln!(&file,"{newline}");
}

pub fn write_assign_chunk() {
    let preamble = "\n\tMSParameters.mass_spectrum.threshold_method = 'signal_noise'\n\tMSParameters.mass_spectrum.s2n_threshold = 3\n\n\tparser = rawFileReader.ImportMassSpectraThermoMSFileReader(esifile)\n\n\ttic=parser.get_tic(ms_type='MS')[0]\n\ttic_df=pd.DataFrame({'time': tic.time,'scan': tic.scans})\n\tresults = []\n\n\tfor timestart in times:\n\n\t\tscans=tic_df[tic_df.time.between(timestart,timestart+interval)].scan.tolist()\n\t\tmass_spectrum = parser.get_average_mass_spectrum_by_scanlist(scans) ";
    
    let mut file = OpenOptions::new()
        .append(true)
        .open(INPUT_FILE)
        .unwrap();
    
    let mut newline = preamble.to_owned();

    writeln!(&file,"{newline}");
}

pub fn write_search_chunk() {

    let preamble = "\n\n\t\tmass_spectrum.percentile_assigned(report_error=True)\n\t\tassignments=mass_spectrum.to_dataframe()\n\t\tassignments['Time']=timestart\n\t\tresults.append(assignments)";

    let mut file = OpenOptions::new()
        .append(true)
        .open(INPUT_FILE)
        .unwrap();

    let mut newline = preamble.to_owned();

    writeln!(&file,"{newline}");
}

pub fn write_search_return() {

    let preamble = "\n\n\tresults=pd.concat(results,ignore_index=True)\n\n\treturn(results)";

    let mut file = OpenOptions::new()
        .append(true)
        .open(INPUT_FILE)
        .unwrap();

    let mut newline = preamble.to_owned();

    writeln!(&file,"{newline}");
}


pub fn write_calibration_chunk(cal_params_hash: HashMap<&str, String>) {
    let preamble = "\n\t\t# calibration settings\n\t\tmass_spectrum.settings.min_calib_ppm_error = -10\n\t\tmass_spectrum.settings.max_calib_ppm_error = 10\n\t\trefmasslist = ";
    
    let mut file = OpenOptions::new()
        .append(true)
        .open(INPUT_FILE)
        .unwrap();

    let mut newline = preamble.to_owned();
    
    let string_holder = cal_params_hash.get("ref_mass_list").unwrap();
    newline.push_str(string_holder);

    let calib_ppm_holder = cal_params_hash.get("calib_ppm_error_threshold").unwrap();
    newline.push_str("\n\t\tcal_ppm_threshold = ");
    newline.push_str(calib_ppm_holder);

    let calib_snr_holder = cal_params_hash.get("calib_snr_thrshold").unwrap();
    newline.push_str("\n\t\tcal_snr_thr = ");
    newline.push_str(calib_snr_holder);

    newline.push_str("\n\t\tcalfn = MzDomainCalibration(mass_spectrum,refmasslist) \n\t\tref_mass_list_fmt = calfn.load_ref_mass_list(refmasslist)\n\t\timzmeas, mzrefs = calfn.find_calibration_points(mass_spectrum, ref_mass_list_fmt, calib_ppm_error_threshold=cal_ppm_threshold, calib_snr_threshold=cal_snr_thr)\n\t\tcalfn.recalibrate_mass_spectrum(mass_spectrum, imzmeas, mzrefs, order=2)");
    writeln!(&file,"{newline}");

}


pub fn write_py_main(time_params_hash:HashMap<&str,&str>) {

    let func_body = "\n\n\nif __name__ == '__main__':\n\n\tdata_dir = '/CoreMS/usrdata/'\n\tresults = []\n\n\tinterval = ";

    let mut newline = func_body.to_owned();
    
    let interval = time_params_hash.get("interval").unwrap();
    newline.push_str(interval);

    let time_min = time_params_hash.get("time_min").unwrap();
    newline.push_str("\n\ttime_min = ");
    newline.push_str(time_min);

    let time_max = time_params_hash.get("time_max").unwrap();
    newline.push_str("\n\ttime_max = ");
    newline.push_str(time_max);

    newline.push_str("\n\ttimes = list(range(time_min,time_max,interval))\n\n\tflist = os.listdir(data_dir)\n\tf_raw = [f for f in flist if '.raw' in f]\n\tos.chdir(data_dir)\n\ti=1\n\n\tfor f in f_raw:\n\t\tprint(f)\n\t\toutput = assign_formula(esifile = f, times = times)\n\t\toutput['file'] = f\n\t\tresults.append(output)\n\t\ti = i + 1 \n\n\tfname = 'assignments.csv'\n\tdf = pd.concat(results)\n\tdf.to_csv(data_dir+fname)");

    let mut file = OpenOptions::new()
        .append(true)
        .open(INPUT_FILE)
        .unwrap();

    

    writeln!(&file,"{newline}");
}