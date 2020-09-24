extern crate rand;
extern crate time;

use rand::Rng;
use std::time::Instant;

fn create_matrix_and_fill(size: usize) -> Vec<Vec<f64>> {
    let mut rng = rand::thread_rng();
    vec![vec![rng.gen(); size as usize]; size as usize]
}

fn create_vector_and_fill(size: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    vec![rng.gen(); size as usize]
}
fn product_matrix1(a: Vec<Vec<f64>>, x: Vec<f64>, mut y: Vec<f64>, max: usize) {
    for i in 0..max {
        for j in 0..max {
            y[i] += a[i][j] * x[j];
        }
    }
}
fn product_matrix2(a: Vec<Vec<f64>>, x: Vec<f64>, mut y: Vec<f64>, max: usize) {
    for j in 0..max {
        for i in 0..max {
            y[i] += a[i][j] * x[j];
        }
    }
}
fn product_3_loop(a: Vec<Vec<f64>>, b: Vec<Vec<f64>>, mut r: Vec<Vec<f64>>, max: usize) {
    for i in 0..max {
        for j in 0..max {
            for k in 0..max {
                r[i][j] += a[i][k] * b[k][j];
            }
        }
    }
}
fn min(a: usize, b: usize) -> usize {
    if a > b {
        b
    } else {
        a
    }
}
fn product_6_loop(
    a: Vec<Vec<f64>>,
    b: Vec<Vec<f64>>,
    mut r: Vec<Vec<f64>>,
    max: usize,
    block: usize,
) {
    if block >= max {}
    let mut i = 0;
    while i < max {
        let mut j = 0;
        while j < max {
            let mut k = 0;
            while k < max {
                let mut ii = i;
                while ii < min(i + block, max) {
                    let mut jj = j;
                    while jj < min(j + block, max) {
                        let mut kk = k;
                        while kk < min(k + block, max) {
                            r[ii][kk] += a[ii][jj] * b[jj][kk];
                            kk += 1;
                        }
                        jj += 1;
                    }
                    ii += 1;
                }
                k += block;
            }
            j += block;
        }
        i += block;
    }
}
fn main() {
    let max: usize = 100;
    let _block = 2;
    let a = create_matrix_and_fill(max);
    let b = create_matrix_and_fill(max);
    let r = create_matrix_and_fill(max);
    let _x = create_vector_and_fill(max);
    let _y = create_vector_and_fill(max);
    let now = Instant::now();
    {
        product_6_loop(a,b,r,max,_block);
        //product_matrix1(a,_x,_y,max);
        //product_matrix2(a,_x,_y,max);
        //product_3_loop
        //product_3_loop(a, b, r, max);
    }
    let elapsed = now.elapsed();
    println!(
        "Tiempo 6_loop tamanio {} bloque {} : {:?}",
        max, _block, elapsed
    );
}
