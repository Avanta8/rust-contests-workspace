//{"name":"C. Ordered Permutations","group":"Codeforces - Codeforces Round 992 (Div. 2)","url":"https://codeforces.com/contest/2040/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n3 2\n3 3\n4 11\n4 6\n6 39\n7 34\n","output":"1 3 2\n2 3 1\n-1\n2 4 3 1\n-1\n2 3 4 5 7 6 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"COrderedPermutations"}}}

#![allow(
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::comparison_chain,
    clippy::let_and_return,
    clippy::needless_range_loop
)]

#[allow(unused_imports)]
use algo_lib::dbg;
use algo_lib::{
    io::{input::Input, output::Output},
    misc::test_type::TaskType,
};

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let size = input.read_size();
    let mut k = input.read_long();

    let mut ans = vec![0; size];
    let mut front = 0;
    let mut back = size - 1;
    let mut prev = 1;

    if let Some(v) = 1i64.checked_shl(size as u32 - 1) {
        if k > v && v > 0 {
            out.print_line(-1);
            return;
        }
    }

    while k > 1 {
        // dbg!(k, ans);
        let mut n = 0;
        let mut start = 0;
        let sz = back - front + 1;
        // assert!(1i64.wrapping_shl(sz as u32) >= k);
        for p in 1..=sz {
            // let cnt = if sz == p { 1 } else { 1 << (sz - p - 1) };
            let cnt = if sz == p {
                1
            } else {
                if let Some(v) = 1i64.checked_shl((sz - p - 1) as u32) {
                    if v <= 0 {
                        start = p + prev - 1;
                        break;
                    }
                    v
                } else {
                    start = p + prev - 1;
                    break;
                }
            };
            if n + cnt >= k {
                start = p + prev - 1;
                break;
            };
            n += cnt;
        }
        // if start == 0 {
        //     out.print_line(-1);
        //     return;
        // }
        // dbg!(start, n);
        k -= n;

        ans[front] = start;
        front += 1;
        while prev < start {
            ans[back] = prev;
            back -= 1;
            prev += 1;
        }
        prev = start + 1;
    }
    assert_eq!(k, 1);

    // dbg!(front, back);
    for i in front..=back {
        ans[i] = prev;
        prev += 1;
    }
    // dbg!(ans);

    // dbg!(1i64 << 63);
    // dbg!(1i64.checked_shl(64));
    out.print_line(ans);
}

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
