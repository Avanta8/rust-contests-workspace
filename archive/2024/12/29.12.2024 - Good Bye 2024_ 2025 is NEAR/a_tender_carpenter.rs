//{"name":"A. Tender Carpenter","group":"Codeforces - Good Bye 2024: 2025 is NEAR","url":"https://codeforces.com/contest/2053/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n4\n2 3 5 7\n4\n115 9 2 28\n5\n8 4 1 6 2\n6\n1 5 4 1 4 7\n2\n100000 100000\n","output":"YES\nNO\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ATenderCarpenter"}}}

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
    let mut arr = input.read_long_vec(size);

    for (mut a, mut b) in arr.iter().copied().zip(arr.iter().copied().skip(1)) {
        if a > b {
            std::mem::swap(&mut a, &mut b);
        }
        if a * 2 > b {
            out.print_line("YES");
            return;
        }
    }
    out.print_line("NO");
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
