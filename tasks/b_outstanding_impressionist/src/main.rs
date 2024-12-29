//{"name":"B. Outstanding Impressionist","group":"Codeforces - Good Bye 2024: 2025 is NEAR","url":"https://codeforces.com/contest/2053/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2\n1 1\n1 1\n4\n1 3\n1 3\n1 3\n1 3\n6\n3 6\n2 2\n1 2\n1 1\n3 4\n2 2\n7\n3 4\n4 4\n4 4\n1 3\n2 5\n1 4\n2 2\n3\n4 5\n4 4\n5 5\n","output":"00\n1111\n100110\n1001111\n011\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BOutstandingImpressionist"}}}

#![allow(
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::comparison_chain,
    clippy::let_and_return,
    clippy::needless_range_loop
)]

use std::collections::BTreeSet;

#[allow(unused_imports)]
use algo_lib::dbg;
use algo_lib::{
    collections::fxhash::{FxHashMap, FxHashSet},
    io::{input::Input, output::Output},
    misc::test_type::TaskType,
};

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let size = input.read_size();
    let arr = input.read_vec::<(usize, usize)>(size);

    let mut set = (0..=size * 2).collect::<BTreeSet<_>>();

    let mut counts = FxHashMap::default();

    for &(a, b) in arr.iter() {
        if a == b {
            set.remove(&a);
            *counts.entry(a).or_insert(0) += 1;
        }
    }

    // dbg!(set);
    // dbg!(counts);

    let mut ans = vec![];
    for &(a, b) in arr.iter() {
        if a == b {
            ans.push(counts[&a] == 1);
        } else {
            ans.push(set.range(a..=b).next().is_some());
        }
    }
    // dbg!(ans);

    out.print_line(
        ans.iter()
            .map(|&b| if b { '1' } else { '0' })
            .collect::<String>(),
    );
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
