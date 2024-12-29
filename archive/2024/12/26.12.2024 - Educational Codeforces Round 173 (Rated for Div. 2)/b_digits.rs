//{"name":"B. Digits","group":"Codeforces - Educational Codeforces Round 173 (Rated for Div. 2)","url":"https://codeforces.com/contest/2043/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2 6\n7 1\n8 5\n","output":"1 3\n1 3 7 9\n1 3 5 7 9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BDigits"}}}

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

fn calc_cycle(d: i64, m: i64) -> Vec<i64> {
    let mut cycle = vec![];

    let mut n = String::new();
    loop {
        let p = if n.is_empty() {
            0
        } else {
            n.parse::<i64>().unwrap() % m
        };
        if cycle.contains(&p) {
            break;
        }
        cycle.push(p);
        n.push_str(&format!("{}", d));
    }

    cycle
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let times = input.read_long();
    let d = input.read_long();

    let mut ans = vec![1];
    if d % 3 == 0 || times >= 3 {
        ans.push(3);
    }
    if d % 5 == 0 {
        ans.push(5);
    }
    if d % 7 == 0 {
        ans.push(7);
    } else {
        if times >= 3 {
            ans.push(7)
        }
    }
    if d % 9 == 0 {
        ans.push(9);
    } else {
        let cycle = calc_cycle(d, 9);
        if cycle.len() == 9 {
            if times >= 6 {
                ans.push(9);
            }
        } else if cycle.len() == 3 {
            if times >= 3 {
                ans.push(9);
            }
        } else {
            unreachable!();
        }
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
