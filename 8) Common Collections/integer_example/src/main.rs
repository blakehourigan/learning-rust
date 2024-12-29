use std::cmp::Ordering;
use std::f32;

fn main() {
    let integers = [5, 3, 2, 10, 20, 24, 90, 100, 127, 15];

    let v = create_sorted_vector(&integers);
    println!("vector created and sorted: {v:?}");

    let median = compute_median_value(v);

    println!("the median vector value is {0}", median);
}

fn create_sorted_vector(a: &[i32]) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    println!("original integers: {0:?}", a);
    for i in 0..a.len() {
        v.push(a[i]);

        for j in 0..i {
            match v[i].cmp(&v[j]) {
                Ordering::Less => {
                    let tmp = v[i];
                    v[i] = v[j];
                    v[j] = tmp;
                    continue;
                }
                _ => (),
            }
        }
    }
    v
}

fn compute_median_value(vector: Vec<i32>) -> i32 {
    let median;
    let v_len: f32 = vector.len() as f32;

    match (vector.len() % 2).cmp(&0) {
        Ordering::Equal => {
            let loc1: usize = (v_len / 2.0) as usize;
            let value1 = vector[loc1];

            let loc2 = loc1 + 1;
            let value2 = vector[loc2];

            median = (value1 + value2) / 2;
        }
        _ => {
            let loc = f32::ceil(v_len / 2.0) as usize;
            median = vector[loc];
        }
    }
    median
}
