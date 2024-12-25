//{"name":"E. Matrix Transformation","group":"Codeforces - Educational Codeforces Round 173 (Rated for Div. 2)","url":"https://codeforces.com/contest/2043/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 1\n12\n13\n2 2\n10 10\n42 42\n21 21\n21 21\n2 2\n74 10\n42 106\n21 85\n85 21\n2 4\n1 2 3 4\n5 6 7 8\n3 2 3 4\n1 0 1 0\n","output":"Yes\nYes\nNo\nYes\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMatrixTransformation"}}}

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
        TaskType::Classic => {
            input.is_empty()
        }
        TaskType::Interactive => true,
    }
}


//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
