// Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
// The overall run time complexity should be O(log (m+n)).
// Constraints:
//     nums1.length == m
//     nums2.length == n
//     0 <= m <= 1000
//     0 <= n <= 1000
//     1 <= m + n <= 2000
//     -106 <= nums1[i], nums2[i] <= 106

fn merge_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut merged = Vec::new();
    merged.append(&mut nums1.clone());
    merged.append(&mut nums2.clone());
    merged.sort();

    merged
}

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let merged = merge_arrays(nums1, nums2);
    match merged.len() % 2 == 0 {
        true => {
            let i_first = merged.len() / 2;
            let i_second = i_first - 1;

            (*merged.get(i_first).unwrap() as f64 + *merged.get(i_second).unwrap() as f64) / 2.0
        }
        _ => merged.get(merged.len() / 2).unwrap().clone() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn ex_1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];

        assert_eq!(merge_arrays(nums1.clone(), nums2.clone()), vec![1, 2, 3]);
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.00000);
    }

    #[test]
    pub fn ex_2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];

        assert_eq!(merge_arrays(nums1.clone(), nums2.clone()), vec![1, 2, 3, 4]);
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.50000);
    }
}
