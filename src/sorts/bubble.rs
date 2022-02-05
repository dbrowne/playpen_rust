// Dwight Browne 2022
// Simple bubble sorts in Rust.
// references:
//  https://stackoverflow.com/questions/28294735/how-to-swap-the-elements-of-an-array-slice-or-vec
//  https://sortvisualizer.com/bubblesort/


// bubble sorts that mutates its input
pub fn mbubble<T: Ord>(inp: &mut [T]) {
    let arr_len = inp.len();
    for i in 0..arr_len {
        for j in 0..arr_len - 1 - i {
            if inp[j] > inp[j + 1] {
                inp.swap(j, j + 1);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::mbubble;

    #[test]
    fn randvals() {
        let mut inp_vals = vec![23, 207, 14, -3, 0, 3, 4, 9, 8, 223, 91, 15];
        let k_result = vec![-3, 0, 3, 4, 8, 9, 14, 15, 23, 91, 207, 223];
        mbubble(&mut inp_vals);
        for idx in 0..k_result.len() - 1 {
            assert_eq!(k_result[idx], inp_vals[idx]);
        }
    }

    #[test]
    fn sortedvals() {
        let mut inp_vals = vec![-2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8];
        let k_result = vec![-2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8];
        mbubble(&mut inp_vals);
        for idx in 0..k_result.len() - 1 {
            assert_eq!(k_result[idx], inp_vals[idx]);
        }
    }

    #[test]
    fn revsortedvals() {
        let mut inp_vals = vec![12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 0, -1];
        let k_result = vec![-1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        mbubble(&mut inp_vals);
        for idx in 0..k_result.len() -1 {

        }
    }
}