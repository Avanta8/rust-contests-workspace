//{"name":"D. Move Back at a Cost","group":"Codeforces - Codeforces Round 990 (Div. 2)","url":"https://codeforces.com/contest/2047/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\n2 1 3\n5\n1 2 2 1 4\n6\n1 2 3 6 5 4\n","output":"1 3 3\n1 1 3 3 5\n1 2 3 4 6 7\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMoveBackAtACost"}}}

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
    collections::segment_tree::SegmentTree,
    io::{input::Input, output::Output},
    misc::test_type::TaskType,
};

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let size = input.read::<usize>();
    let arr = input.read_vec::<i64>(size);

    let mut stack = vec![]; // increasing
    let mut next_smaller = vec![false; size];
    for (i, elem) in arr.iter().copied().enumerate().rev() {
        while let Some(&top) = stack.last() {
            if elem > top {
                next_smaller[i] = true;
                break;
            }
            stack.pop();
        }
        stack.push(elem);
    }

    let mut min_moved = i64::MAX;
    let mut orig = vec![];
    let mut moved = vec![];
    for (i, a) in arr.iter().copied().enumerate() {
        if a > min_moved || next_smaller[i] {
            min_moved = min_moved.min(a + 1);
            moved.push(a + 1);
        } else {
            orig.push(a);
        }
    }

    moved.sort();
    orig.extend(moved);
    out.print_line(orig);
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
