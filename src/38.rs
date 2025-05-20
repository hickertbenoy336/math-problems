// Solution to the problem of finding the longest common subsequence (LCS)
fn lcs(a: &str, b: &str) -> usize {
    let m = a.len();
    let n = b.len();

    if m == 0 || n == 0 {
        return 0;
    }

    let mut dp = vec![0; n + 1];

    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[j] = dp[j - 1] + 1;
            } else {
                dp[j] = 0;
            }
        }
    }

    dp[n]
}
