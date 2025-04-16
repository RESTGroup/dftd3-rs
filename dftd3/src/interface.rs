use crate::ffi;
use derive_builder::{Builder, UninitializedFieldError};
use duplicate::duplicate_item;
use std::ffi::{c_char, c_int, CStr};
use std::ptr::{null, null_mut};
use std::result::Result;

/* #region DFTD3 version */

/// Get the version of the DFTD3 library.
///
/// The version is returned as a string in the format "major.minor.patch".
pub fn get_api_version() -> String {
    let version = unsafe { ffi::dftd3_get_version() };
    format!("{}.{}.{}", version / 10000, version / 100 % 100, version % 100)
}

/// Get the version of the DFTD3 library in list of integers (major, minor,
/// patch).
pub fn get_api_version_compact() -> [usize; 3] {
    let version = unsafe { ffi::dftd3_get_version() } as usize;
    [version / 10000, version / 100 % 100, version % 100]
}

/* #endregion */

/* #region DFTD3Error */

/// DFTD3 error class.
///
/// This is enum type to handle C error (from dftd3 itself) or rust error
/// (always presented as `String`).
pub enum DFTD3Error {
    C(ffi::dftd3_error),
    Rust(String),
    BuilderError(UninitializedFieldError),
}

impl From<UninitializedFieldError> for DFTD3Error {
    fn from(ufe: UninitializedFieldError) -> DFTD3Error {
        DFTD3Error::BuilderError(ufe)
    }
}

impl Drop for DFTD3Error {
    fn drop(&mut self) {
        if let DFTD3Error::C(ptr) = self {
            unsafe { ffi::dftd3_delete_error(&mut ptr.clone()) }
        }
    }
}

impl Default for DFTD3Error {
    fn default() -> Self {
        DFTD3Error::new()
    }
}

impl std::error::Error for DFTD3Error {}

impl DFTD3Error {
    pub fn new() -> Self {
        let ptr = unsafe { ffi::dftd3_new_error() };
        DFTD3Error::C(ptr)
    }

    /// Check if the error is set.
    ///
    /// True if the error is set, false otherwise.
    pub fn check(&self) -> bool {
        match self {
            DFTD3Error::C(ptr) => unsafe { ffi::dftd3_check_error(*ptr) != 0 },
            _ => true,
        }
    }

    pub fn get_c_ptr(&mut self) -> ffi::dftd3_error {
        match self {
            DFTD3Error::C(ptr) => *ptr,
            _ => std::ptr::null_mut(),
        }
    }

    pub fn get_message(&self) -> String {
        match self {
            DFTD3Error::C(ptr) => {
                const LEN_BUFFER: usize = 512;
                let buffer = [0u8; LEN_BUFFER];
                let raw = buffer.as_ptr() as *mut c_char;
                let msg = unsafe {
                    ffi::dftd3_get_error(*ptr, raw, &(LEN_BUFFER as c_int));
                    CStr::from_ptr(raw)
                };
                msg.to_string_lossy().to_string()
            },
            DFTD3Error::Rust(msg) => msg.clone(),
            DFTD3Error::BuilderError(ufe) => {
                format!("Builder error: {:?}", ufe)
            },
        }
    }
}

impl std::fmt::Debug for DFTD3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.check() {
            write!(f, "DFTD3Error: {}", self.get_message())
        } else {
            write!(f, "DFTD3Error: No error")
        }
    }
}

impl std::fmt::Display for DFTD3Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.check() {
            write!(f, "DFTD3Error: {}", self.get_message())
        } else {
            write!(f, "")
        }
    }
}

/* #endregion */

/* #region DFTD3Structure */

/// Molecular structure data.
///
/// Represents a wrapped structure object in `s-dftd3`. The molecular structure
/// data object has a fixed number of atoms and immutable atomic identifiers.
///
/// Note that except for number of atoms is stored in this struct, geometric
/// positions and lattice is not retrivable. API caller should handle these
/// information for themselves.
///
/// # Note
///
/// In most cases, this struct should not be used directly. Instead, use
/// [`DFTD3Model`].
///
/// # See also
///
/// Official python wrapper [`Structure`](https://github.com/dftd3/simple-dftd3/blob/v1.2.1/python/dftd3/interface.py#L31-L152).
pub struct DFTD3Structure {
    /// Pointer to the internal DFTD3 structure object.
    pub(crate) ptr: ffi::dftd3_structure,
    /// Number of atoms in the structure.
    natoms: usize,
}

