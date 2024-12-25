//{"name":"F. XORificator 3000","group":"Codeforces - Codeforces Round 984 (Div. 3)","url":"https://codeforces.com/contest/2036/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1 3 1 0\n2 28 3 7\n15 43 1 0\n57 2007 1 0\n1010 1993 2 2\n1 1000000000 30 1543\n","output":"2\n2\n13\n0\n4\n1000000519\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FXORificator3000"}}}

#![allow(
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::collapsible_match
)]

#[allow(unused_imports)]
use algo_lib::dbg;
use algo_lib::{
    io::{input::Input, output::Output},
    misc::test_type::TaskType,
};

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn calc(mut n: i64, power: u32, rem: i64) -> i64 {
    if n == rem && n % (1 << power) == rem {
        return n;
    }
    n -= rem;
    if n <= 0 {
        return 0;
    }
    let base = 1i64 << power;
    let count = n / base;

    let mut cycle_length = 2;
    let mut ans = 0;
    let mut add = 1;
    n >>= power;
    let mut first = true;
    while n > 0 {
        n >>= 1;
        let cycle_idx = count % cycle_length;
        let value = if first {
            first = false;
            let ones = (count + 1) / 2;
            ones % 2
        } else {
            if cycle_idx < cycle_length / 2 {
                0
            } else {
                (cycle_idx - cycle_length / 2 + 1) % 2
            }
        };
        ans += add * value;
        add <<= 1;
        cycle_length <<= 1;
    }
    ans <<= power;
    if count % 2 == 0 {
        assert_eq!(ans + rem, ans ^ rem);
        ans += rem;
    }
    ans
}

fn calc_neg(n: i64, power: u32, rem: i64) -> i64 {
    let total = calc(n, 0, 0);
    total ^ calc(n, power, rem)
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let (l, r, power, rem) = input.read::<(i64, i64, u32, i64)>();
    let ans = calc_neg(r, power, rem) ^ calc_neg(l - 1, power, rem);
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
