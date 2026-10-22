fn countResponseTimeRegressions(responseTimes: &[i32]) -> i32 {
    //Sum could go greater than i32
    // C++ equivalent for revision:
    // int countResponseTimeRegressions(vector<int>& responseTimes) {
    //     int n = responseTimes.size();
    //     if (n <= 1) return 0;
    //     long long sum = responseTimes[0];
    //     int cnt = 0;
    //     for (int i = 1; i < n; i++) {
    //         if (responseTimes[i] > sum / i) cnt++;
    //         sum += responseTimes[i];
    //     }
    //     return cnt;
    // }
    let n = responseTimes.len();
    if n <= 1 {
        return 0;
    }
    let mut sum = responseTimes[0] as i64;
    let mut cnt = 0;
    for i in 1..n {
        let avg = sum / i as i64;
        if responseTimes[i] as i64 > avg {
            cnt += 1;
        }
        sum += responseTimes[i] as i64;
    }
    cnt
}

fn mergeHighDefinitionIntervals(intervals: &[Vec<i32>]) -> Vec<Vec<i32>> {
    // C++ equivalent for revision:
    // vector<vector<int>> mergeHighDefinitionIntervals(vector<vector<int>>& intervals) {
    //     if (intervals.empty()) return {};
    //     sort(intervals.begin(), intervals.end());
    //     vector<vector<int>> res;
    //     res.push_back(intervals[0]);
    //     for (int i = 1; i < intervals.size(); i++) {
    //         auto& prev = res.back();
    //         if (prev[1] >= intervals[i][0]) {
    //             prev[1] = max(prev[1], intervals[i][1]);
    //         } else {
    //             res.push_back(intervals[i]);
    //         }
    //     }
    //     return res;
    // }
    let mut sortTedInt = intervals.to_vec();
    let mut res = Vec::new();
    sortTedInt.sort_by(|a, b| a[0].cmp(&b[0]));

    for interval in sortTedInt {
        let prev = match res.last_mut() {
            Some(a) => a,
            None => {
                res.push(interval);
                continue;
            }
        };

        if prev[1] >= interval[0] {
            *prev = vec![prev[0], std::cmp::max(prev[1], interval[1])];
        } else {
            res.push(interval);
        }
    }
    res
}

fn countSubarraysWithSumAndMaxAtMost(nums: &[i32], k: i64, M: i64) -> i64 {
    // C++ equivalent for revision:
    // long long countSubarraysWithSumAndMaxAtMost(vector<int>& nums, int k, int M) {
    //     long long count = 0, prefix_sum = 0;
    //     unordered_map<long long, int> freq;
    //
    //     for (int num : nums) {
    //         if (num > M) {
    //             freq.clear();
    //             prefix_sum = 0;
    //             continue;
    //         }
    //         prefix_sum += num;
    //
    //         if (prefix_sum == k) count++;
    //         if (freq.count(prefix_sum - k))
    //             count += freq[prefix_sum - k];
    //
    //         freq[prefix_sum]++;
    //     }
    //     return count;
    // }
    let mut count = 0;
    let mut prefix_sum = 0i64;
    use std::collections::HashMap;
    let mut freq: HashMap<i64, i64> = HashMap::new();

    for &num in nums {
        if num as i64 > M {
            freq.clear();
            prefix_sum = 0;
            continue;
        }
        prefix_sum += num as i64;

        if prefix_sum == k {
            count += 1;
        }
        if let Some(&val) = freq.get(&(prefix_sum - k)) {
            count += val;
        }
        *freq.entry(prefix_sum).or_insert(0) += 1;
    }
    count
}

fn isAnagram(s: &str, t: &str) -> i32 {
    // C++ equivalent for revision:
    // int isAnagram(string s, string t) {
    //     if (s.size() != t.size()) return 0;
    //     int hash1[26] = {0};
    //     for (char c : s) hash1[c - 'a']++;
    //     for (char c : t) hash1[c - 'a']--;
    //     for (int i = 0; i < 26; i++) {
    //         if (hash1[i] != 0) return 0;
    //     }
    //     return 1;
    // }
    let (m, n) = (s.len(), t.len());
    if m != n {
        return 0;
    }
    let mut hash1 = vec![0; 26];
    for i in s.bytes() {
        hash1[(i - b'a') as usize] += 1;
    }
    for i in t.bytes() {
        hash1[(i - b'a') as usize] -= 1;
    }
    let mut flag = 1;
    for i in hash1 {
        if i != 0 {
            flag = 0;
            break;
        }
    }

    flag
}

fn findSmallestMissingPositive(orderNumbers: &mut [i32]) -> i32 {
    // Write your code here
    let n = orderNumbers.len();
    for i in 0..n {
        while orderNumbers[i] >= 1
            && orderNumbers[i] <= n as i32
            && orderNumbers[i] != orderNumbers[(orderNumbers[i] - 1) as usize]
        {
            let tmp = orderNumbers[i];
            orderNumbers[i] = orderNumbers[(orderNumbers[i] - 1) as usize];
            orderNumbers[(orderNumbers[i] - 1) as usize] = tmp;
        }
    }
    for i in 0..n {
        if orderNumbers[i] != i as i32 + 1 {
            return (i + 1) as i32;
        }
    }
    (n + 1) as i32
}

fn findLongestArithmeticProgression(arr: &[i32], k: i32) -> i32 {
    // C++ equivalent for revision:
    // int findLongestArithmeticProgression(vector<int>& arr, int k) {
    //     unordered_set<long long> stt(arr.begin(), arr.end());
    //     int best = 0;
    //     for (long long x : stt) {
    //         if (stt.count(x - k)) continue;   // not a chain start
    //         int len = 0;
    //         long long cur = x;
    //         while (stt.count(cur)) {          // walk the chain upward
    //             len++;
    //             cur += k;
    //         }
    //         best = max(best, len);
    //     }
    //     return best;
    // }
    use std::collections::HashSet;
    let stt: HashSet<i64> = arr.iter().map(|&x| x as i64).collect();
    let k = k as i64;
    let mut best = 0i32;
    for &x in &stt {
        if stt.contains(&(x - k)) {
            continue; // not the start of a chain
        }
        let mut len = 0i32;
        let mut cur = x;
        while stt.contains(&cur) {
            len += 1;
            cur += k;
        }
        best = best.max(len);
    }
    best
}

fn findPeakIndex(counts: &[i32]) -> i32 {
    // Write your code here
    let n = counts.len();
    let (mut l, mut r) = (0, n - 1);
    while r > l {
        let mid = l + (r - l) / 2;
        if counts[mid] > counts[mid + 1] {
            r = mid;
        } else {
            l = mid + 1;
        }
    }
    counts[l]
}
