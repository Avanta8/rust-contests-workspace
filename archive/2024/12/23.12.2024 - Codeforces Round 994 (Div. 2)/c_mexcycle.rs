//{"name":"C. MEX Cycle","group":"Codeforces - Codeforces Round 994 (Div. 2)","url":"https://codeforces.com/contest/2049/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n5 1 3\n4 2 4\n6 3 5\n7 3 6\n3 2 3\n5 1 5\n6 2 5\n","output":"0 2 1 0 1\n1 2 1 0\n1 2 0 1 2 0\n0 1 2 0 1 0 1\n2 0 1\n1 0 2 1 0\n0 1 2 0 2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMEXCycle"}}}

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
    let a = input.read_size() - 1;
    let b = input.read_size() - 1;

    let mut ans = vec![1; size];
    for i in (0..size).step_by(2) {
        ans[(a + i) % size] = 0;
    }
    if size % 2 == 1 || a.abs_diff(b) % 2 == 0 {
        ans[a] = 2;
    }

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
