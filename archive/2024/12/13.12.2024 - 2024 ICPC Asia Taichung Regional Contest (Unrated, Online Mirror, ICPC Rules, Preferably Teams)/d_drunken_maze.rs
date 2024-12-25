//{"name":"D. Drunken Maze","group":"Codeforces - 2024 ICPC Asia Taichung Regional Contest (Unrated, Online Mirror, ICPC Rules, Preferably Teams)","url":"https://codeforces.com/problemset/problem/2041/D","interactive":false,"timeLimit":2000,"tests":[{"input":"7 12\n############\n#S........T#\n#.########.#\n#..........#\n#..........#\n#..#..#....#\n############\n","output":"15\n"},{"input":"5 8\n########\n#......#\n#.####.#\n#...T#S#\n########\n","output":"14\n"},{"input":"5 8\n########\n#.#S...#\n#.####.#\n#...T#.#\n########\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DDrunkenMaze"}}}

use std::collections::VecDeque;

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::collections::md_arr::arr2d::Arr2dRead;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::directions::{Direction, Step, Stepper, D4};
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let d1 = input.read_size();
    let d2 = input.read_size();

    let grid = input.read_byte_table(d1, d2);

    let (start, end) = {
        let mut start = None;
        let mut end = None;
        for r in 0..d1 {
            for c in 0..d2 {
                let pos = (r, c);
                match grid.get(r, c) {
                    b'S' => start = Some(pos),
                    b'T' => end = Some(pos),
                    _ => (),
                }
            }
        }
        (start.unwrap(), end.unwrap())
    };

    let stepper = Stepper::new_max((d1, d2));

    let mut seen = Arr2d::new(d1, d2, [[false; 4]; 4]);
    let mut bag = VecDeque::new();
    for d in D4::iter().take(1) {
        bag.push_back((start, d, 0, 0));
    }

    while let Some((pos, dir, count, total)) = bag.pop_front() {
        if pos == end {
            out.print_line(total);
            return;
        }

        for (i, next_dir) in D4::iter().enumerate() {
            let next_count = if next_dir == dir { count + 1 } else { 1 };
            if next_count > 3 {
                continue;
            }
            let Some(next_pos) = stepper.step(pos, next_dir) else {
                continue;
            };
            if grid[next_pos] == b'#' {
                continue;
            }

            let s = &mut seen[next_pos][i][next_count];
            if !*s {
                *s = true;
                bag.push_back((next_pos, next_dir, next_count, total + 1));
            }
        }
    }

    out.print_line(-1);
}

pub static TEST_TYPE: TestType = TestType::Single;
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