impl Drop for DFTD3Structure {
    fn drop(&mut self) {
        unsafe { ffi::dftd3_delete_structure(&mut self.ptr) };
    }
}

impl DFTD3Structure {
    /// Create new molecular structure data from arrays (in Bohr).
    ///
    /// The returned object has immutable atomic species and boundary condition,
    /// also the total number of atoms cannot be changed.
    ///
    /// - `numbers` - element index (6 for O, 7 for N) in the structure
    /// - `positions` - atomic positions in Bohr (natom * 3)
    /// - `lattice` - optional, lattice parameters (3 * 3)
    /// - `periodic` - optional, periodicity (3)
    ///
    /// # See also
    ///
    /// [`DFTD3Model::new`]
    pub fn new(
        numbers: &[usize],
        positions: &[f64],
        lattice: Option<&[f64]>,
        periodic: Option<&[bool]>,
    ) -> Self {
        Self::new_f(numbers, positions, lattice, periodic).unwrap()
    }

    /// Update coordinates and lattice parameters (in Bohr).
    ///
    /// The lattice update is optional also for periodic structures.
    ///
    /// Generally, only the cartesian coordinates and the lattice parameters can
    /// be updated, every other modification, boundary condition, atomic types
    /// or number of atoms requires the complete reconstruction of the object.
    ///
    /// - `positions` - atomic positions in Bohr (natom * 3)
    /// - `lattice` - optional, lattice parameters (3 * 3)
    pub fn update(&mut self, positions: &[f64], lattice: Option<&[f64]>) {
        self.update_f(positions, lattice).unwrap()
    }

    /// Get number of atoms for this current structure.
    pub fn get_natoms(&self) -> usize {
        self.natoms
    }

    /// Create new molecular structure data from arrays (in Bohr, failable).
    ///
    /// # See also
    ///
    /// [`DFTD3Structure::new`]
    pub fn new_f(
        numbers: &[usize],
        positions: &[f64],
        lattice: Option<&[f64]>,
        periodic: Option<&[bool]>,
    ) -> Result<Self, DFTD3Error> {
        let natoms = numbers.len();
        // check dimension
        if positions.len() != 3 * natoms {
            return Err(DFTD3Error::Rust(format!(
                "Invalid dimension for positions, expected {}, got {}",
                3 * natoms,
                positions.len()
            )));
        }
        if lattice.is_some_and(|lattice| lattice.len() != 9) {
            return Err(DFTD3Error::Rust(format!(
                "Invalid dimension for lattice, expected 9, got {}",
                lattice.unwrap().len()
            )));
        }
        if periodic.is_some_and(|periodic| periodic.len() != 3) {
            return Err(DFTD3Error::Rust(format!(
                "Invalid dimension for periodic, expected 3, got {}",
                periodic.unwrap().len()
            )));
        }
        // unwrap optional values
        let lattice_ptr = lattice.map_or(null(), |x| x.as_ptr());
        let periodic_ptr = periodic.map_or(null(), |x| x.as_ptr());
        // type conversion from usual definitions
        let natoms_c_int = natoms as c_int;
        let atomic_numbers = numbers.iter().map(|&x| x as c_int).collect::<Vec<c_int>>();
        // actual driver for creating the structure
        let mut error = DFTD3Error::new();
        let ptr = unsafe {
            ffi::dftd3_new_structure(
                error.get_c_ptr(),
                natoms_c_int,
                atomic_numbers.as_ptr(),
                positions.as_ptr(),
                lattice_ptr,
                periodic_ptr,
            )
        };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr, natoms }),
        }
    }

    /// Update coordinates and lattice parameters (in Bohr, failable).
    ///
    /// # See also
    ///
    /// [`DFTD3Structure::update`]
    pub fn update_f(
        &mut self,
        positions: &[f64],
        lattice: Option<&[f64]>,
    ) -> Result<(), DFTD3Error> {
        // check dimension
        if positions.len() != 3 * self.natoms {
            return Err(DFTD3Error::Rust(format!(
                "Invalid dimension for positions, expected {}, got {}",
                3 * self.natoms,
                positions.len()
            )));
        }
        if lattice.is_some_and(|lattice| lattice.len() != 9) {
            return Err(DFTD3Error::Rust(format!(
                "Invalid dimension for lattice, expected 9, got {}",
                lattice.unwrap().len()
            )));
        }
        // unwrap optional values
        let lattice_ptr = lattice.map_or(null(), |x| x.as_ptr());
        // actual driver for updating the structure
        let mut error = DFTD3Error::new();
        unsafe {
            ffi::dftd3_update_structure(
                error.get_c_ptr(),
                self.ptr,
                positions.as_ptr(),
                lattice_ptr,
            )
        };
        match error.check() {
            true => Err(error),
            false => Ok(()),
        }
    }
}

