//{"name":"D. Refined Product Optimality","group":"Codeforces - Good Bye 2024: 2025 is NEAR","url":"https://codeforces.com/contest/2053/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n3 4\n1 1 2\n3 2 1\n1 3\n2 3\n1 1\n2 1\n6 8\n1 4 2 7 3 5\n7 6 5 6 3 3\n2 5\n1 6\n1 5\n1 5\n1 5\n2 3\n2 3\n1 6\n13 8\n7 7 6 6 5 5 5 2 2 3 4 5 1\n1 4 1 9 6 6 9 1 5 1 3 8 4\n2 2\n2 11\n2 4\n2 4\n1 7\n1 1\n2 12\n1 5\n5 3\n10000000 20000000 30000000 40000000 50000000\n10000000 20000000 30000000 40000000 50000000\n1 1\n2 2\n2 1\n","output":"2 3 3 6 6\n840 840 1008 1344 1680 2016 2016 2016 2352\n2116800 2646000 3528000 3528000 3528000 4233600 4838400 4838400 4838400\n205272023 205272023 205272023 264129429\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRefinedProductOptimality"}}}

#![allow(
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::comparison_chain,
    clippy::let_and_return,
    clippy::needless_range_loop
)]

use std::collections::BTreeMap;

#[allow(unused_imports)]
use algo_lib::dbg;
use algo_lib::{
    io::{input::Input, output::Output},
    misc::test_type::TaskType,
    numbers::gcd::extended_gcd,
};

use algo_lib::misc::test_type::TestType;

const MOD: i64 = 998244353;

type PreCalc = ();

fn mod_inv(n: i64) -> i64 {
    let (d, x, y) = extended_gcd(n, MOD);
    assert_eq!(d, 1);
    (x as i64 % MOD + MOD) % MOD
}

fn mod_div(a: i64, b: i64) -> i64 {
    a * mod_inv(b) % MOD
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let size = input.read_size();
    let op_size = input.read_size();

    let mut a_arr = input.read_long_vec(size);
    let mut b_arr = input.read_long_vec(size);

    let mut a_map = vec![0; size];
    let mut b_map = vec![0; size];

    for (i, &a) in a_arr.iter().enumerate() {
        a_map[i] = a;
    }
    for (i, &b) in b_arr.iter().enumerate() {
        b_map[i] = b;
    }

    a_arr.sort();
    b_arr.sort();

    let mut a_counts = BTreeMap::new();
    for (i, &a) in a_arr.iter().enumerate() {
        a_counts.entry(a).or_insert((0, i)).0 += 1;
    }
    let mut b_counts = BTreeMap::new();
    for (i, &a) in b_arr.iter().enumerate() {
        b_counts.entry(a).or_insert((0, i)).0 += 1;
    }

    // dbg!(a_counts);
    // dbg!(b_counts);

    let operations = input.read_vec::<(i32, usize)>(op_size);

    let mut total = 1;
    for (a, b) in a_arr.iter().copied().zip(b_arr.iter().copied()) {
        total = (total * a.min(b)) % MOD;
    }

    let mut ans = vec![total];

    for (o, mut orig_idx) in operations {
        orig_idx -= 1;
        if o == 1 {
            let val = a_map[orig_idx];
            a_map[orig_idx] += 1;

            let &(mut count, arr_idx) = a_counts.get(&val).unwrap();

            count -= 1;
            if count > 0 {
                a_counts.insert(val, (count, arr_idx));
            } else {
                a_counts.remove(&val);
            }
            if let Some(&(next_count, next_arr_idx)) = a_counts.get(&(val + 1)) {
                assert_eq!(arr_idx + count, next_arr_idx - 1);
                a_counts.insert(val + 1, (next_count + 1, next_arr_idx - 1));
            } else {
                a_counts.insert(val + 1, (1, arr_idx + count));
            }

            // dbg!(total, arr_idx, count);
            total = mod_div(total, a_arr[arr_idx].min(b_arr[arr_idx]));
            // dbg!(total);
            if count > 0 {
                total = mod_div(total, a_arr[arr_idx + count].min(b_arr[arr_idx + count]));
            }

            a_arr[arr_idx] = a_arr[arr_idx + count];
            a_arr[arr_idx + count] = val + 1;

            total = (total * a_arr[arr_idx].min(b_arr[arr_idx])) % MOD;
            if count > 0 {
                total = (total * a_arr[arr_idx + count].min(b_arr[arr_idx + count])) % MOD;
            }
        } else {
            let val = b_map[orig_idx];
            b_map[orig_idx] += 1;

            let &(mut count, arr_idx) = b_counts.get(&val).unwrap();

            count -= 1;
            if count > 0 {
                b_counts.insert(val, (count, arr_idx));
            } else {
                b_counts.remove(&val);
            }
            if let Some(&(next_count, next_arr_idx)) = b_counts.get(&(val + 1)) {
                assert_eq!(arr_idx + count, next_arr_idx - 1);
                b_counts.insert(val + 1, (next_count + 1, next_arr_idx - 1));
            } else {
                b_counts.insert(val + 1, (1, arr_idx + count));
            }

            total = mod_div(total, a_arr[arr_idx].min(b_arr[arr_idx]));
            if count > 0 {
                total = mod_div(total, a_arr[arr_idx + count].min(b_arr[arr_idx + count]));
            }

            b_arr[arr_idx] = b_arr[arr_idx + count];
            b_arr[arr_idx + count] = val + 1;

            total = (total * a_arr[arr_idx].min(b_arr[arr_idx])) % MOD;
            if count > 0 {
                total = (total * a_arr[arr_idx + count].min(b_arr[arr_idx + count])) % MOD;
            }
        }

        // dbg!(a_arr);
        // dbg!(b_arr);
        ans.push(total);
    }

    out.print_line(ans);
}

// fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
//     let size = input.read_size();
//     let op_size = input.read_size();
//
//     let a_arr = input.read_long_vec(size);
//     let b_arr = input.read_long_vec(size);
//
//     let operations = input.read_vec::<(i32, usize)>(op_size);
//
//     let mut a_en = a_arr.iter().copied().zip(0usize..).collect::<Vec<_>>();
//     let mut b_en = b_arr.iter().copied().zip(0usize..).collect::<Vec<_>>();
//
//     a_en.sort();
//     b_en.sort();
//
//     let mut a_map = vec![0; size];
//     let mut b_map = vec![0; size];
//     let mut a_inv_map = vec![0; size];
//     let mut b_inv_map = vec![0; size];
//
//     let mut a_arr = vec![0; size];
//     let mut b_arr = vec![0; size];
//
//     for (i, (a, idx)) in a_en.into_iter().enumerate() {
//         a_map[idx] = i;
//         a_inv_map[i] = idx;
//         a_arr[i] = a;
//     }
//     for (i, (b, idx)) in b_en.into_iter().enumerate() {
//         b_map[idx] = i;
//         b_inv_map[i] = idx;
//         b_arr[i] = b;
//     }
//
//     let mut ans = vec![];
//
//     let mut total = 1;
//     for (a, b) in a_arr.iter().copied().zip(b_arr.iter().copied()) {
//         total = (total * a.min(b)) % MOD;
//     }
//
//     ans.push(total);
//     // dbg!(a_arr);
//     // dbg!(b_arr);
//     // dbg!(a_map);
//     // dbg!(b_map);
//     // dbg!(a_inv_map);
//     // dbg!(b_inv_map);
//     // dbg!("");
//
//     for (o, mut orig_idx) in operations {
//         orig_idx -= 1;
//         if o == 1 {
//             let mut idx = a_map[orig_idx];
//             assert_eq!(a_inv_map[idx], orig_idx);
//
//             total = mod_div(total, a_arr[idx].min(b_arr[idx]));
//             a_arr[idx] += 1;
//             total = (total * a_arr[idx].min(b_arr[idx])) % MOD;
//
//             while idx + 1 < size && a_arr[idx] > a_arr[idx + 1] {
//                 total = mod_div(total, a_arr[idx].min(b_arr[idx]));
//                 total = mod_div(total, a_arr[idx + 1].min(b_arr[idx + 1]));
//
//                 a_arr.swap(idx, idx + 1);
//
//                 total = (total * a_arr[idx].min(b_arr[idx])) % MOD;
//                 total = (total * a_arr[idx + 1].min(b_arr[idx + 1])) % MOD;
//
//                 a_map[orig_idx] = idx + 1;
//                 a_map[a_inv_map[idx + 1]] = idx;
//
//                 // a_inv_map[idx] = a_inv_map[idx + 1];
//                 // a_inv_map[idx + 1] = orig_idx;
//                 a_inv_map.swap(idx, idx + 1);
//                 idx += 1;
//             }
//         } else {
//             let mut idx = b_map[orig_idx];
//             assert_eq!(b_inv_map[idx], orig_idx);
//
//             total = mod_div(total, a_arr[idx].min(b_arr[idx]));
//             b_arr[idx] += 1;
//             total = (total * a_arr[idx].min(b_arr[idx])) % MOD;
//
//             while idx + 1 < size && b_arr[idx] > b_arr[idx + 1] {
//                 total = mod_div(total, a_arr[idx].min(b_arr[idx]));
//                 total = mod_div(total, a_arr[idx + 1].min(b_arr[idx + 1]));
//
//                 b_arr.swap(idx, idx + 1);
//
//                 total = (total * a_arr[idx].min(b_arr[idx])) % MOD;
//                 total = (total * a_arr[idx + 1].min(b_arr[idx + 1])) % MOD;
//
//                 b_map[orig_idx] = idx + 1;
//                 b_map[b_inv_map[idx + 1]] = idx;
//
//                 b_inv_map.swap(idx, idx + 1);
//                 idx += 1;
//             }
//         }
//         // dbg!(a_arr);
//         // dbg!(b_arr);
//         // dbg!(a_map);
//         // dbg!(b_map);
//         // dbg!("");
//         ans.push(total);
//     }
//
//     out.print_line(ans);
// }

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    #[allow(clippy::let_unit_value)]
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
