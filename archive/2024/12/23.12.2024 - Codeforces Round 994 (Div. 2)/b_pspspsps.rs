//{"name":"B. pspspsps","group":"Codeforces - Codeforces Round 994 (Div. 2)","url":"https://codeforces.com/contest/2049/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"9\n4\ns.sp\n6\npss..s\n5\nppppp\n2\nsp\n4\n.sp.\n8\npsss....\n1\n.\n8\npspspsps\n20\n....................\n","output":"YES\nNO\nYES\nYES\nNO\nNO\nYES\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPspspsps"}}}

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
    let arr = input.read_string().chars().collect::<Vec<_>>();

    let mut midp = false;
    let mut mids = false;
    for (i, &char) in arr.iter().enumerate() {
        if char == 'p' {
            if i + 1 < size {
                midp = true;
            }
        } else if char == 's' {
            if i > 0 {
                mids = true;
            }
        }
    }

    out.print_line(if midp && mids { "NO" } else { "YES" })
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
