//{"name":"C. Sums on Segments","group":"Codeforces - Educational Codeforces Round 173 (Rated for Div. 2)","url":"https://codeforces.com/contest/2043/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n5\n1 -1 10 1 1\n5\n-1 -1 -1 -1 -1\n2\n-1 2\n2\n7 1\n3\n1 4 -1\n","output":"8\n-1 0 1 2 9 10 11 12\n6\n-5 -4 -3 -2 -1 0\n4\n-1 0 1 2\n4\n0 1 7 8\n6\n-1 0 1 3 4 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSumsOnSegments"}}}

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
    collections::fxhash::FxHashSet,
    io::{input::Input, output::Output},
    misc::test_type::TaskType,
};

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let size = input.read_size();
    let arr = input.read_long_vec(size);

    // let mut before_range = (0, 0);
    // let mut after_range = (0, 0);
    let mut current = (0, 0);
    let mut after = None;

    let mut numbers = FxHashSet::default();
    numbers.insert(0);

    for (i, &n) in arr.iter().enumerate() {
        // dbg!(current, after);
        if n == 1 {
            current.1 += 1;
            current.0 += 1;
            if current.1 == 0 {
                current.1 = 1;
            }
            if current.0 == 2 {
                current.0 = 1;
            }
            numbers.insert(current.1);
            if let Some((low, high)) = after {
                numbers.insert(high + 1);
                after = Some((low + 1, high + 1));
            }
        } else if n == -1 {
            current.1 -= 1;
            current.0 -= 1;
            if current.0 == 0 {
                current.0 = -1;
            }
            if current.1 == -2 {
                current.1 = -1;
            }
            numbers.insert(current.0);
            if let Some((low, high)) = after {
                numbers.insert(low - 1);
                after = Some((low - 1, high - 1));
            }
        } else {
            let low = n.min(current.0 + n);
            let high = n.max(current.1 + n);
            after = Some((low, high));
            for x in low..=high {
                numbers.insert(x);
            }
            current = (0, 0);
        }
    }

    // dbg!(numbers);
    let mut ans = numbers.into_iter().collect::<Vec<_>>();
    ans.sort();
    out.print_line(ans.len());
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
