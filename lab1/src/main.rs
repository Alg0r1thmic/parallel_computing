use rand::Rng;

fn create_matrix_and_fill(size: usize) -> Vec<Vec<f64>> {
    let mut rng = rand::thread_rng();
    vec![vec![rng.gen();size as usize]; size as usize]
}

fn create_vector_and_fill(size:usize)-> Vec<f64>{
    let mut rng = rand::thread_rng();
    vec![rng.gen();size as usize]    
}
fn product_matrix(a:Vec<Vec<f64>>,x:Vec<f64>,mut y:Vec<f64>,max:usize){
    for i in 0..max {
        for j in 0..max {
            y[i]+=a[i][j]*x[j];
        }
    }
    for j in 0..max {
        for i in 0..max {
            y[i]+=a[i][j]*x[j];
        }
    }
}
fn product_3_loop(a:Vec<Vec<f64>>,b:Vec<Vec<f64>>,mut r:Vec<Vec<f64>>,max:usize){
    for i in 0..max {
        for j in 0..max{
            for k in 0..max {
                r[i][j]+=a[i][k]*b[k][j];
            }
        }
    }
    println!("finish");
}
fn min(a:usize,b:usize) -> usize{
    if a > b { b } else { a }
}
fn product_6_loop(a:Vec<Vec<f64>>,b:Vec<Vec<f64>>,mut r:Vec<Vec<f64>>,max:usize,block:usize){
   if block >=max {
   } 
   for i in 0..max {
       for j in 0..max {
           for k in 0..max {
               let mut ii=i;
               while ii< min(i+block, max){
                    let mut jj=j;
                    while jj< min(j+block,max){
                        let mut kk=k;
                        while kk<min(k+block,max){
                            r[ii][kk]+=a[ii][jj]*b[jj][kk];
                            kk+=1;
                        }
                        jj+=1;
                    }
                    ii+=1;
                }
           }
       }
   }
}
fn main() {
    let max:usize=20;
    let block=10;
    let a=create_matrix_and_fill(max);
    let b=create_matrix_and_fill(max);
    let r=create_matrix_and_fill(max);
    let x=create_vector_and_fill(max);
    let y=create_vector_and_fill(max);
    //product_matrix(a,x,y,max);
    //product_3_loop
    //product_3_loop(a, b,r, max);
    product_6_loop(a,b,r,max,block);
    //println!("min{}",min(2, 3));
    
}