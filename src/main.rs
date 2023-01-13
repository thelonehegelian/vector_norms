use ndarray::{array, Array1, ArrayView1};
fn main() {
    fn l1_norm(arr: Array1<f64>) -> f64 {
        arr.fold(0.0, |acc, x| acc + x.abs())
        // let sum = arr.mapv(|x| x.abs()).sum();
    }
    fn l2_norm(arr: Array1<f64>) -> f64 {
        // arr.fold(0.0, |acc, x| acc + x.powi(2)).sqrt()
        arr.dot(&arr).sqrt()
    }
    fn normalize(mut arr: Array1<f64>) -> f64 {
        let norm = l2_norm(arr.clone());
        arr.mapv_inplace(|x| x / norm);
        norm
    }
    let x = array![1., 2., 3., 4., 5.];
    println!("||x||_2 = {}", l2_norm(x.clone()));
    println!("||x||_1 = {}", l1_norm(x.clone()));
    println!("Normalizing x yields {:?}", normalize(x));
}
