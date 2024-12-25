//{"name":"D. Problem about GCD","group":"Codeforces - Educational Codeforces Round 173 (Rated for Div. 2)","url":"https://codeforces.com/contest/2043/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4 8 2\n4 8 3\n4 8 4\n5 7 6\n","output":"4 6\n-1 -1\n4 8\n6 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DProblemAboutGCD"}}}

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

pub fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        a %= b;
        std::mem::swap(&mut a, &mut b);
    }
    a.abs()
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let (mut l, mut r, g) = input.read::<(i128, i128, i128)>();

    l = (l + g - 1) / g;
    r = r / g;

    if r < l {
        out.print_line((-1, -1));
        return;
    }

    for ds in 0..50 {
        if r - l - ds < 0 {
            out.print_line((-1, -1));
            return;
        }
        for dl in 0..=ds {
            let dr = ds - dl;
            let nl = l + dl;
            let nr = r - dr;
            if gcd(nl, nr) == 1 {
                out.print_line((nl * g, nr * g));
                return;
            }
        }
    }
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