/* #endregion */

/* #region DFTD3Param */

/// Basic struct for damping parameters, representing a parametrization of
/// a DFT-D3 method.
///
/// The damping parameters contained in the object are immutable. To change the
/// parametrization, a new object must be created. Furthermore, the object is
/// opaque to the user and the contained data cannot be accessed directly.
///
/// # Note
///
/// This struct is better not be initialized by user. Structs that implements
/// [`DFTD3ParamAPI`] should be used for parameter initialization.
///
/// This struct is considered as a low-level parameter interface. This object
/// does not have its official python wrapper correspondent. So use this struct
/// with caution.
///
/// Official python wrapper provides (not exactly) abstract class
/// `DampingParam`, which corresponds [`DFTD3ParamAPI`] in this project.
pub struct DFTD3Param {
    ptr: ffi::dftd3_param,
}

impl Drop for DFTD3Param {
    fn drop(&mut self) {
        unsafe { ffi::dftd3_delete_param(&mut self.ptr) };
    }
}

impl DFTD3Param {
    /// Create new zero damping parameters (failable)
    pub fn new_zero_damping_f(
        s6: f64,
        s8: f64,
        s9: f64,
        rs6: f64,
        rs8: f64,
        alp: f64,
    ) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let ptr =
            unsafe { ffi::dftd3_new_zero_damping(error.get_c_ptr(), s6, s8, s9, rs6, rs8, alp) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Create new zero damping parameters
    pub fn new_zero_damping(s6: f64, s8: f64, s9: f64, rs6: f64, rs8: f64, alp: f64) -> Self {
        Self::new_zero_damping_f(s6, s8, s9, rs6, rs8, alp).unwrap()
    }

    /// Load zero damping parameters from internal storage (failable)
    pub fn load_zero_damping_f(method: &str, atm: bool) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let token = std::ffi::CString::new(method).unwrap();
        let ptr = unsafe { ffi::dftd3_load_zero_damping(error.get_c_ptr(), token.into_raw(), atm) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Load zero damping parameters from internal storage
    pub fn load_zero_damping(method: &str, atm: bool) -> Self {
        Self::load_zero_damping_f(method, atm).unwrap()
    }

    /// Create new rational damping parameters (failable)
    pub fn new_rational_damping_f(
        s6: f64,
        s8: f64,
        s9: f64,
        a1: f64,
        a2: f64,
        alp: f64,
    ) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let ptr =
            unsafe { ffi::dftd3_new_rational_damping(error.get_c_ptr(), s6, s8, s9, a1, a2, alp) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Create new rational damping parameters
    pub fn new_rational_damping(s6: f64, s8: f64, s9: f64, a1: f64, a2: f64, alp: f64) -> Self {
        Self::new_rational_damping_f(s6, s8, s9, a1, a2, alp).unwrap()
    }

    /// Load rational damping parameters from internal storage (failable)
    pub fn load_rational_damping_f(method: &str, atm: bool) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let token = std::ffi::CString::new(method).unwrap();
        let ptr =
            unsafe { ffi::dftd3_load_rational_damping(error.get_c_ptr(), token.into_raw(), atm) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Load rational damping parameters from internal storage
    pub fn load_rational_damping(method: &str, atm: bool) -> Self {
        Self::load_rational_damping_f(method, atm).unwrap()
    }

    /// Create new modified zero damping parameters (failable)
    pub fn new_mzero_damping_f(
        s6: f64,
        s8: f64,
        s9: f64,
        rs6: f64,
        rs8: f64,
        alp: f64,
        bet: f64,
    ) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let ptr = unsafe {
            ffi::dftd3_new_mzero_damping(error.get_c_ptr(), s6, s8, s9, rs6, rs8, alp, bet)
        };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Create new modified zero damping parameters
    pub fn new_mzero_damping(
        s6: f64,
        s8: f64,
        s9: f64,
        rs6: f64,
        rs8: f64,
        alp: f64,
        bet: f64,
    ) -> Self {
        Self::new_mzero_damping_f(s6, s8, s9, rs6, rs8, alp, bet).unwrap()
    }

    /// Load modified zero damping parameters from internal storage (failable)
    pub fn load_mzero_damping_f(method: &str, atm: bool) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let token = std::ffi::CString::new(method).unwrap();
        let ptr =
            unsafe { ffi::dftd3_load_mzero_damping(error.get_c_ptr(), token.into_raw(), atm) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Load modified zero damping parameters from internal storage
    pub fn load_mzero_damping(method: &str, atm: bool) -> Self {
        Self::load_mzero_damping_f(method, atm).unwrap()
    }

    /// Create new modified rational damping parameters (failable)
    pub fn new_mrational_damping_f(
        s6: f64,
        s8: f64,
        s9: f64,
        a1: f64,
        a2: f64,
        alp: f64,
    ) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let ptr =
            unsafe { ffi::dftd3_new_mrational_damping(error.get_c_ptr(), s6, s8, s9, a1, a2, alp) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Create new modified rational damping parameters
    pub fn new_mrational_damping(s6: f64, s8: f64, s9: f64, a1: f64, a2: f64, alp: f64) -> Self {
        Self::new_mrational_damping_f(s6, s8, s9, a1, a2, alp).unwrap()
    }

    /// Load modified rational damping parameters from internal storage
    /// (failable)
    pub fn load_mrational_damping_f(method: &str, atm: bool) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let token = std::ffi::CString::new(method).unwrap();
        let ptr =
            unsafe { ffi::dftd3_load_mrational_damping(error.get_c_ptr(), token.into_raw(), atm) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Load modified rational damping parameters from internal storage
    pub fn load_mrational_damping(method: &str, atm: bool) -> Self {
        Self::load_mrational_damping_f(method, atm).unwrap()
    }

    /// Create new optimized damping parameters (failable)
    pub fn new_optimizedpower_damping_f(
        s6: f64,
        s8: f64,
        s9: f64,
        a1: f64,
        a2: f64,
        alp: f64,
        bet: f64,
    ) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let ptr = unsafe {
            ffi::dftd3_new_optimizedpower_damping(error.get_c_ptr(), s6, s8, s9, a1, a2, alp, bet)
        };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Create new optimized damping parameters
    pub fn new_optimizedpower_damping(
        s6: f64,
        s8: f64,
        s9: f64,
        a1: f64,
        a2: f64,
        alp: f64,
        bet: f64,
    ) -> Self {
        Self::new_optimizedpower_damping_f(s6, s8, s9, a1, a2, alp, bet).unwrap()
    }

    /// Load optimized damping parameters from internal storage (failable)
    pub fn load_optimizedpower_damping_f(method: &str, atm: bool) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let token = std::ffi::CString::new(method).unwrap();
        let ptr = unsafe {
            ffi::dftd3_load_optimizedpower_damping(error.get_c_ptr(), token.into_raw(), atm)
        };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr }),
        }
    }

    /// Load optimized damping parameters from internal storage
    pub fn load_optimizedpower_damping(method: &str, atm: bool) -> Self {
        Self::load_optimizedpower_damping_f(method, atm).unwrap()
    }
}

/* #endregion */

/* #region DFTD3 Damping derivatives */

/// Load damping parameters by functional and DFT-D3 versions.
///
/// Available versions are:
/// - `d3bj`: rational damping;
/// - `d3zero`: zero damping;
/// - `d3bjm`: modified rational damping;
/// - `d3zerom`: modified zero damping;
/// - `d3op`: optimized power damping.
pub fn dftd3_load_param(version: &str, method: &str, atm: bool) -> DFTD3Param {
    dftd3_load_param_f(version, method, atm).unwrap()
}

/// Load damping parameters by functional and DFT-D3 versions.
///
/// # See also
///
/// [`dftd3_load_param`]
pub fn dftd3_load_param_f(
    version: &str,
    method: &str,
    atm: bool,
) -> Result<DFTD3Param, DFTD3Error> {
    let version = version.to_lowercase().replace(['-', '_', ' '], "");
    match version.as_str() {
        "d3bj" | "bj" => DFTD3Param::load_rational_damping_f(method, atm),
        "d3zero" | "zero" => DFTD3Param::load_zero_damping_f(method, atm),
        "d3bjm" | "d3mbj" | "bjm" | "mbj" => DFTD3Param::load_mrational_damping_f(method, atm),
        "d3zerom" | "d3mzero" | "zerom" | "mzero" => DFTD3Param::load_mzero_damping_f(method, atm),
        "d3op" | "op" => DFTD3Param::load_optimizedpower_damping_f(method, atm),
        _ => Err(DFTD3Error::Rust(format!("Unknown DFTD3 version: {}", version))),
    }
}

/// Trait for damping parameters by custom parameters.
pub trait DFTD3ParamAPI {
    fn new_param_f(self) -> Result<DFTD3Param, DFTD3Error>;
    fn new_param(self) -> DFTD3Param
    where
        Self: Sized,
    {
        self.new_param_f().unwrap()
    }
}

/// Trait for loading damping parameters.
pub trait DFTD3LoadParamAPI {
    fn load_param_f(method: &str, atm: bool) -> Result<DFTD3Param, DFTD3Error>;
    fn load_param(method: &str, atm: bool) -> DFTD3Param {
        Self::load_param_f(method, atm).unwrap()
    }
}

#[derive(Builder, Debug, Clone)]
#[builder(pattern = "owned", build_fn(error = "DFTD3Error"))]
pub struct DFTD3RationalDampingParam {
    #[builder(default = 1.0)]
    pub s6: f64,
    pub s8: f64,
    #[builder(default = 1.0)]
    pub s9: f64,
    pub a1: f64,
    pub a2: f64,
    #[builder(default = 14.0)]
    pub alp: f64,
}

impl DFTD3ParamAPI for DFTD3RationalDampingParam {
    fn new_param_f(self) -> Result<DFTD3Param, DFTD3Error> {
        let Self { s6, s8, s9, a1, a2, alp } = self;
        DFTD3Param::new_rational_damping_f(s6, s8, s9, a1, a2, alp)
    }
}

/// Original DFT-D3 damping function,\ :footcite:`grimme2010` based on a variant
/// proposed by Chai and Head-Gordon.\ :footcite:`chai2008`
/// Since it is damping the dispersion energy to zero at short distances it is
/// usually called zero damping scheme for simplicity. However, due to this
/// short-range limit of the dispersion energy a repulsive contribution to the
/// gradient can arise, which is considered artificial.\ :footcite:`grimme2011`
#[derive(Builder, Debug, Clone)]
#[builder(pattern = "owned", build_fn(error = "DFTD3Error"))]
pub struct DFTD3ZeroDampingParam {
    #[builder(default = 1.0)]
    pub s6: f64,
    pub s8: f64,
    #[builder(default = 1.0)]
    pub s9: f64,
    pub rs6: f64,
    #[builder(default = 1.0)]
    pub rs8: f64,
    #[builder(default = 14.0)]
    pub alp: f64,
}

impl DFTD3ParamAPI for DFTD3ZeroDampingParam {
    fn new_param_f(self) -> Result<DFTD3Param, DFTD3Error> {
        let Self { s6, s8, s9, rs6, rs8, alp } = self;
        DFTD3Param::new_zero_damping_f(s6, s8, s9, rs6, rs8, alp)
    }
}

/// Modified version of the rational damping parameters. The functional form of
/// the damping function is *unmodified* with respect to the original rational
/// damping scheme. However, for a number of functionals new parameters were
/// introduced.:footcite:`smith2016`
///
/// This constructor allows to automatically load the reparameterized damping
/// function from the library rather than the original one. Providing a full
/// parameter set is functionally equivalent to using the `RationalDampingParam`
/// constructor.
#[derive(Builder, Debug, Clone)]
#[builder(pattern = "owned", build_fn(error = "DFTD3Error"))]
pub struct DFTD3ModifiedRationalDampingParam {
    #[builder(default = 1.0)]
    pub s6: f64,
    pub s8: f64,
    #[builder(default = 1.0)]
    pub s9: f64,
    pub a1: f64,
    pub a2: f64,
    #[builder(default = 14.0)]
    pub alp: f64,
}

impl DFTD3ParamAPI for DFTD3ModifiedRationalDampingParam {
    fn new_param_f(self) -> Result<DFTD3Param, DFTD3Error> {
        let Self { s6, s8, s9, a1, a2, alp } = self;
        DFTD3Param::new_mrational_damping_f(s6, s8, s9, a1, a2, alp)
    }
}

/// Modified zero damping function for DFT-D3.\ :footcite:`smith2016`
/// This scheme adds an additional offset parameter to the zero damping scheme
/// of the original DFT-D3.
///
/// .. note::
///
///    This damping function is identical to zero damping for ``bet=0.0``.
#[derive(Builder, Debug, Clone)]
#[builder(pattern = "owned", build_fn(error = "DFTD3Error"))]
pub struct DFTD3ModifiedZeroDampingParam {
    #[builder(default = 1.0)]
    pub s6: f64,
    pub s8: f64,
    #[builder(default = 1.0)]
    pub s9: f64,
    pub rs6: f64,
    #[builder(default = 1.0)]
    pub rs8: f64,
    #[builder(default = 14.0)]
    pub alp: f64,
    pub bet: f64,
}

impl DFTD3ParamAPI for DFTD3ModifiedZeroDampingParam {
    fn new_param_f(self) -> Result<DFTD3Param, DFTD3Error> {
        let Self { s6, s8, s9, rs6, rs8, alp, bet } = self;
        DFTD3Param::new_mzero_damping_f(s6, s8, s9, rs6, rs8, alp, bet)
    }
}

/// Optimized power version of the rational damping parameters.\
/// :footcite:`witte2017` The functional form of the damping function is
/// modified by adding an additional zero-damping like power function.
///
/// This constructor allows to automatically load the reparameterized damping
/// function from the library rather than the original one. Providing the
/// parameter `bet=0` is equivalent to using rational the `RationalDampingParam`
/// constructor.
#[derive(Builder, Debug, Clone)]
#[builder(pattern = "owned", build_fn(error = "DFTD3Error"))]
pub struct DFTD3OptimizedPowerDampingParam {
    #[builder(default = 1.0)]
    pub s6: f64,
    pub s8: f64,
    #[builder(default = 1.0)]
    pub s9: f64,
    pub a1: f64,
    pub a2: f64,
    #[builder(default = 14.0)]
    pub alp: f64,
    pub bet: f64,
}

impl DFTD3ParamAPI for DFTD3OptimizedPowerDampingParam {
    fn new_param_f(self) -> Result<DFTD3Param, DFTD3Error> {
        let Self { s6, s8, s9, a1, a2, alp, bet } = self;
        DFTD3Param::new_optimizedpower_damping_f(s6, s8, s9, a1, a2, alp, bet)
    }
}

#[duplicate_item(
     DampingParam                        load_damping_f                ;
    [DFTD3RationalDampingParam        ] [load_rational_damping_f      ];
    [DFTD3ZeroDampingParam            ] [load_zero_damping_f          ];
    [DFTD3ModifiedRationalDampingParam] [load_mrational_damping_f     ];
    [DFTD3ModifiedZeroDampingParam    ] [load_mzero_damping_f         ];
    [DFTD3OptimizedPowerDampingParam  ] [load_optimizedpower_damping_f];
)]
impl DFTD3LoadParamAPI for DampingParam {
    fn load_param_f(method: &str, atm: bool) -> Result<DFTD3Param, DFTD3Error> {
        DFTD3Param::load_damping_f(method, atm)
    }
}
#[duplicate_item(
    DampingParamBuilder;
    [DFTD3RationalDampingParamBuilder];
    [DFTD3ZeroDampingParamBuilder];
    [DFTD3ModifiedRationalDampingParamBuilder];
    [DFTD3ModifiedZeroDampingParamBuilder];
    [DFTD3OptimizedPowerDampingParamBuilder];
)]
impl DampingParamBuilder {
    pub fn init(self) -> DFTD3Param {
        self.init_f().unwrap()
    }

