fn merge_sorted_vectors(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            result.push(a[i]);
            i += 1;
        } else {
            result.push(b[j]);
            j += 1;
        }
    }

    while i < a.len() {
        result.push(a[i]);
        i += 1;
    }

    while j < b.len() {
        result.push(b[j]);
        j += 1;
    }

    result
}

fn main(){
    let a = [1, 3, 5, 7, 9];
    let b = [2, 4, 6, 8, 10];
    println!("{:?}", merge_sorted_vectors(&a, &b));
}