use crate::ffi;
use crate::interface::*;
use std::ptr::null_mut;
use std::result::Result;

/* #region DFTD3GCP */

/// DFT-D3 geometric counterpoise correction.
pub struct DFTD3GCP {
    ptr: ffi::dftd3_gcp,
    structure: DFTD3Structure,
}

impl Drop for DFTD3GCP {
    fn drop(&mut self) {
        unsafe { ffi::dftd3_delete_gcp(&mut self.ptr) };
    }
}

impl DFTD3GCP {
    /// Create new GCP object from structure.
    pub fn new(
        numbers: &[usize],
        positions: &[f64],
        lattice: Option<&[f64]>,
        periodic: Option<&[bool]>,
        method: &str,
        basis: &str,
    ) -> Self {
        Self::new_f(numbers, positions, lattice, periodic, method, basis).unwrap()
    }

    /// Evaluate the counterpoise correction.
    pub fn get_counterpoise(&self, eval_grad: bool) -> DFTD3Output {
        self.get_counterpoise_f(eval_grad).unwrap()
    }

    /// Get number of atoms for this current structure.
    pub fn get_natoms(&self) -> usize {
        self.structure.get_natoms()
    }

    /// Set realspace cutoff for evaluation of interactions (in Bohr).
    pub fn update(&mut self, positions: &[f64], lattice: Option<&[f64]>) {
        self.structure.update(positions, lattice)
    }

    /// Load geometric counter-poise parameters from internal storage
    pub fn load_gcp_param(structure: DFTD3Structure, method: &str, basis: &str) -> Self {
        Self::load_gcp_param_f(structure, method, basis).unwrap()
    }

    /// Set realspace cutoffs (quantities in Bohr)
    pub fn set_realspace_cutoff(&self, bas: f64, srb: f64) {
        self.set_realspace_cutoff_f(bas, srb).unwrap()
    }

    /// Create new GCP object from structure (failable).
    pub fn new_f(
        numbers: &[usize],
        positions: &[f64],
        lattice: Option<&[f64]>,
        periodic: Option<&[bool]>,
        method: &str,
        basis: &str,
    ) -> Result<Self, DFTD3Error> {
        let structure = DFTD3Structure::new_f(numbers, positions, lattice, periodic)?;
        Self::load_gcp_param_f(structure, method, basis)
    }

    /// Evaluate the counterpoise correction (failable)
    pub fn get_counterpoise_f(&self, eval_grad: bool) -> Result<DFTD3Output, DFTD3Error> {
        let structure = &self.structure;
        let natoms = structure.get_natoms();
        let mut energy = 0.0;
        let mut grad = match eval_grad {
            true => Some(vec![0.0; 3 * natoms]),
            false => None,
        };
        let mut sigma = match eval_grad {
            true => Some(vec![0.0; 9]),
            false => None,
        };
        let mut error = DFTD3Error::new();
        unsafe {
            ffi::dftd3_get_counterpoise(
                error.get_c_ptr(),
                structure.ptr,
                self.ptr,
                &mut energy,
                grad.as_mut().map_or(null_mut(), |x| x.as_mut_ptr()),
                sigma.as_mut().map_or(null_mut(), |x| x.as_mut_ptr()),
            )
        };
        match error.check() {
            true => Err(error),
            false => Ok(DFTD3Output { energy, grad, sigma }),
        }
    }

    /// Load geometric counter-poise parameters from internal storage (failable)
    pub fn load_gcp_param_f(
        structure: DFTD3Structure,
        method: &str,
        basis: &str,
    ) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let token_method = std::ffi::CString::new(method).unwrap();
        let token_basis = std::ffi::CString::new(basis).unwrap();
        let ptr = unsafe {
            ffi::dftd3_load_gcp_param(
                error.get_c_ptr(),
                structure.ptr,
                token_method.into_raw(),
                token_basis.into_raw(),
            )
        };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr, structure }),
        }
    }

    /// Set realspace cutoffs (quantities in Bohr) (failable)
    pub fn set_realspace_cutoff_f(&self, bas: f64, srb: f64) -> Result<(), DFTD3Error> {
        let mut error = DFTD3Error::new();
        unsafe { ffi::dftd3_set_gcp_realspace_cutoff(error.get_c_ptr(), self.ptr, bas, srb) };
        match error.check() {
            true => Err(error),
            false => Ok(()),
        }
    }
}

/// Evaluate the counterpoise correction (failable)
pub fn get_counterpoise_f(
    structure: &DFTD3Structure,
    gcp: &DFTD3GCP,
) -> Result<(f64, Vec<f64>, Vec<f64>), DFTD3Error> {
    let natoms = structure.get_natoms();
    let mut energy = 0.0;
    let mut grad = vec![0.0; 3 * natoms];
    let mut sigma = vec![0.0; 9];
    let mut error = DFTD3Error::new();

    unsafe {
        ffi::dftd3_get_counterpoise(
            error.get_c_ptr(),
            structure.ptr,
            gcp.ptr,
            &mut energy,
            grad.as_mut_ptr(),
            sigma.as_mut_ptr(),
        )
    };
    match error.check() {
        true => Err(error),
        false => Ok((energy, grad, sigma)),
    }
}

/// Evaluate the counterpoise correction
pub fn get_counterpoise(structure: &DFTD3Structure, gcp: &DFTD3GCP) -> (f64, Vec<f64>, Vec<f64>) {
    get_counterpoise_f(structure, gcp).unwrap()
}

/* #endregion */
