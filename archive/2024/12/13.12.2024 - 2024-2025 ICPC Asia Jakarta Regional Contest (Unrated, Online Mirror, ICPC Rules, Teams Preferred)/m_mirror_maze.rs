//{"name":"M. Mirror Maze","group":"Codeforces - 2024-2025 ICPC Asia Jakarta Regional Contest (Unrated, Online Mirror, ICPC Rules, Teams Preferred)","url":"https://codeforces.com/problemset/problem/2045/M","interactive":false,"timeLimit":1000,"tests":[{"input":"4 4\n.//.\n.\\\\.\n.\\/.\n....\n","output":"2\nN3 W2\n"},{"input":"4 6\n./..\\.\n.\\...\\\n./../\\\n......\n","output":"2\nE3 S2\n"},{"input":"4 4\n....\n./\\.\n.\\/.\n....\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MMirrorMaze"}}}

#[allow(unused_imports)]
use algo_lib::dbg;
use algo_lib::{
    collections::{
        fxhash::FxHashSet,
        md_arr::arr2d::{Arr2d, Arr2dRead},
    },
    io::{input::Input, output::Output},
    misc::{
        directions::{Step, Stepper, D4},
        test_type::TaskType,
    },
};

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn simulate(
    grid: &Arr2d<char>,
    start: (usize, usize),
    mut dir: D4,
    stepper: &Stepper<usize, D4>,
) -> FxHashSet<(usize, usize)> {
    let mut current = start;
    let mut seen = FxHashSet::default();
    while !seen.contains(&(current, dir)) {
        seen.insert((current, dir));
        match grid[current] {
            '\\' => {
                dir = match dir {
                    D4::N => D4::W,
                    D4::E => D4::S,
                    D4::S => D4::E,
                    D4::W => D4::N,
                }
            }
            '/' => {
                dir = match dir {
                    D4::N => D4::E,
                    D4::E => D4::N,
                    D4::S => D4::W,
                    D4::W => D4::S,
                }
            }
            _ => (),
        }

        let Some(new_current) = stepper.step(current, dir) else {
            break;
        };
        current = new_current;
    }
    seen.into_iter().map(|(p, _)| p).collect()
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let size = (input.read_size(), input.read_size());

    let grid = input.read_char_table(size.0, size.1);

    let mut mirrors = FxHashSet::default();
    for d0 in 0..size.0 {
        for d1 in 0..size.1 {
            match grid[(d0, d1)] {
                '\\' | '/' => {
                    mirrors.insert((d0, d1));
                }
                _ => (),
            }
        }
    }

    let stepper = Stepper::new_max(size);
    let mut possible = vec![];

    let valid = |seen: &FxHashSet<(usize, usize)>| {
        for m in mirrors.iter() {
            if !seen.contains(m) {
                return false;
            }
        }
        true
    };

    for s1 in 0..size.1 {
        let seen = simulate(&grid, (0, s1), D4::S, &stepper);
        if valid(&seen) {
            possible.push(format!("N{}", s1 + 1));
        }
        let seen = simulate(&grid, (size.0 - 1, s1), D4::N, &stepper);
        if valid(&seen) {
            possible.push(format!("S{}", s1 + 1));
        }
    }
    for s0 in 0..size.0 {
        let seen = simulate(&grid, (s0, 0), D4::E, &stepper);
        if valid(&seen) {
            possible.push(format!("W{}", s0 + 1));
        }
        let seen = simulate(&grid, (s0, size.1 - 1), D4::W, &stepper);
        if valid(&seen) {
            possible.push(format!("E{}", s0 + 1));
        }
    }

    out.print_line(possible.len());
    out.print_line(&possible);
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
