//{"name":"C. Trip to the Olympiad","group":"Codeforces - Hello 2025","url":"https://codeforces.com/contest/2057/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n0 2\n0 8\n1 3\n6 22\n128 137\n69 98\n115 127\n0 1073741823\n","output":"1 2 0\n8 7 1\n2 1 3\n7 16 11\n134 132 137\n98 85 76\n123 121 118\n965321865 375544086 12551794\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTripToTheOlympiad"}}}

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
    let low = input.read_u64();
    let high = input.read_u64();

    let mut current = (high + 1).next_power_of_two() >> 1;
    let mut res = 0;
    loop {
        let high_val = high & current;
        let low_val = low & current;

        if high_val == 0 {
            assert_eq!(low_val, 0);
        }

        if high_val == low_val {
            res |= current & high_val;
            current >>= 1;
        } else {
            assert!(high_val > 0);
            assert_eq!(low_val, 0);
            break;
        }
    }

    let mut ans = if current + res == high {
        [current, current - 1, current - 2]
    } else {
        [current + 1, current, current - 1]
    };

    dbg!(res);
    dbg!(ans);
    for e in ans.iter_mut() {
        assert_eq!(*e | res, *e + res);
        *e += res;
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
