//{"name":"E. Resourceful Caterpillar Sequence","group":"Codeforces - Good Bye 2024: 2025 is NEAR","url":"https://codeforces.com/contest/2053/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2\n1 2\n5\n1 2\n1 3\n2 4\n2 5\n12\n1 6\n11 2\n4 8\n12 3\n2 7\n6 12\n8 1\n2 3\n5 12\n9 2\n10 3\n10\n1 2\n2 3\n3 4\n4 5\n5 6\n4 7\n6 8\n4 9\n4 10\n25\n1 16\n11 22\n6 14\n3 1\n20 14\n23 17\n25 19\n10 11\n3 18\n10 6\n2 21\n4 5\n11 12\n4 9\n9 13\n8 6\n6 1\n3 7\n8 19\n10 24\n15 13\n1 2\n3 4\n17 8\n","output":"0\n6\n40\n27\n171\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EResourcefulCaterpillarSequence"}}}

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
    collections::fxhash::FxHashSet,
    io::{input::Input, output::Output},
    misc::test_type::TaskType,
};

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

pub fn gen_graph(edges: &[(usize, usize)], len: usize) -> Vec<Vec<usize>> {
    let mut graph = vec![vec![]; len];
    for &(a, b) in edges {
        graph[a].push(b);
        graph[b].push(a);
    }
    graph
}

pub fn find_leaves(graph: &[Vec<usize>]) -> Vec<usize> {
    graph
        .iter()
        .enumerate()
        .filter_map(|(i, neighbours)| (neighbours.len() == 1).then(|| i))
        .collect()
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let size = input.read_size();
    let mut edges = input.read_vec::<(usize, usize)>(size - 1);

    for e in edges.iter_mut() {
        e.0 -= 1;
        e.1 -= 1;
    }

    let graph = gen_graph(&edges, size);
    let leaves = find_leaves(&graph);

    let mut nl = FxHashSet::default();
    for &l in leaves.iter() {
        for &n in graph[l].iter() {
            nl.insert(n);
        }
    }
    for &l in leaves.iter() {
        nl.remove(&l);
    }

    let nl_count = nl.len() as i64;

    let mut vert_count = size as i64;
    let mut edges_count = size as i64 - 1;
    let mut leaves_count = leaves.len() as i64;
    let mut non_count = vert_count - leaves_count;

    let mut total = 0;

    total += leaves_count * (vert_count - leaves_count);
    total += nl_count * (vert_count - nl_count - leaves_count);

    dbg!(leaves);
    dbg!(nl);
    dbg!(leaves_count, vert_count, nl_count);
    out.print_line(total);
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