    pub fn init_f(self) -> Result<DFTD3Param, DFTD3Error> {
        self.build()?.new_param_f()
    }
}

/* #endregion */

/* #region DFTD3 outputs */

/// DFTD3 returned result.
///
/// This struct implements `From` trait to convert to tuple. So you can use this
/// struct in this way:
///
/// ```ignore
/// let (energy, grad, sigma) = dftd3_model.get_dispersion(param, eval_grad).into();
/// ```
pub struct DFTD3Output {
    /// Dispersion energy.
    pub energy: f64,
    /// Gradient of the dispersion energy (natom * 3).
    pub grad: Option<Vec<f64>>,
    /// Strain derivatives (3 * 3).
    pub sigma: Option<Vec<f64>>,
}

impl From<DFTD3Output> for (f64, Option<Vec<f64>>, Option<Vec<f64>>) {
    fn from(output: DFTD3Output) -> Self {
        (output.energy, output.grad, output.sigma)
    }
}

/// DFTD3 pairwise returned result.
///
/// This struct implements `From` trait to convert to tuple. So you can use this
/// struct in this way:
///
/// ```ignore
/// let (pair_energy2, pair_energy3) = dftd3_model.get_pairwise_dispersion(param).into();
/// ```
pub struct DFTD3PairwiseOutput {
    /// Pairwise additive pairwise energy (natom * natom)
    pub pair_energy2: Vec<f64>,
    /// Pairwise non-additive pairwise energy (natom * natom)
    pub pair_energy3: Vec<f64>,
}

