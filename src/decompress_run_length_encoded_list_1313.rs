/// https://leetcode.com/problems/decompress-run-length-encoded-list/
struct Solution;

impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.iter().step_by(2).fold(0, |acc, el| acc + el);
        let mut res = Vec::<i32>::with_capacity(len as _);

        for one in nums.chunks_exact(2) {
            let freq = one[0];
            let val = one[1];
            for _ in 0..freq {
                res.push(val);
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn a01() {
        let actual = Solution::decompress_rl_elist(vec![1, 2, 3, 4]);
        let expected = vec![2, 4, 4, 4];
        assert_eq!(actual, expected);
    }

    #[test]
    fn a02() {
        let actual = Solution::decompress_rl_elist(vec![1, 1, 2, 3]);
        let expected = vec![1, 3, 3];
        assert_eq!(actual, expected);
    }
}
