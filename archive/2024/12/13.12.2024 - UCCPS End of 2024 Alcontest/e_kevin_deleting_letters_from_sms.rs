//{"name":"E. Kevin Deleting Letters from SMS","group":"Codeforces - UCCPS End of 2024 Alcontest","url":"https://codeforces.com/group/NjBkaVbpba/contest/566403/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4 1\nyxwv\n9 8\ntqbjtqmif\n10 5\neatqvgjpog\n10 8\ntpoudnqtob\n","output":"v\nqbjtqmif\nagjog\noudnqtob\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EKevinDeletingLettersFromSMS"}}}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use algo_lib::collections::segment_tree::SegmentTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    let letters = input.read_string().into_bytes();

    let mut segtree = SegmentTree::new(n, |a: (u8, usize), b| a.min(b));
    segtree.fill(letters.into_iter().zip(0..).collect::<Vec<_>>());

    let mut start = 0;
    let mut ans = vec![];
    for end in n - k..n {
        let (value, idx) = segtree.query(start..=end).unwrap();
        start = idx + 1;
        ans.push(value);
    }
    out.print_line(String::from_utf8(ans).unwrap());
}

// fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
//     let n = input.read_size();
//     let k = input.read_size();
//
//     let letters = input.read_string().into_bytes();
//
//     let mut heap = letters
//         .iter()
//         .take(n - k)
//         .copied()
//         .zip(0..)
//         .map(Reverse)
//         .collect::<BinaryHeap<_>>();
//
//     let mut ans = vec![];
//     let mut last_idx = 0;
//     for (idx, &next) in letters.iter().enumerate().skip(n - k) {
//         heap.push(Reverse((next, idx)));
//         while let Some(Reverse((best, i))) = heap.pop() {
//             if i >= last_idx {
//                 ans.push(best);
//                 last_idx = i;
//                 break;
//             }
//         }
//     }
//
//     out.print_line(String::from_utf8(ans).unwrap());
// }

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
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
