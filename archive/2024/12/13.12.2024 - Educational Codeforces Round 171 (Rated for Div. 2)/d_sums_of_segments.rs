//{"name":"D. Sums of Segments","group":"Codeforces - Educational Codeforces Round 171 (Rated for Div. 2)","url":"https://codeforces.com/contest/2026/problem/D","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n1 2 5 10\n15\n1 1\n1 2\n1 3\n1 4\n1 5\n1 10\n5 10\n6 10\n2 8\n3 4\n3 10\n3 8\n5 6\n5 5\n1 8\n","output":"1\n4\n12\n30\n32\n86\n56\n54\n60\n26\n82\n57\n9\n2\n61\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DSumsOfSegments"}}}

#![allow(
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::let_and_return
)]

#[allow(unused_imports)]
use algo_lib::dbg;
use algo_lib::{
    collections::md_arr::arr2d::{Arr2d, Arr2dCharWrite},
    io::{input::Input, output::Output},
    misc::{
        recursive_function::{Callable2, RecursiveFunction2, RecursiveFunction3},
        test_type::TaskType,
    },
};

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let size = input.read_size();
    let vec = input.read_long_vec(size);

    let mut prefix_vec = vec![0];
    for &e in &vec {
        prefix_vec.push(prefix_vec.last().unwrap() + e);
    }

    // sum(vec[l..=r]) i.e. s(l, r)
    let query_vec = |l: usize, r: usize| prefix_vec[r + 1] - prefix_vec[l];

    let table = {
        let mut k = 0;
        while 1 << k <= size {
            k += 1;
        }
        let mut table = Arr2d::new(k + 1, size, 0);

        for i in 0..size {
            table[(0, i)] = vec[i];
        }

        for i in 1..k {
            let p = 1 << i;

            for j in 0..=size - p {
                let split = j + p / 2;
                table[(i, j)] = table[(i - 1, j)]
                    + (p as i64 / 2 * query_vec(j, j + p / 2 - 1))
                    + table[(i - 1, split)];
            }
        }
        table
    };

    // S(l, l) + S(l, l + 1) + ... + S(l, r)
    let query_sum = |mut l: usize, r: usize| -> i64 {
        let mut total = 0;
        let k = table.d1();
        for i in (0..k).rev() {
            let p = 1 << i;
            let diff = 1 + r - l;
            if p <= diff {
                let m = (diff - p) as i64;
                total += table[(i, l)] + m * query_vec(l, l + p - 1);
                l += p;
            }
        }
        total
    };

    let mut sums_triangle = vec![0; size];
    sums_triangle[size - 1] = vec[size - 1];
    for i in (0..size - 1).rev() {
        sums_triangle[i] = sums_triangle[i + 1] + (size - i) as i64 * vec[i];
    }
    let mut prefix_triangle = vec![0];
    for &e in sums_triangle.iter() {
        prefix_triangle.push(prefix_triangle.last().unwrap() + e);
    }
    let query_triangle = |l: usize, r: usize| prefix_triangle[r + 1] - prefix_triangle[l];

    //  1 -> 0
    //  2 -> 1
    //  3 -> 1
    //  4 -> 2
    //
    //  both outputs indexed from 0
    let search_index = |n: usize| -> (usize, usize) {
        let target = (size * (size + 1) / 2) - n + 1;
        let mut low = 0;
        let mut high = size * (size + 1);
        // last yes

        while low + 1 < high {
            let mid = (low + high) / 2;

            let value = mid * (mid + 1) / 2;
            if value < target {
                low = mid;
            } else {
                high = mid;
            }
        }
        (size - low - 1, ((low + 1) * (low + 2) / 2 - target))
    };

    let query_size = input.read_size();
    for _ in 0..query_size {
        let l = input.read_size();
        let r = input.read_size();

        let (lt, li) = search_index(l);
        let (rt, ri) = search_index(r);

        let mut total = 0;
        if lt + 1 < rt {
            total += query_triangle(lt + 1, rt - 1);
        }

        if lt == rt {
            total += query_sum(lt, lt + ri)
                - if li == 0 {
                    0
                } else {
                    query_sum(lt, lt + li - 1)
                };
        } else {
            total += query_sum(lt, size - 1)
                - if li == 0 {
                    0
                } else {
                    query_sum(lt, lt + li - 1)
                };
            total += query_sum(rt, rt + ri);
        }

        out.print_line(total);
    }
}

pub static TEST_TYPE: TestType = TestType::Single;
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
