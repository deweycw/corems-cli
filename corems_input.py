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
#import corems.lc_icpms_ftms.calc.lc_icrms_qc_assign as icrms
#import corems.lc_icpms_ftms.calc.lc_icrms_helpers as lcmsfns
os.chdir('/CoreMS/usrdata')


def assign_formula(esifile, times, cal_ppm_threshold=(-1,1), refmasslist=None):


	#global search settings
