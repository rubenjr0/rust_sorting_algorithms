#[inline]
pub fn selection_sort(v: &mut Vec<usize>) {
    let n = v.len();
    for i in 0..(n - 1) {
        let mut min_index = i;
        for j in i..n {
            if v[j] < v[min_index] {
                min_index = j;
            }
        }
        let temp = v[i];
        v[i] = v[min_index];
        v[min_index] = temp;
    }
}

#[inline]
pub fn insertion_sort(v: &mut Vec<usize>) {
    let n = v.len();
    for i in 0..(n) {
        for j in 0..i {
            if v[i] < v[j] {
                let temp = v[j];
                v[j] = v[i];
                v[i] = temp;
            }
        }
    }
}

#[inline]
pub fn bubble_sort(v: &mut Vec<usize>) {
    let n = v.len();
    for _ in 0..n {
        for j in 0..(n - 1) {
            if v[j + 1] < v[j] {
                let temp = v[j];
                v[j] = v[j + 1];
                v[j + 1] = temp;
            }
        }
    }
}