impl From<DFTD3PairwiseOutput> for (Vec<f64>, Vec<f64>) {
    fn from(output: DFTD3PairwiseOutput) -> Self {
        (output.pair_energy2, output.pair_energy3)
    }
}

/* #endregion */

/* #region DFTD3Model */

/// DFTD3 dispersion model.
///
/// Contains the required information to evaluate all dispersion related
/// properties, like C6 coefficients. It also manages an instance of the
/// geometry it was constructed for to ensure that the dispersion model is
/// always used with the correct structure input.
///
/// # See also
///
/// Official python wrapper [`DispersionModel`](https://github.com/dftd3/simple-dftd3/blob/v1.2.1/python/dftd3/interface.py#L387-L459).
pub struct DFTD3Model {
    /// Pointer to the internal DFTD3 model object.
    ptr: ffi::dftd3_model,
    /// Internal DFTD3 structure object.
    structure: DFTD3Structure,
}

impl Drop for DFTD3Model {
    fn drop(&mut self) {
        unsafe { ffi::dftd3_delete_model(&mut self.ptr) };
    }
}

impl DFTD3Model {
    /// Create new molecular structure data and module from arrays (in Bohr).
    ///
    /// The returned object has immutable atomic species and boundary condition,
    /// also the total number of atoms cannot be changed.
    ///
    /// - `numbers` - element index (6 for O, 7 for N) in the structure
    /// - `positions` - atomic positions in Bohr (natom * 3)
    /// - `lattice` - optional, lattice parameters (3 * 3)
    /// - `periodic` - optional, periodicity (3)
    pub fn new(
        numbers: &[usize],
        positions: &[f64],
        lattice: Option<&[f64]>,
        periodic: Option<&[bool]>,
    ) -> Self {
        Self::new_f(numbers, positions, lattice, periodic).unwrap()
    }

