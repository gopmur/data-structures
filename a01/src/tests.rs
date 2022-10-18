use super::*;

// ---------- Ltm ---------- //
#[test]
fn ltm_from() {
    let m = mat![
        1, 0, 0, 0 ;
        2, 3, 0, 0 ;
        4, 5, 6, 0 ;
        7, 8, 9, 10;
    ];
    let m = Ltm::from(&m).unwrap();
    assert_eq!(
        m,
        Ltm {
            data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            size: 4,
        }
    );
}

#[test]
#[should_panic]
fn ltm_from_panic_not_square() {
    let m = mat![
        1, 0, 0, 0 ;
        2, 3, 0, 0 ;
        4, 5, 6, 0 ;
    ];
    Ltm::from(&m).unwrap();
}

#[test]
#[should_panic]
fn ltm_from_panic_not_ltm() {
    let m = mat![
        1, 2, 3, 4 ;
        0, 5, 6, 7 ;
        0, 0, 8, 9 ;
        0, 0, 0, 10;
    ];
    Ltm::from(&m).unwrap();
}

#[test]
fn ltm_to_vec() {
    let m = mat![
        1, 0, 0, 0 ;
        2, 3, 0, 0 ;
        4, 5, 6, 0 ;
        7, 8, 9, 10;
    ];
    let ltm = Ltm {
        data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        size: 4,
    };
    assert_eq!(m, ltm.to_vec());
}

#[test]
fn ltm_is_ltm() { 
    let m = mat![
        1, 0, 0, 0 ;
        2, 3, 0, 0 ;
        4, 5, 6, 0 ;
        7, 8, 9, 10;
    ];
    assert_eq!(Ltm::is_ltm(&m), true);
    let m = mat![
        1, 2, 3, 4 ;
        0, 5, 6, 7 ;
        0, 0, 8, 9 ;
        0, 0, 0, 10;
    ];
    assert_eq!(Ltm::is_ltm(&m), false);
}

// -------- Htm test -------- //
#[test]
fn htm_from() {
    let m = mat![
        1, 2, 3, 4 ;
        0, 5, 6, 7 ;
        0, 0, 8, 9 ;
        0, 0, 0, 10;
    ];
    let m = Htm::from(&m).unwrap();
    assert_eq!(
        m,
        Htm {
            data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            size: 4,
        }
    );
}

#[test]
#[should_panic]
fn htm_from_panic_not_square() {
    let m = mat![
        1, 0, 0, 0 ;
        2, 3, 0, 0 ;
        4, 5, 6, 0 ;
    ];
    Htm::from(&m).unwrap();
}

#[test]
#[should_panic]
fn htm_from_panic_not_ltm() {
    let m = mat![
        1, 0, 0, 0 ;
        2, 3, 0, 0 ;
        4, 5, 6, 0 ;
        7, 8, 9, 10;
    ];
    Htm::from(&m).unwrap();
}

#[test]
fn htm_to_vec() {
    let m = mat![
        1, 2, 3, 4 ;
        0, 5, 6, 7 ;
        0, 0, 8, 9 ;
        0, 0, 0, 10;
    ];
    let htm = Htm {
        data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
        size: 4,
    };
    assert_eq!(m, htm.to_vec());
}

#[test]
fn htm_is_htm() { 
    let m = mat![
        1, 2, 3, 4 ;
        0, 5, 6, 7 ;
        0, 0, 8, 9 ;
        0, 0, 0, 10;
    ];
    assert_eq!(Htm::is_htm(&m), true);
    let m = mat![
        1, 0, 0, 0 ;
        2, 3, 0, 0 ;
        4, 5, 6, 0 ;
        7, 8, 9, 10;
    ];
    assert_eq!(Htm::is_htm(&m), false);
}

// ----- SparseMatrix ------ //
#[test]
fn sparse_matrix_from() {
    let m = mat![
        0, 0, 2, 4;
        0, 1, 0, 0;
        0, 0, 0, 0;
        0, 0, 3, 0;
        0, 0, 0, 0;
    ];
    assert_eq!(
        SparseMatrix::from(&m), 
        SparseMatrix {
            size: (5, 4),
            non_zero: 4,
            data: mat![
                0, 2, 2;
                0, 3, 4;
                1, 1, 1;
                3, 2, 3;
            ]
        }
    );
}

#[test]
fn sparse_matrix_transpose() {
    let m = mat![
        0, 0, 2, 4;
        0, 1, 0, 0;
        0, 0, 0, 0;
        0, 0, 3, 0;
        0, 0, 0, 0;
    ];
    let m = SparseMatrix::from(&m);
    assert_eq!(
        m.transpose(), 
        SparseMatrix {
            size: (4, 5),
            non_zero: 4,
            data: mat![
                1, 1, 1;
                2, 0, 2;
                2, 3, 3;
                3, 0, 4;
            ]
        }
    );
}

#[test]
fn sparse_matrix_mul() {
    /*
        a = 0 0 2 4
            0 1 0 0
            0 0 0 0
            0 0 3 0
            0 0 0 0
            
        b = 8 0 0 7 0
            0 0 6 0 0
            0 5 0 0 0
            0 0 0 9 0
            
        a * b = 0  10  0  36  0
                0   0  6   0  0
                0   0  0   0  0
                0  15  0   0  0
                0   0  0   0  0

    */
    let a = SparseMatrix {
        size: (5, 4),
        non_zero: 4,
        data: mat![
            0, 2, 2;
            0, 3, 4;
            1, 1, 1;
            3, 2, 3;
        ]
    };

    let b = SparseMatrix {
        size: (4, 5),
        non_zero: 5,
        data: mat![
            0, 0, 8;
            0, 3, 7;
            1, 2, 6;
            2, 1, 5;
            3, 3, 9;
        ]
    };
    
    assert_eq!(
        SparseMatrix::mul(&a, &b).unwrap(),
        SparseMatrix {
            size: (5, 5),
            non_zero: 4,
            data: mat![
                0, 1, 10;
                0, 3, 36;
                1, 2, 6 ;
                3, 1, 15;
            ]
        }
    )
}   

#[test]
#[should_panic]
fn sparse_matrix_mul_panic() {

    let a = SparseMatrix {
        size: (5, 4),
        non_zero: 4,
        data: mat![
            0, 2, 2;
            0, 3, 4;
            1, 1, 1;
            3, 2, 3;
        ]
    };
    
    let b = SparseMatrix {
        size: (3, 5),
        non_zero: 5,
        data: mat![
            0, 0, 8;
            0, 3, 7;
            1, 2, 6;
            2, 1, 5;
        ]
    };

    SparseMatrix::mul(&a, &b).unwrap();

}

#[test]
fn sparse_matrix_to_vec() {

    let m = SparseMatrix {
        size: (5, 4),
        non_zero: 4,
        data: mat![
            0, 2, 2;
            0, 3, 4;
            1, 1, 1;
            3, 2, 3;
        ]
    };

    assert_eq!(
        m.to_vec(),
        mat![
            0, 0, 2, 4;
            0, 1, 0, 0;
            0, 0, 0, 0;
            0, 0, 3, 0;
            0, 0, 0, 0;
        ]
    )

}