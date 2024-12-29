//{"name":"C. Bewitching Stargazer","group":"Codeforces - Good Bye 2024: 2025 is NEAR","url":"https://codeforces.com/contest/2053/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n7 2\n11 3\n55 13\n5801 6\n8919 64\n8765432 1\n","output":"12\n18\n196\n1975581\n958900\n38416403456028\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CBewitchingStargazer"}}}

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
    misc::{
        recursive_function::{Callable2, RecursiveFunction2},
        test_type::TaskType,
    },
};

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_i128();
    let k = input.read_i128();

    // let mut luck = 0;

    let mut f = RecursiveFunction2::new(|f, l, r| {
        if r - l + 1 < k {
            return (0, 0);
        }
        let m = (l + r) / 2;
        let diff = (r - l + 1) / 2;
        if (r - l + 1) % 2 == 0 {
            // f.call(l, m);
            // f.call(m + 1, r);
            let (luck, count) = f.call(l, m);
            (luck + luck + count * diff, count * 2)
        } else {
            // luck += m;
            // f.call(l, m - 1);
            // f.call(m + 1, r);
            let (luck, count) = f.call(l, m - 1);
            (m + luck + luck + count * (diff + 1), count * 2 + 1)
        }
    });

    let ans = f.call(1, n);

    out.print_line(ans.0);
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