    /// Evaluate the dispersion energy and its derivatives.
    ///
    /// Output `DFTD3Output` contains
    ///
    /// - `energy` - dispersion energy
    /// - `grad` - gradient of the dispersion energy (natom * 3)
    /// - `sigma` - strain derivatives (3 * 3)
    pub fn get_dispersion(&self, param: &DFTD3Param, eval_grad: bool) -> DFTD3Output {
        self.get_dispersion_f(param, eval_grad).unwrap()
    }

    /// Evaluate the pairwise dispersion energy.
    ///
    /// Output `DFTD3PairwiseOutput` contains
    ///
    /// - `pair_energy2` - pairwise additive pairwise energy (natom * natom)
    /// - `pair_energy3` - pairwise non-additive pairwise energy (natom * natom)
    pub fn get_pairwise_dispersion(&self, param: &DFTD3Param) -> DFTD3PairwiseOutput {
        self.get_pairwise_dispersion_f(param).unwrap()
    }

    /// Set realspace cutoff for evaluation of interactions (in Bohr)
    pub fn set_realspace_cutoff(&self, r0: f64, r1: f64, r2: f64) {
        self.set_realspace_cutoff_f(r0, r1, r2).unwrap()
    }

    /// Get number of atoms for this current structure.
    pub fn get_natoms(&self) -> usize {
        self.structure.get_natoms()
    }

