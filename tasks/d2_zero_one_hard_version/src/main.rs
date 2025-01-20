//{"name":"D2. Zero-One (Hard Version)","group":"Codeforces - Codeforces Round 821 (Div. 2)","url":"https://codeforces.com/contest/1733/problem/D2","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n5 8 9\n01001\n00101\n6 2 11\n000001\n100000\n5 7 2\n01000\n11011\n7 8 3\n0111001\n0100001\n6 3 4\n010001\n101000\n5 10 1\n01100\n01100\n","output":"8\n10\n-1\n6\n7\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D2ZeroOneHardVersion"}}}

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
    let size = input.read_size();
    let x = input.read_long();
    let y = input.read_long();

    let s1 = (0..size)
        .map(|_| input.read_char() == '1')
        .collect::<Vec<_>>();
    let s2 = (0..size)
        .map(|_| input.read_char() == '1')
        .collect::<Vec<_>>();

    let mut ind = vec![];
    for (i, (&a, &b)) in s1.iter().zip(s2.iter()).enumerate() {
        if a != b {
            ind.push(i);
        }
    }

    dbg!(ind);

    if ind.len() % 2 == 1 {
        out.print_line(-1);
        return;
    }

    if x >= y {
        if ind.len() == 2 && ind[0] + 1 == ind[1] {
            if y * 2 < x && size >= 3 {
                out.print_line(y * 2);
            } else {
                out.print_line(x);
            }
        } else {
            out.print_line(y * ind.len() as i64 / 2);
        }
    } else {
        out.print_line(-2);
    }
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
