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

pub fn partition_at(v: &Vec<usize>, pivot: usize) -> (Vec<usize>, Vec<usize>) {
    let (mut left, right): (Vec<usize>, Vec<usize>) = v.iter().partition(|&x| x <= &pivot);
    let pivot_idx = left.iter().position(|&x| x == pivot).unwrap();
    left.remove(pivot_idx);
    (left, right)
}

#[inline]
pub fn quick_sort(v: &mut Vec<usize>) {
    let n = v.len();
    if n > 2 {
        let pivot = (v[0] + v[n / 2] + v[n - 1]) / 3;
        let (mut left, mut right) = partition_at(v, pivot);
        quick_sort(&mut left);
        quick_sort(&mut right);
        left.push(pivot);
        left.append(&mut right);
        *v = left;
    } else if n == 2 {
        if v[1] < v[0] {
            let temp = v[1];
            v[1] = v[0];
            v[0] = temp;
        }
    }
}