    /// Create new D3 dispersion model from structure.
    pub fn from_structure(structure: DFTD3Structure) -> Self {
        Self::from_structure_f(structure).unwrap()
    }

    /// Update coordinates and lattice parameters (in Bohr).
    ///
    /// The lattice update is optional also for periodic structures.
    ///
    /// Generally, only the cartesian coordinates and the lattice parameters can
    /// be updated, every other modification, boundary condition, atomic types
    /// or number of atoms requires the complete reconstruction of the object.
    ///
    /// - `positions` - atomic positions in Bohr (natom * 3)
    /// - `lattice` - optional, lattice parameters (3 * 3)
    pub fn update(&mut self, positions: &[f64], lattice: Option<&[f64]>) {
        self.structure.update(positions, lattice)
    }

    /// Create new molecular structure data and module from arrays (in Bohr,
    /// failable).
    ///
    /// # See also
    ///
    /// [`DFTD3Model::new`]
    pub fn new_f(
        numbers: &[usize],
        positions: &[f64],
        lattice: Option<&[f64]>,
        periodic: Option<&[bool]>,
    ) -> Result<Self, DFTD3Error> {
        let structure = DFTD3Structure::new_f(numbers, positions, lattice, periodic)?;
        Self::from_structure_f(structure)
    }

