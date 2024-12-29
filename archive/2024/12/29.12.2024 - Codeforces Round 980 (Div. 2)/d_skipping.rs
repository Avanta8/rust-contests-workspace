//{"name":"D. Skipping","group":"Codeforces - Codeforces Round 980 (Div. 2)","url":"https://codeforces.com/contest/2024/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2\n15 16\n2 1\n5\n10 10 100 100 1000\n3 4 1 1 1\n3\n100 49 50\n3 2 2\n4\n100 200 300 1000\n2 3 4 1\n","output":"16\n200\n100\n1000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSkipping"}}}

#![allow(
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::comparison_chain,
    clippy::let_and_return,
    clippy::needless_range_loop
)]

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

#[allow(unused_imports)]
use algo_lib::dbg;
use algo_lib::{
    io::{input::Input, output::Output},
    misc::{
        memo::{memoization::Memoization2, memoization_vec::Memoization1d},
        recursive_function::{Callable, Callable2},
        test_type::TaskType,
    },
};

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let size = input.read_size();
    let scores = input.read_long_vec(size);
    let mut jumps = input.read_size_vec(size);

    for x in jumps.iter_mut() {
        *x -= 1;
    }

    let mut costs = vec![i64::MAX / 2; size];

    let mut bag = BinaryHeap::from([(Reverse(0), 0)]);
    while let Some((Reverse(cost), idx)) = bag.pop() {
        if cost >= costs[idx] {
            continue;
        }
        costs[idx] = cost;

        if idx > 0 {
            bag.push((Reverse(cost), idx - 1));
        }
        bag.push((Reverse(cost + scores[idx]), jumps[idx]))
    }

    let mut total = 0;
    let mut best = 0;
    for (cost, score) in costs.into_iter().zip(scores) {
        total += score;
        best = best.max(total - cost);
    }

    out.print_line(best);
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
