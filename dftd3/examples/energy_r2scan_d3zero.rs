use dftd3::prelude::*;

fn main_test() {
    // atom indices
    let numbers = vec![6, 6, 6, 6, 6, 6, 53, 1, 1, 1, 1, 1, 16, 1, 6, 1, 1, 1];
    // geometry in angstrom
    #[rustfmt::skip]
    let positions = vec![
        -0.755422531,  -0.796459123,  -1.023590391,
         0.634274834,  -0.880017014,  -1.075233285,
         1.406955202,   0.199695367,  -0.653144334,
         0.798863737,   1.361204515,  -0.180597909,
        -0.593166787,   1.434312023,  -0.133597923,
        -1.376239198,   0.359205222,  -0.553258516,
        -1.514344238,   3.173268101,   0.573601106,
         1.110906949,  -1.778801728,  -1.440619836,
         1.399172302,   2.197767355,   0.147412751,
         2.486417780,   0.142466525,  -0.689380574,
        -2.454252250,   0.422581120,  -0.512807958,
        -1.362353593,  -1.630564523,  -1.348743149,
        -3.112683203,   6.289227834,   1.226984439,
        -4.328789697,   5.797771251,   0.973373089,
        -2.689135032,   6.703163830,  -0.489062886,
        -1.684433029,   7.115457372,  -0.460265708,
        -2.683867206,   5.816530502,  -1.115183775,
        -3.365330613,   7.451201412,  -0.890098894,
    ];
    // convert angstrom to bohr
    let positions = positions.iter().map(|&x| x / 0.52917721067).collect::<Vec<f64>>();
    // generate DFTD3 model
    let model = DFTD3Model::new(&numbers, &positions, None, None);
    // explicitly set DFTD3 parameters
    for atm in [true, false] {
        let energy_ref = if atm { -0.01410721853585842 } else { -0.014100267345314462 };

        let param = DFTD3ZeroDampingParamBuilder::default()
            .s8(1.683)
            .rs6(1.139)
            .s9(if atm { 1.0 } else { 0.0 })
            .init();
        // obtain the dispersion energy without gradient and sigma
        let (energy, _, _) = model.get_dispersion(&param, false).into();

        println!("Dispersion energy: {}", energy);
        assert!((energy - energy_ref).abs() < 1e-9);

        // this way to provide custom damping parameter is also valid
        let param = DFTD3ZeroDampingParam {
            s6: 1.0,
            s8: 1.683,
            rs6: 1.139,
            rs8: 1.0,
            alp: 14.0,
            s9: if atm { 1.0 } else { 0.0 },
        };
        let param = param.new_param();
        // obtain the dispersion energy without gradient and sigma
        let (energy, _, _) = model.get_dispersion(&param, false).into();

        println!("Dispersion energy: {}", energy);
        assert!((energy - energy_ref).abs() < 1e-9);

        // this way to provide custom damping parameter is also valid
        let param = DFTD3Param::new_zero_damping(
            1.0,                         // s6
            1.683,                       // s8
            if atm { 1.0 } else { 0.0 }, // s9
            1.139,                       // rs6
            1.0,                         // rs8
            14.0,                        // alp
        );
        // obtain the dispersion energy without gradient and sigma
        let (energy, _, _) = model.get_dispersion(&param, false).into();

        println!("Dispersion energy: {}", energy);
        assert!((energy - energy_ref).abs() < 1e-9);
    }
}

#[test]
fn test() {
    main_test();
}

fn main() {
    main_test();
}

/* equivalent PySCF with dftd3

// https://github.com/dftd3/simple-dftd3/blob/v1.2.1/python/dftd3/test_pyscf.py#L59-L102

```python
import pyscf
from pyscf import gto
import dftd3.pyscf as disp


mol = gto.M(atom="""
    C   -0.755422531  -0.796459123  -1.023590391
    C    0.634274834  -0.880017014  -1.075233285
    C    1.406955202   0.199695367  -0.653144334
    C    0.798863737   1.361204515  -0.180597909
    C   -0.593166787   1.434312023  -0.133597923
    C   -1.376239198   0.359205222  -0.553258516
    I   -1.514344238   3.173268101   0.573601106
    H    1.110906949  -1.778801728  -1.440619836
    H    1.399172302   2.197767355   0.147412751
    H    2.486417780   0.142466525  -0.689380574
    H   -2.454252250   0.422581120  -0.512807958
    H   -1.362353593  -1.630564523  -1.348743149
    S   -3.112683203   6.289227834   1.226984439
    H   -4.328789697   5.797771251   0.973373089
    C   -2.689135032   6.703163830  -0.489062886
    H   -1.684433029   7.115457372  -0.460265708
    H   -2.683867206   5.816530502  -1.115183775
    H   -3.365330613   7.451201412  -0.890098894
""")

for atm in (True, False):
    d3 = disp.DFTD3Dispersion(
        mol,
        param={
            "s6": 1.0,
            "s8": 1.683,
            "rs6": 1.139,
            "rs8": 1.0,
            "alp": 14.0,
            "s9": 1.0 if atm else 0.0,
        },
        version="d3zero",
    )
    print(d3.kernel()[0])
```

*/
