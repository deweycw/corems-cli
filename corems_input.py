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


#global search settings
MSParameters.molecular_search.error_method = None
MSParameters.molecular_search.min_ppm_error = -0.25
MSParameters.molecular_search.max_ppm_error = 0.25
MSParameters.molecular_search.threshold_method = 'signal_noise'
MSParameters.molecular_search.s2n_threshold = 3
MSParameters.molecular_search.score_method = 'prob_score'
MSParameters.molecular_search.output_score_method = 'prob_score'


#first search settings
MSParameters.molecular_search.min_dbe = 0
MSParameters.molecular_search.max_dbe = 20
MSParameters.molecular_search.ion_charge = 1
MSParameters.molecular_search.isRadical = False
MSParameters.molecular_search.isAdduct = True
MSParameters.molecular_search.isProtonated = True
#first search elements
MSParameters.molecular_search.usedAtoms['C'] = (1,50)
MSParameters.molecular_search.usedAtoms['H'] = (4,100)
MSParameters.molecular_search.usedAtoms['O'] = (0,20)
MSParameters.molecular_search.usedAtoms['N'] = (0,4)
MSParameters.molecular_search.usedAtoms['S'] = (0,1)
MSParameters.molecular_search.usedAtoms['Si'] = (0,10)
MSParameters.molecular_search.usedAtoms['Cu'] = (0,1)