    /// Evaluate the dispersion energy and its derivatives (failable).
    ///
    /// # See also
    ///
    /// [`DFTD3Model::get_dispersion`]
    pub fn get_dispersion_f(
        &self,
        param: &DFTD3Param,
        eval_grad: bool,
    ) -> Result<DFTD3Output, DFTD3Error> {
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
            ffi::dftd3_get_dispersion(
                error.get_c_ptr(),
                structure.ptr,
                self.ptr,
                param.ptr,
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

    /// Evaluate the pairwise dispersion energy (failable).
    pub fn get_pairwise_dispersion_f(
        &self,
        param: &DFTD3Param,
    ) -> Result<DFTD3PairwiseOutput, DFTD3Error> {
        let structure = &self.structure;
        let natoms = structure.get_natoms();
        let mut pair_energy2 = vec![0.0; natoms * natoms];
        let mut pair_energy3 = vec![0.0; natoms * natoms];
        let mut error = DFTD3Error::new();

        unsafe {
            ffi::dftd3_get_pairwise_dispersion(
                error.get_c_ptr(),
                structure.ptr,
                self.ptr,
                param.ptr,
                pair_energy2.as_mut_ptr(),
                pair_energy3.as_mut_ptr(),
            )
        };
        match error.check() {
            true => Err(error),
            false => Ok(DFTD3PairwiseOutput { pair_energy2, pair_energy3 }),
        }
    }

    /// Set realspace cutoff for evaluation of interactions (in Bohr, failable).
    ///
    /// # See also
    ///
    /// [`DFTD3Model::set_realspace_cutoff`]
    pub fn set_realspace_cutoff_f(
        &self,
        disp2: f64,
        disp3: f64,
        cn: f64,
    ) -> Result<(), DFTD3Error> {
        let mut error = DFTD3Error::new();
        unsafe {
            ffi::dftd3_set_model_realspace_cutoff(error.get_c_ptr(), self.ptr, disp2, disp3, cn)
        };
        match error.check() {
            true => Err(error),
            false => Ok(()),
        }
    }

    /// Create new D3 dispersion model from structure (failable).
    ///
    /// # See also
    ///
    /// [`DFTD3Model::from_structure`]
    pub fn from_structure_f(structure: DFTD3Structure) -> Result<Self, DFTD3Error> {
        let mut error = DFTD3Error::new();
        let ptr = unsafe { ffi::dftd3_new_d3_model(error.get_c_ptr(), structure.ptr) };
        match error.check() {
            true => Err(error),
            false => Ok(Self { ptr, structure }),
        }
    }

    /// Update coordinates and lattice parameters (in Bohr, failable).
    ///
    /// # See also
    ///
    /// [`DFTD3Model::update`]
    pub fn update_f(
        &mut self,
        positions: &[f64],
        lattice: Option<&[f64]>,
    ) -> Result<(), DFTD3Error> {
        self.structure.update_f(positions, lattice)
    }
}

/* #endregion */

#[cfg(test)]
mod tests {
    use ffi::dftd3_load_optimizedpower_damping;

    use super::*;

    #[test]
    fn test_get_api_version() {
        println!("API version: {}", get_api_version());
    }

    #[test]
    fn test_get_api_version_compact() {
        println!("API version: {:?}", get_api_version_compact());
    }

    #[test]
    fn test_dftd3_error() {
        let mut error = DFTD3Error::new();
        println!("Error check   : {}", error.check());
        println!("Error message : {}", error.get_message());
        let token = std::ffi::CString::new("Hello").unwrap();
        unsafe { dftd3_load_optimizedpower_damping(error.get_c_ptr(), token.into_raw(), false) };
        println!("Error check   : {}", error.check());
        println!("Error message : {}", error.get_message());
        let token = std::ffi::CString::new("B3LYP").unwrap();
        unsafe { dftd3_load_optimizedpower_damping(error.get_c_ptr(), token.into_raw(), false) };
        println!("Error check   : {}", error.check());
        println!("Error message : {}", error.get_message());
    }

    #[test]
    fn test_get_dispersion() {
        let numbers = vec![1, 1];
        let positions = vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        let model = DFTD3Model::new(&numbers, &positions, None, None);
        let param = DFTD3Param::load_mrational_damping("B3LYP", false);
        let (energy, grad, sigma) = model.get_dispersion(&param, true).into();
        println!("Dispersion energy: {}", energy);
        println!("Dispersion gradient: {:?}", grad);
        println!("Dispersion sigma: {:?}", sigma);
    }
}
