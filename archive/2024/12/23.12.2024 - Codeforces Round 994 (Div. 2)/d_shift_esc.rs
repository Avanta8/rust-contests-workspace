//{"name":"D. Shift + Esc","group":"Codeforces - Codeforces Round 994 (Div. 2)","url":"https://codeforces.com/contest/2049/problem/D","interactive":false,"timeLimit":2500,"tests":[{"input":"5\n3 3 100\n3 4 9\n5 2 4\n0 101 101\n3 4 1\n10 0 0 10\n0 0 10 0\n10 10 0 10\n1 1 3\n4\n3 2 3\n1 2\n3 6\n5 4\n10 10 14\n58 49 25 12 89 69 8 49 71 23\n45 27 65 59 36 100 73 23 5 84\n82 91 54 92 53 15 43 46 11 65\n61 69 71 87 67 72 51 42 55 80\n1 64 8 54 61 70 47 100 84 50\n86 93 43 51 47 35 56 20 33 61\n100 59 5 68 15 55 69 8 8 60\n33 61 20 79 69 51 23 24 56 28\n67 76 3 69 58 79 75 10 65 63\n6 64 73 79 17 62 55 53 61 58\n","output":"113\n6\n4\n13\n618\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DShiftEsc"}}}

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
    collections::md_arr::arr2d::Arr2dRead,
    io::{input::Input, output::Output},
    misc::test_type::TaskType,
};

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

const INF: i64 = i64::MAX >> 1;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let rows = input.read_size();
    let cols = input.read_size();
    let k = input.read_long();

    let grid = input.read_long_table(rows, cols);

    let get = |row: usize, col: usize, shift: usize| -> i64 { grid[row][(col + shift) % cols] };

    let mut dp = vec![vec![vec![[INF; 2]; cols]; cols]; rows]; // dp[row][col][row shift][moved]

    dp[0][0][0][0] = get(0, 0, 0);

    for row in 0..rows {
        for col in 0..cols {
            for shift in 0..cols {
                let value = &mut dp[row][col][shift];
                if value[0] < value[1] {
                    value[1] = value[0];
                }
                let value = *value;

                if shift + 1 < cols {
                    dp[row][col][shift + 1][0] = (dp[row][col][shift + 1][0])
                        .min(value[0] - get(row, col, shift) + get(row, col, shift + 1) + k);
                }

                if col + 1 < cols {
                    dp[row][col + 1][shift][1] =
                        (dp[row][col + 1][shift][1]).min(value[1] + get(row, col + 1, shift));
                }

                if row + 1 < rows {
                    dp[row + 1][col][0][0] =
                        (dp[row + 1][col][0][0]).min(value[1] + get(row + 1, col, 0));
                }
            }
        }
    }

    out.print_line(dp[rows - 1][cols - 1].iter().map(|[_, b]| b).min().unwrap());
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
