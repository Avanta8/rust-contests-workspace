//{"name":"D. Non Prime Tree","group":"Codeforces - Codeforces Round 992 (Div. 2)","url":"https://codeforces.com/problemset/problem/2040/D","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n5\n1 2\n2 3\n2 4\n3 5\n7\n1 2\n1 3\n2 4\n3 5\n3 6\n3 7\n","output":"2 10 1 6 5\n8 7 12 1 4 6 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DNonPrimeTree"}}}

#![allow(
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::comparison_chain,
    clippy::let_and_return,
    clippy::needless_range_loop
)]

use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap, VecDeque},
};

#[allow(unused_imports)]
use algo_lib::dbg;
use algo_lib::{
    collections::graph::gen_graph,
    io::{input::Input, output::Output},
    misc::test_type::TaskType,
};

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let size = input.read_size();
    let mut edges = input.read_size_pair_vec(size - 1);

    for (a, b) in edges.iter_mut() {
        *a -= 1;
        *b -= 1;
    }

    let graph = gen_graph(&edges, size);

    let mut sieve = vec![true; size * 2];
    sieve[0] = false;
    sieve[1] = false;

    for i in 2..sieve.len() {
        if !sieve[i] {
            continue;
        }
        for j in (i..sieve.len()).step_by(i).skip(1) {
            sieve[j] = false;
        }
    }

    let mut choices = (2..=size * 2).rev().collect::<Vec<_>>();

    let mut vals = vec![0; size];
    vals[0] = 1;

    let mut bag = BinaryHeap::from([(Reverse(1), 0)]);

    let mut parent = vec![usize::MAX; size];
    let mut rem = vec![vec![]; size];
    rem[0] = graph[0].clone();

    while let Some((Reverse(val), node)) = bag.pop() {
        assert_eq!(vals[node], val);

        let mut next = rem[node].pop().unwrap();

        parent[next] = node;

        if !rem[node].is_empty() {
            bag.push((Reverse(val), node));
        }

        while sieve[choices.last().unwrap() - val] {
            choices.pop();
        }

        loop {
            vals[next] = choices.pop().unwrap();
            let mut neighbours = graph[next]
                .iter()
                .copied()
                .filter(|&n| n != parent[next])
                .collect::<Vec<_>>();

            if let Some(nn) = neighbours.pop() {
                if !neighbours.is_empty() {
                    bag.push((Reverse(vals[next]), next));
                    rem[next] = neighbours;
                }
                parent[nn] = next;
                next = nn;
            } else {
                break;
            }
        }
    }

    out.print_line(vals);
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
