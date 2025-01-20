//{"name":"C. Competitive Fishing","group":"Codeforces - Educational Codeforces Round 172 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/2042/C","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n4 1\n1001\n4 1\n1010\n4 1\n0110\n4 2\n0110\n6 3\n001110\n10 20\n1111111111\n5 11\n11111\n","output":"2\n-1\n2\n-1\n3\n4\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCompetitiveFishing"}}}

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
    let req = input.read_long();

    let fish = (0..size)
        .map(|_| input.read_byte() == b'1')
        .collect::<Vec<_>>();

    let mut options = vec![];
    let mut current = 0;
    for &a in fish.iter().rev() {
        if current > 0 {
            options.push(current);
        }
        if a {
            current += 1;
        } else {
            current -= 1;
        }
    }

    options.sort();
    options.reverse();

    let mut total = 0;
    for (i, elem) in options.into_iter().enumerate() {
        total += elem;
        if total >= req {
            out.print_line(i + 2);
            return;
        }
    }
    out.print_line(-1);
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
