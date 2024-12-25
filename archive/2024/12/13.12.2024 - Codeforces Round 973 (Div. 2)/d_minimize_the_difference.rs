//{"name":"D. Minimize the Difference","group":"Codeforces - Codeforces Round 973 (Div. 2)","url":"https://codeforces.com/problemset/problem/2013/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n1\n1\n3\n1 2 3\n4\n4 1 2 3\n4\n4 2 3 1\n5\n5 14 4 10 2\n","output":"0\n2\n1\n1\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMinimizeTheDifference"}}}

#![allow(
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::comparison_chain,
    clippy::let_and_return
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
    let vec = input.read_long_vec(size);

    let max_elem = *vec.iter().max().unwrap();

    // Can we make every element at least as big as this?
    let min_target = |target: i64| -> bool {
        let mut total = 0;

        for (&x, count) in vec.iter().zip(1..) {
            total += x;
            if total / count < target {
                return false;
            }
        }
        true
    };

    // Can we make every element at most as big as this?
    let max_target = |target: i64| -> bool {
        let mut total = 0;

        for (&x, count) in vec.iter().rev().zip(1..) {
            total += x;
            if (total + count - 1) / count > target {
                return false;
            }
        }
        true
    };

    // maximise minimum
    let min = {
        let mut low = 0; // yes
        let mut high = max_elem + 1; // no

        // last yes
        while low + 1 < high {
            let mid = (low + high) / 2;

            if min_target(mid) {
                low = mid;
            } else {
                high = mid;
            }
        }

        low
    };

    let max = {
        let mut low = -1; // no
        let mut high = max_elem; // yes

        // first yes
        while low + 1 < high {
            let mid = (low + high) / 2;

            if max_target(mid) {
                high = mid;
            } else {
                low = mid;
            }
        }
        high
    };

    out.print_line(max - min);
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
