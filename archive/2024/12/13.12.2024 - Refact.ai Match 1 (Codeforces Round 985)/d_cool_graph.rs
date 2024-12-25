//{"name":"D. Cool Graph","group":"Codeforces - Refact.ai Match 1 (Codeforces Round 985)","url":"https://codeforces.com/contest/2029/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n3 0\n3 1\n1 2\n3 2\n1 2\n2 3\n3 3\n1 2\n2 3\n3 1\n6 6\n1 2\n1 6\n4 5\n3 4\n4 6\n3 6\n","output":"0\n1\n1 2 3\n0\n1\n1 2 3\n3\n1 3 6\n2 4 5\n3 4 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DCoolGraph"}}}

#![allow(
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::collapsible_match,
    clippy::comparison_chain,
    clippy::let_and_return
)]

use std::{collections::VecDeque, default};

#[allow(unused_imports)]
use algo_lib::dbg;
use algo_lib::{
    collections::{
        fxhash::{FxHashMap, FxHashSet},
        graph::gen_graph,
    },
    io::{input::Input, output::Output},
    misc::test_type::TaskType,
};

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let size = input.read_size();
    let edges_size = input.read_size();

    let edges = input
        .read_size_pair_vec(edges_size)
        .into_iter()
        .map(|(a, b)| (a - 1, b - 1))
        .collect::<Vec<_>>();
    let mut graph = gen_graph(&edges, size)
        .into_iter()
        .map(|row| row.into_iter().collect::<FxHashSet<_>>())
        .collect::<Vec<_>>();

    let mut operations = vec![];

    for node in 0..size {
        while graph[node].len() > 1 {
            let a = *graph[node].iter().next().unwrap();
            graph[node].remove(&a);
            graph[a].remove(&node);
            let b = *graph[node].iter().next().unwrap();
            graph[node].remove(&b);
            graph[b].remove(&node);

            operations.push([node, a, b]);
            if graph[a].contains(&b) {
                graph[a].remove(&b);
                graph[b].remove(&a);
            } else {
                graph[a].insert(b);
                graph[b].insert(a);
            }
        }
    }

    let mut multiple = vec![];
    let mut single = vec![];
    let mut seen = vec![false; size];
    for start in 0..size {
        if seen[start] {
            continue;
        }
        let mut link = None;
        let mut bag = VecDeque::new();
        bag.push_front(start);

        while let Some(current) = bag.pop_back() {
            if seen[current] {
                continue;
            }
            seen[current] = true;

            for &neighbour in graph[current].iter() {
                bag.push_front(neighbour);
                link = Some((current, neighbour));
            }
        }

        if let Some((a, b)) = link {
            multiple.push((a, b));
        } else {
            single.push(start);
        }
    }

    if let Some((a, b)) = multiple.pop() {
        for (u, v) in multiple {
            operations.push([a, u, v]);
        }
        if !single.is_empty() {
            for (&u, &v) in single.iter().zip(single.iter().skip(1)) {
                operations.push([a, u, v]);
            }
            operations.push([a, *single.last().unwrap(), b]);
        }
    }

    let mut operation_counts = FxHashMap::default();
    for mut op in operations {
        op = [op[0] + 1, op[1] + 1, op[2] + 1];
        op.sort_unstable();
        *operation_counts.entry(op).or_insert(0) += 1;
    }

    let ops = operation_counts
        .into_iter()
        .filter(|(_, count)| count % 2 == 1)
        .map(|item| item.0)
        .collect::<Vec<_>>();

    out.print_line(ops.len());
    for op in ops {
        out.print_line(op);
    }

    // dbg!(graph);
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
