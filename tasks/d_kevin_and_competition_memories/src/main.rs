//{"name":"D. Kevin and Competition Memories","group":"Codeforces - Codeforces Global Round 28","url":"https://codeforces.com/contest/2048/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4 4\n4 3 7 5\n2 5 4 6\n5 5\n5 0 4 8 6\n1 3 9 2 7\n6 7\n1 1 4 5 1 4\n1 9 1 9 8 1 0\n7 6\n1 9 1 9 8 1 0\n1 1 4 5 1 4\n","output":"7 4 2 3\n6 2 1 1 2\n7 3 2 1 1 1 1\n15 9 5 4 4 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DKevinAndCompetitionMemories"}}}

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
    let ratings_size = input.read_size();
    let difficulties_size = input.read_size();

    let mut ratings = input.read_long_vec(ratings_size);
    let difficulties = input.read_long_vec(difficulties_size);

    let kevin = ratings.swap_remove(0);
    ratings.sort();
    ratings.reverse();

    let mut ranks = difficulties
        .iter()
        .map(|&difficulty| {
            if difficulty <= kevin {
                1
            } else {
                ratings.partition_point(|&rating| rating >= difficulty) as i64 + 1
            }
        })
        .collect::<Vec<_>>();
    ranks.sort();

    let mut ans = vec![0; difficulties_size];
    for k in 0..difficulties_size {
        for idx in (k..difficulties_size).step_by(k + 1) {
            ans[k] += ranks[idx];
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
