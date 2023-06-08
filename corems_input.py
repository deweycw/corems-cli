import os
from tempfile import tempdir
import time
import numpy as np
import warnings
from datetime import date, datetime
import pandas as pd

warnings.filterwarnings('ignore')
from pathlib import Path
import sys
sys.path.append('./')

os.chdir('/CoreMS')
from corems.mass_spectra.input import rawFileReader
from corems.molecular_id.factory.classification import HeteroatomsClassification, Labels
from corems.molecular_id.search.priorityAssignment import OxygenPriorityAssignment
from corems.molecular_id.search.molecularFormulaSearch import SearchMolecularFormulas
from corems.encapsulation.factory.parameters import MSParameters
from corems.encapsulation.constant import Atoms
from corems.mass_spectrum.calc.Calibration import MzDomainCalibration
import corems.lc_icpms_ftms.calc.lc_icrms_qc_assign as icrms
import corems.lc_icpms_ftms.calc.lc_icrms_helpers as lcmsfns
os.chdir('/CoreMS/usrdata')


def assign_formula(esifile, times, cal_ppm_threshold=(-1,1), refmasslist=None):


	#global search settings
	MSParameters.molecular_search.error_method = None
	MSParameters.molecular_search.min_ppm_error = -0.25
	MSParameters.molecular_search.max_ppm_error = 0.25
	MSParameters.molecular_search.threshold_method = 'signal_noise'
	MSParameters.molecular_search.s2n_threshold = 5
	MSParameters.molecular_search.score_method = 'prob_score'
	MSParameters.molecular_search.output_score_method = 'prob_score'

	parser = rawFileReader.ImportMassSpectraThermoMSFileReader(esifile)

	tic=parser.get_tic(ms_type='MS')[0]
	tic_df=pd.DataFrame({'time': tic.time,'scan': tic.scans})
	results = []

	for timestart in times:

		scans=tic_df[tic_df.time.between(timestart,timestart+interval)].scan.tolist()
		mass_spectrum = parser.get_average_mass_spectrum_by_scanlist(scans) 


		#first search settings
		MSParameters.molecular_search.min_dbe = 0
		MSParameters.molecular_search.max_dbe = 20
		MSParameters.molecular_search.ion_charge = 1
		MSParameters.molecular_search.isRadical = False
		MSParameters.molecular_search.isAdduct = False
		MSParameters.molecular_search.isProtonated = True

		# calibration settings
		mass_spectrum.settings.min_calib_ppm_error = -10
		mass_spectrum.settings.max_calib_ppm_error = 10
		calfn = MzDomainCalibration(mass_spectrum, 'mz_ref.db')

		#first search elements
		MSParameters.molecular_search.usedAtoms['C'] = (1,50)
		MSParameters.molecular_search.usedAtoms['H'] = (4,100)
		MSParameters.molecular_search.usedAtoms['O'] = (0,20)
		MSParameters.molecular_search.usedAtoms['N'] = (0,4)
		MSParameters.molecular_search.usedAtoms['S'] = (0,1)
		MSParameters.molecular_search.usedAtoms['Si'] = (0,10)
		MSParameters.molecular_search.usedAtoms['Cu'] = (0,1)

		SearchMolecularFormulas(mass_spectrum,first_hit = False).run_worker_mass_spectrum()


		#next search settings
		MSParameters.molecular_search.min_dbe = 0
		MSParameters.molecular_search.max_dbe = 40
		MSParameters.molecular_search.ion_charge = 2
		MSParameters.molecular_search.isRadical = False
		MSParameters.molecular_search.isAdduct = True
		MSParameters.molecular_search.isProtonated = True

		#next search elements
		MSParameters.molecular_search.usedAtoms['C'] = (1,50)
		MSParameters.molecular_search.usedAtoms['H'] = (4,100)
		MSParameters.molecular_search.usedAtoms['O'] = (0,20)
		MSParameters.molecular_search.usedAtoms['N'] = (0,4)
		MSParameters.molecular_search.usedAtoms['S'] = (0,1)
		MSParameters.molecular_search.usedAtoms['Si'] = (0,10)
		MSParameters.molecular_search.usedAtoms['Cu'] = (0,10)

		SearchMolecularFormulas(mass_spectrum,first_hit = True).run_worker_mass_spectrum()


		mass_spectrum.percentile_assigned(report_error=True)
		assignments=mass_spectrum.to_dataframe()
		assignments['Time']=timestart
		results.append(assignments)


	results=pd.concat(results,ignore_index=True)

	return(results)



if __name__ -- '__main__':

	data_dir = '/CoreMS/usrdata/'

	interval = 2
	time_min = 0
	time_max = 28
	times = list(range(time_min,time_max,interval))

	flist = os.listdir(data_dir)
	f_raw = [f for f in flist if '.raw' in f]
	os.chdir(data_dir)
	i=1

	for i in f_raw:
		output = assign_formula(esifile = f, times = times)
		output['file'] = f
		results.append(output)
		i = i + 1 

	df = pd.concat(results)
	df.to_csv(data_dir+fname)
