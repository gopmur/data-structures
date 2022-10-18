// Ali Jafari 4001262057

#[cfg(test)]
mod tests;

use std::cmp::Ordering;

// macro for making matrices easier not really important 
#[macro_export]
macro_rules! mat {
    ($( $( $e:expr ),+);+ $(;)?) => {{
        
        let mut mat = Vec::new();
        let mut row = Vec::new();

        $(
            $(
                row.push($e);
            )+
            mat.push(row.clone());
            row.clear();
        )+

        mat

    }};
}

#[derive(Debug, PartialEq)]
pub struct SparseMatrix {
    pub size: (usize, usize),
    pub non_zero: usize,
    data: Vec<Vec<i32>>
}

impl SparseMatrix {

    pub fn from(matrix: &Vec<Vec<i32>>) -> Self {

        let size = (matrix.len(), matrix[0].len());
        let mut non_zero = 0;

        for row in matrix {
            for e in row {
                if *e != 0 {
                    non_zero += 1;
                }
            }
        }

        let mut sparse_matrix = Self {
            size,
            non_zero,
            data: Vec::with_capacity(non_zero)
        };

        for i in 0..sparse_matrix.size.0 {
            for j in 0..sparse_matrix.size.1 {
                if matrix[i][j] != 0 {
                    sparse_matrix.data.push(vec![i as i32, j as i32, matrix[i][j]])
                }
            }
        }

        sparse_matrix

    }

    pub fn transpose(&self) -> Self {

        let mut matrix_t = Self {
            size: (self.size.1, self.size.0),
            non_zero: self.non_zero,
            data: self.data.clone(),
        };
        
        let mut temp;

        for row in &mut matrix_t.data {
            temp = row[0];
            row[0] = row[1];
            row[1] = temp;
        }

        matrix_t.data.sort_by(
            |a, b| {

                if      a[0] > b[0] { Ordering::Greater }                
                else if a[0] < b[0] { Ordering::Less    }
                else if a[1] > b[1] { Ordering::Greater }
                else if a[1] < b[1] { Ordering::Less    }
                else                { Ordering::Equal   }

            }
        );

        matrix_t

    }

    pub fn mul(a: &Self, b: &Self) -> Result<Self, String> {
        
        if a.size.1 != b.size.0 {
            return Err(format!(
                "Can't multiply matrix with size ({}, {}) with a matrix with size ({}, {})",
                a.size.0, a.size.1, b.size.0, b.size.1
            ));
        }

        let mut c = Self {
            size: (a.size.0, b.size.1),
            non_zero: 0,
            data: Vec::new()
        };

        for a_i in 0..a.non_zero {
            for b_i in 0..b.non_zero {
                
                if a.data[a_i][1] == b.data[b_i][0] {
                    for row in &mut c.data {
                        if (row[0], row[1]) == (a.data[a_i][0], b.data[b_i][1]) {
                            row[2] += a.data[a_i][2] * b.data[b_i][2];
                            break;
                        }
                    }
                    c.data.push(vec![a.data[a_i][0], b.data[b_i][1], a.data[a_i][2] * b.data[b_i][2]]);
                    c.non_zero += 1;
                }

            }
        }

        // probably not efficient 
        c.data.sort_by(
            |a, b| {

                if      a[0] > b[0] { Ordering::Greater }                
                else if a[0] < b[0] { Ordering::Less    }
                else if a[1] > b[1] { Ordering::Greater }
                else if a[1] < b[1] { Ordering::Less    }
                else                { Ordering::Equal   }
                
            }
        );

        Ok(c)

    }

    pub fn to_vec(&self) -> Vec<Vec<i32>> {

        let mut vec_matrix: Vec<Vec<i32>> = Vec::with_capacity(self.size.0); 
        let mut k = 0;

        for i in 0..self.size.0 {{
            let mut row: Vec<i32> = Vec::with_capacity(self.size.1);
            for j in 0..self.size.1 {

                if k < self.non_zero && i as i32 == self.data[k][0] && j as i32 == self.data[k][1] {
                    row.push(self.data[k][2]);
                    k += 1;
                }

                else {
                    row.push(0);
                }

            }
            vec_matrix.push(row);
        }}

        vec_matrix

    }

}

// lower trangular matrix
#[derive(Debug, PartialEq)]
pub struct Ltm {
    pub size: usize,
    data: Vec<i32>
} 

impl Ltm {

    pub fn from(matrix: &Vec<Vec<i32>>) -> Result<Self, String> {

        if matrix.len() != matrix[0].len() {
            return Err("Not a square matrix".to_string());
        }

        if !Self::is_ltm(matrix) {
            return Err("Not a lower trangular matrix".to_string());
        }

        let mut alternative_matrix = Self {
            size: matrix.len(),
            data: Vec::with_capacity(matrix.len()*(matrix.len() + 1)/2),
        };

        for i in 0..alternative_matrix.size {
            for j in 0..=i {
                alternative_matrix.data.push(matrix[i][j]);
            }
        }

        Ok(alternative_matrix)

    }

    pub fn to_vec(&self) -> Vec<Vec<i32>> {
        
        let mut matrix = Vec::with_capacity(self.size);

        for i in 0..self.size {{
            let mut row = Vec::with_capacity(self.size);
            for j in 0..self.size {
                if j > i {
                    row.push(0);
                }
                else {
                    row.push(self.data[i*(i + 1)/2 + j]);
                }
            }
            matrix.push(row);
        }}

        matrix 

    }

    fn is_ltm(matrix: &Vec<Vec<i32>>) -> bool {

        if matrix.len() != matrix[0].len() {
            return false;
        }

        let mut is_ltm = true;
        for j in 0..matrix.len() {
            for i in 0..j {
                if matrix[i][j] != 0 {
                    is_ltm = false;
                    break;
                }
            }
        }

        is_ltm

    }

}

// higher trangular matrix
#[derive(Debug, PartialEq)] 
pub struct Htm {
    pub size: usize,
    data: Vec<i32>,
}

impl Htm {

    pub fn from(matrix: &Vec<Vec<i32>>) -> Result<Self, String> {

        if matrix.len() != matrix[0].len() {
            return Err("Not a square matrix".to_string());
        }

        if !Self::is_htm(matrix) {
            return Err("Not a higher trangular matrix".to_string());
        }

        let mut alternative_matrix = Self {
            size: matrix.len(),
            data: Vec::with_capacity(matrix.len()*(matrix.len() + 1)/2)
        };

        for i in 0..alternative_matrix.size {
            for j in i..alternative_matrix.size {
                alternative_matrix.data.push(matrix[i][j]);
            }
        }

        Ok(alternative_matrix)

    }

    pub fn to_vec(&self) -> Vec<Vec<i32>> {
        
        let mut matrix = Vec::with_capacity(self.size);

        for i in 0..self.size {{
            let mut row = Vec::with_capacity(self.size);
            for j in 0..self.size {
                if j < i {
                    row.push(0);
                }
                else {
                    row.push(self.data[i*(2*self.size - i - 1)/2 + j]);
                }
            }
            matrix.push(row);
        }}

        matrix 

    }

    fn is_htm(matrix: &Vec<Vec<i32>>) -> bool {

        if matrix.len() != matrix[0].len() {
            return false;
        }

        let mut is_htm = true;
        for i in 0..matrix.len() {
            for j in 0..i {
                if matrix[i][j] != 0 {
                    is_htm = false;
                    break;
                }
            }
        }

        is_htm

    }

}

