extern crate alloc;
#[macro_use]
extern crate porus;
prelude!(leetcode);

use alloc::collections::BTreeMap;
use core::mem::size_of;
use core::ptr::copy_nonoverlapping;
use core::slice;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = BTreeMap::new();
        for (i, x) in nums.iter().enumerate() {
            let y = target - x;
            if let Some(&j) = map.get(&y) {
                return vec![j as i32, i as i32];
            }
            map.insert(x, i);
        }
        unreachable!();
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn twoSum(
    nums: *const i32,
    numsSize: i32,
    target: i32,
    returnSize: *mut i32,
) -> *const i32 {
    let v = Solution::two_sum(
        unsafe { slice::from_raw_parts(nums, numsSize as usize) }.to_vec(),
        target,
    );
    unsafe {
        *returnSize = 2;
    }
    let p = unsafe { porus::libc::malloc(size_of::<i32>() * 2) as *mut i32 };
    unsafe { copy_nonoverlapping(v.as_ptr(), p, 2) };
    return p;
}
