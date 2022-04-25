use rand::prelude::*;

/// Get a random vector of size `nbr_elem`
pub fn get_randvec(nbr_elements: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut v: Vec<f64> = Vec::new();
    for _ in 0..nbr_elements {
        v.push(rng.gen::<f64>());
    }
    v
}

/// Sum all the elements in v using an iterator
pub fn iterator_access(v: &[f64]) -> f64 {
    v.iter().sum()
}

/// Sum all the elements in v using indexing
pub fn checked_index(v: &[f64]) -> f64 {
    let mut res = 0_f64;
    for i in 0..v.len() {
        res += v[i]
    }
    res
}

/// Sum all the elements in v using unchecked indexing
pub fn unchecked_index(v: &[f64]) -> f64 {
    let mut res = 0_f64;
    for i in 0..v.len() {
        unsafe {
            res += v.get_unchecked(i);
        }
    }
    res
}


/// Sum all elements using backwards indexing
pub fn backwards_index(v: &[f64]) -> f64 {
    let mut res = 0_f64;
    for i in (0..v.len()).rev() {
        res += v[i]
    }
    res
}

/// Sum all elements using skip-10 indexing
/// The point of this method is to slow down the indexing to achieve more cache-misses
pub fn skip10_index(v: &[f64]) -> f64 {
    let mut res = 0_f64;
    for i in 0..10 {
        for j in (i..v.len()).step_by(10) {
            res += v[j]
        }   
    }
    res
}


#[cfg(test)]
mod test_super {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_all_equal() {
        let v = get_randvec(10_000_000);
        let res1 = checked_index(&v);
        let res2 = iterator_access(&v);
        let res3 = unchecked_index(&v);
        let res4 = backwards_index(&v);
        let res5 = skip10_index(&v);
        // print all the results
        println!("checked_index {:?}", res1);
        println!("iterator_access {:?}", res2);
        println!("unchecked_index {:?}", res3);
        println!("backwards_index {:?}", res4);
        println!("skip10_index {:?}", res5);
        // assert that all the results are equal
        assert_abs_diff_eq!(res1, res2, epsilon = 1e-5);
        assert_abs_diff_eq!(res1, res3, epsilon = 1e-5);
        assert_abs_diff_eq!(res1, res4, epsilon = 1e-5);
        assert_abs_diff_eq!(res1, res5, epsilon = 1e-5);

        
    }
}