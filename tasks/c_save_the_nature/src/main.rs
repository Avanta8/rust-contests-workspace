//{"name":"C. Save the Nature","group":"Codeforces - Technocup 2020 - Elimination Round 1","url":"https://codeforces.com/problemset/problem/1223/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\n100\n50 1\n49 1\n100\n8\n100 200 100 200 100 200 100 100\n10 2\n15 3\n107\n3\n1000000000 1000000000 1000000000\n50 1\n50 1\n3000000000\n5\n200 100 100 100 100\n69 5\n31 2\n90\n","output":"-1\n6\n3\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSaveTheNature"}}}

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
    numbers::gcd::lcm,
};

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let size = input.read_size();
    let mut arr = input
        .read_long_vec(size)
        .into_iter()
        .map(|x| x / 100)
        .collect::<Vec<_>>();
    arr.sort();
    arr.reverse();
    let (mut pera, mut stepa) = input.read::<(i64, i64)>();
    let (mut perb, mut stepb) = input.read::<(i64, i64)>();
    let req = input.read_long();

    if pera < perb {
        std::mem::swap(&mut pera, &mut perb);
        std::mem::swap(&mut stepa, &mut stepb);
    }

    let stepm = lcm(stepa, stepb);

    let check = |mut count: i64| -> bool {
        count = count.min(size as i64);
        let mut tickets = arr[..count as usize].to_vec();
        tickets.reverse();

        let mut double = count / stepm;
        let mut sa = count / stepa - double;
        let mut sb = count / stepb - double;

        let mut total = 0;
        while let Some(price) = tickets.pop() {
            if double > 0 {
                double -= 1;
                total += pera * price + perb * price;
            } else if sa > 0 {
                sa -= 1;
                total += pera * price;
            } else if sb > 0 {
                sb -= 1;
                total += perb * price;
            } else {
                break;
            }
        }

        total >= req
    };

    let mut low = 0;
    let mut high = size as i64;
    if !check(high) {
        out.print_line(-1);
        return;
    }

    while low + 1 < high {
        let mid = (low + high) / 2;
        if check(mid) {
            high = mid;
        } else {
            low = mid;
        }
    }
    out.print_line(high);
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
