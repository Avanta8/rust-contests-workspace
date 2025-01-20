//{"name":"B. Gorilla and the Exam","group":"Codeforces - Hello 2025","url":"https://codeforces.com/contest/2057/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1 0\n48843\n3 1\n2 3 2\n5 3\n1 2 3 4 5\n7 0\n4 7 1 3 2 4 1\n11 4\n3 2 1 4 4 3 4 2 1 3 3\n5 5\n1 2 3 4 5\n","output":"1\n1\n2\n5\n2\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BGorillaAndTheExam"}}}

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
    slice::count_slice::count_slice,
};

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let size = input.read_size();
    let mut k = input.read_size();

    let vec = input.read_long_vec(size);

    let mut counts = count_slice(&vec).into_values().collect::<Vec<_>>();
    counts.sort();
    counts.reverse();

    while k > 0 && !counts.is_empty() {
        let c = counts.pop().unwrap();

        if k >= c {
            k -= c;
        } else {
            counts.push(c);
            break;
        }
    }

    out.print_line(counts.len().max(1));
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
