// https://codeforces.com/contest/2053/problem/E
pub mod solution {
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
use crate::dbg;
use crate::algo_lib::collections::fxhash::FxHashSet;
use crate::algo_lib::io::input::Input;
use crate::algo_lib::io::output::Output;
use crate::algo_lib::misc::test_type::TaskType;

use crate::algo_lib::misc::test_type::TestType;

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

}
pub mod algo_lib {
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]
#![allow(clippy::missing_safety_doc)]

pub mod collections {
pub mod fxhash {
// Copyright 2015 The Rust Project Developers. See the COPYRIGHT at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cell::Cell;
use std::convert::TryInto;
use std::time::SystemTime;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::BuildHasherDefault;
use std::hash::Hasher;
use std::mem::size_of;
use std::ops::BitXor;

pub type FxHashMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

pub type FxHashSet<V> = HashSet<V, BuildHasherDefault<FxHasher>>;

#[derive(Default)]
pub struct FxHasher {
    hash: usize,
}

thread_local! {
    static K: Cell<usize> = Cell::new(
        ((SystemTime::UNIX_EPOCH.elapsed().unwrap().as_nanos().wrapping_mul(2) + 1) & 0xFFFFFFFFFFFFFFFF) as usize
    );
}

impl FxHasher {
    #[inline]
    fn add_to_hash(&mut self, i: usize) {
        self.hash = self
            .hash
            .rotate_left(5)
            .bitxor(i)
            .wrapping_mul(K.with(|k| k.get()));
    }
}

impl Hasher for FxHasher {
    #[inline]
    fn write(&mut self, mut bytes: &[u8]) {
        let read_usize = |bytes: &[u8]| u64::from_ne_bytes(bytes[..8].try_into().unwrap());

        let mut hash = FxHasher { hash: self.hash };
        while bytes.len() >= size_of::<usize>() {
            hash.add_to_hash(read_usize(bytes) as usize);
            bytes = &bytes[size_of::<usize>()..];
        }
        if (size_of::<usize>() > 4) && (bytes.len() >= 4) {
            hash.add_to_hash(u32::from_ne_bytes(bytes[..4].try_into().unwrap()) as usize);
            bytes = &bytes[4..];
        }
        if (size_of::<usize>() > 2) && bytes.len() >= 2 {
            hash.add_to_hash(u16::from_ne_bytes(bytes[..2].try_into().unwrap()) as usize);
            bytes = &bytes[2..];
        }
        if (size_of::<usize>() > 1) && !bytes.is_empty() {
            hash.add_to_hash(bytes[0] as usize);
        }
        self.hash = hash.hash;
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.add_to_hash(i as usize);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.add_to_hash(i as usize);
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.add_to_hash(i as usize);
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.add_to_hash(i as usize);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.add_to_hash(i);
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.hash as u64
    }
}
}
}
pub mod io {
pub mod input {
use std::fmt::Debug;
use std::io::Read;
use std::mem::MaybeUninit;
use std::str::FromStr;

pub struct Input<'s> {
    input: &'s mut (dyn Read + Send),
    buf: Vec<u8>,
    at: usize,
    buf_read: usize,
}

macro_rules! read_impl {
    ($t: ty, $read_name: ident, $read_vec_name: ident) => {
        pub fn $read_name(&mut self) -> $t {
            self.read()
        }

        pub fn $read_vec_name(&mut self, len: usize) -> Vec<$t> {
            self.read_vec(len)
        }
    };

    ($t: ty, $read_name: ident, $read_vec_name: ident, $read_pair_vec_name: ident) => {
        read_impl!($t, $read_name, $read_vec_name);

        pub fn $read_pair_vec_name(&mut self, len: usize) -> Vec<($t, $t)> {
            self.read_vec(len)
        }
    };
}

impl<'s> Input<'s> {
    const DEFAULT_BUF_SIZE: usize = 4096;

    pub fn new(input: &'s mut (dyn Read + Send)) -> Self {
        Self {
            input,
            buf: vec![0; Self::DEFAULT_BUF_SIZE],
            at: 0,
            buf_read: 0,
        }
    }

    pub fn new_with_size(input: &'s mut (dyn Read + Send), buf_size: usize) -> Self {
        Self {
            input,
            buf: vec![0; buf_size],
            at: 0,
            buf_read: 0,
        }
    }

    pub fn get(&mut self) -> Option<u8> {
        if self.refill_buffer() {
            let res = self.buf[self.at];
            self.at += 1;
            if res == b'\r' {
                if self.refill_buffer() && self.buf[self.at] == b'\n' {
                    self.at += 1;
                }
                return Some(b'\n');
            }
            Some(res)
        } else {
            None
        }
    }

    pub fn peek(&mut self) -> Option<u8> {
        if self.refill_buffer() {
            let res = self.buf[self.at];
            Some(if res == b'\r' { b'\n' } else { res })
        } else {
            None
        }
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(b) = self.peek() {
            if !char::from(b).is_whitespace() {
                return;
            }
            self.get();
        }
    }

    pub fn next_token(&mut self) -> Option<Vec<u8>> {
        self.skip_whitespace();
        let mut res = Vec::new();
        while let Some(c) = self.get() {
            if char::from(c).is_whitespace() {
                break;
            }
            res.push(c);
        }
        if res.is_empty() {
            None
        } else {
            Some(res)
        }
    }

    //noinspection RsSelfConvention
    pub fn is_exhausted(&mut self) -> bool {
        self.peek().is_none()
    }

    //noinspection RsSelfConvention
    pub fn is_empty(&mut self) -> bool {
        self.skip_whitespace();
        self.is_exhausted()
    }

    pub fn read<T: Readable>(&mut self) -> T {
        T::read(self)
    }

    pub fn read_vec<T: Readable>(&mut self, size: usize) -> Vec<T> {
        let mut res = Vec::with_capacity(size);
        for _ in 0..size {
            res.push(self.read());
        }
        res
    }

    pub fn read_string(&mut self) -> String {
        match self.next_token() {
            None => {
                panic!("Input exhausted");
            }
            Some(res) => unsafe { String::from_utf8_unchecked(res) },
        }
    }

    pub fn read_line(&mut self) -> String {
        let mut res = String::new();
        while let Some(c) = self.get() {
            if c == b'\n' {
                break;
            }
            if c == b'\r' {
                if self.peek() == Some(b'\n') {
                    self.get();
                }
                break;
            }
            res.push(c.into());
        }
        res
    }

    fn read_integer<T: FromStr>(&mut self) -> T
    where
        <T as FromStr>::Err: Debug,
    {
        let res = self.read_string();
        res.parse::<T>().unwrap()
    }

    pub fn read_byte(&mut self) -> u8 {
        self.skip_whitespace();
        self.get().unwrap()
    }

    pub fn read_char(&mut self) -> char {
        self.read_byte().into()
    }

    read_impl!(u32, read_unsigned, read_unsigned_vec);
    read_impl!(u64, read_u64, read_u64_vec);
    read_impl!(usize, read_size, read_size_vec, read_size_pair_vec);
    read_impl!(i32, read_int, read_int_vec, read_int_pair_vec);
    read_impl!(i64, read_long, read_long_vec, read_long_pair_vec);
    read_impl!(i128, read_i128, read_i128_vec);

    fn refill_buffer(&mut self) -> bool {
        if self.at == self.buf_read {
            self.at = 0;
            self.buf_read = self.input.read(&mut self.buf).unwrap();
            self.buf_read != 0
        } else {
            true
        }
    }
}

pub trait Readable {
    fn read(input: &mut Input) -> Self;
}

impl Readable for char {
    fn read(input: &mut Input) -> Self {
        input.read_char()
    }
}

impl<T: Readable> Readable for Vec<T> {
    fn read(input: &mut Input) -> Self {
        let size = input.read();
        input.read_vec(size)
    }
}

impl Readable for String {
    fn read(input: &mut Input) -> Self {
        input.read_string()
    }
}

impl<T: Readable, const N: usize> Readable for [T; N] {
    // fn read(input: &mut Input) -> Self {
    //     let mut data: [MaybeUninit<T>; N] = [const { MaybeUninit::uninit() }; N];
    //
    //     // Dropping a `MaybeUninit` does nothing, so if there is a panic during this loop,
    //     // we have a memory leak, but there is no memory safety issue.
    //     for elem in &mut data[..] {
    //         elem.write(input.read());
    //     }
    //
    //     // Everything is initialized. Transmute the array to the
    //     // initialized type.
    //     unsafe { std::mem::transmute::<_, [T; N]>(data) }
    // }

    fn read(input: &mut Input) -> Self {
        let mut array: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        // let mut array: [MaybeUninit<T>; N] = [const { MaybeUninit::uninit() }; N];
        for elem in array.iter_mut() {
            elem.write(input.read());
        }

        // Is this zero cost?
        // Transmuting doesn't seem to work with const generics?
        array.map(|x| unsafe { x.assume_init() })
    }
}

macro_rules! read_integer {
    ($($t:ident)+) => {$(
        impl Readable for $t {
            fn read(input: &mut Input) -> Self {
                input.read_integer()
            }
        }
    )+};
}

read_integer!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

macro_rules! tuple_readable {
    ($($name:ident)+) => {
        impl<$($name: Readable), +> Readable for ($($name,)+) {
            fn read(input: &mut Input) -> Self {
                ($($name::read(input),)+)
            }
        }
    }
}

tuple_readable! {T}
tuple_readable! {T U}
tuple_readable! {T U V}
tuple_readable! {T U V X}
tuple_readable! {T U V X Y}
tuple_readable! {T U V X Y Z}
tuple_readable! {T U V X Y Z A}
tuple_readable! {T U V X Y Z A B}
tuple_readable! {T U V X Y Z A B C}
tuple_readable! {T U V X Y Z A B C D}
tuple_readable! {T U V X Y Z A B C D E}
tuple_readable! {T U V X Y Z A B C D E F}
}
pub mod output {
use std::io::Write;

pub struct Output<'s> {
    output: &'s mut dyn Write,
    buf: Vec<u8>,
    at: usize,
    auto_flush: bool,
    precision: Option<usize>,
}

impl<'s> Output<'s> {
    const DEFAULT_BUF_SIZE: usize = 4096;

    pub fn new(output: &'s mut dyn Write) -> Self {
        Self {
            output,
            buf: vec![0; Self::DEFAULT_BUF_SIZE],
            at: 0,
            auto_flush: false,
            precision: None,
        }
    }

    pub fn new_with_auto_flush(output: &'s mut dyn Write) -> Self {
        Self {
            output,
            buf: vec![0; Self::DEFAULT_BUF_SIZE],
            at: 0,
            auto_flush: true,
            precision: None,
        }
    }

    pub fn flush(&mut self) {
        if self.at != 0 {
            self.output.write_all(&self.buf[..self.at]).unwrap();
            self.output.flush().unwrap();
            self.at = 0;
            self.output.flush().expect("Couldn't flush output");
        }
    }

    pub fn print<T: Writable>(&mut self, s: T) {
        s.write(self);
    }

    pub fn print_line<T: Writable>(&mut self, s: T) {
        self.print(s);
        self.put(b'\n');
    }

    pub fn put(&mut self, b: u8) {
        self.buf[self.at] = b;
        self.at += 1;
        if self.at == self.buf.len() {
            self.flush();
        }
    }

    pub fn maybe_flush(&mut self) {
        if self.auto_flush {
            self.flush();
        }
    }

    pub fn print_per_line<T: Writable>(&mut self, arg: &[T]) {
        for i in arg {
            i.write(self);
            self.put(b'\n');
        }
    }

    pub fn print_iter<T: Writable, I: Iterator<Item = T>>(&mut self, iter: I) {
        let mut first = true;
        for e in iter {
            if first {
                first = false;
            } else {
                self.put(b' ');
            }
            e.write(self);
        }
    }

    pub fn print_iter_ref<'a, T: 'a + Writable, I: Iterator<Item = &'a T>>(&mut self, iter: I) {
        let mut first = true;
        for e in iter {
            if first {
                first = false;
            } else {
                self.put(b' ');
            }
            e.write(self);
        }
    }

    pub fn set_precision(&mut self, precision: usize) {
        self.precision = Some(precision);
    }
    pub fn reset_precision(&mut self) {
        self.precision = None;
    }
    pub fn get_precision(&self) -> Option<usize> {
        self.precision
    }
}

impl Write for Output<'_> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut start = 0usize;
        let mut rem = buf.len();
        while rem > 0 {
            let len = (self.buf.len() - self.at).min(rem);
            self.buf[self.at..self.at + len].copy_from_slice(&buf[start..start + len]);
            self.at += len;
            if self.at == self.buf.len() {
                self.flush();
            }
            start += len;
            rem -= len;
        }
        self.maybe_flush();
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.flush();
        Ok(())
    }
}

pub trait Writable {
    fn write(&self, output: &mut Output);
}

impl Writable for &str {
    fn write(&self, output: &mut Output) {
        output.write_all(self.as_bytes()).unwrap();
    }
}

impl Writable for String {
    fn write(&self, output: &mut Output) {
        output.write_all(self.as_bytes()).unwrap();
    }
}

impl Writable for char {
    fn write(&self, output: &mut Output) {
        output.put(*self as u8);
    }
}

impl<T: Writable> Writable for [T] {
    fn write(&self, output: &mut Output) {
        output.print_iter_ref(self.iter());
    }
}

impl<T: Writable, const N: usize> Writable for [T; N] {
    fn write(&self, output: &mut Output) {
        output.print_iter_ref(self.iter());
    }
}

impl<T: Writable> Writable for &T {
    fn write(&self, output: &mut Output) {
        T::write(self, output)
    }
}

impl<T: Writable> Writable for Vec<T> {
    fn write(&self, output: &mut Output) {
        self.as_slice().write(output);
    }
}

macro_rules! write_to_string {
    ($($t:ident)+) => {$(
        impl Writable for $t {
            fn write(&self, output: &mut Output) {
                self.to_string().write(output);
            }
        }
    )+};
}

write_to_string!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize);

macro_rules! tuple_writable {
    ($name0:ident $($name:ident: $id:tt )*) => {
        impl<$name0: Writable, $($name: Writable,)*> Writable for ($name0, $($name,)*) {
            fn write(&self, out: &mut Output) {
                self.0.write(out);
                $(
                out.put(b' ');
                self.$id.write(out);
                )*
            }
        }
    }
}

tuple_writable! {T}
tuple_writable! {T U:1}
tuple_writable! {T U:1 V:2}
tuple_writable! {T U:1 V:2 X:3}
tuple_writable! {T U:1 V:2 X:3 Y:4}
tuple_writable! {T U:1 V:2 X:3 Y:4 Z:5}
tuple_writable! {T U:1 V:2 X:3 Y:4 Z:5 A:6}

impl<T: Writable> Writable for Option<T> {
    fn write(&self, output: &mut Output) {
        match self {
            None => (-1).write(output),
            Some(t) => t.write(output),
        }
    }
}
}
}
pub mod misc {
pub mod dbg_macro {
#[macro_export]
#[allow(unused_macros)]
macro_rules! dbg {
    ($first_val:expr, $($val:expr),+ $(,)?) => {
        if cfg!(debug_assertions) {
            eprint!("[{}:{}] {} = {:?}",
                        file!(), line!(), stringify!($first_val), &$first_val);
            ($(eprint!(", {} = {:?}", stringify!($val), &$val)),+,);
            eprintln!();
        }
    };
    ($first_val:expr) => {
        if cfg!(debug_assertions) {
            eprintln!("[{}:{}] {} = {:?}",
                        file!(), line!(), stringify!($first_val), &$first_val)
        }
    };
}
}
pub mod test_type {
pub enum TestType {
    Single,
    MultiNumber,
    MultiEof,
}

pub enum TaskType {
    Classic,
    Interactive,
}
}
}
}
fn main() {
let mut sin = std::io::stdin();
let input = algo_lib::io::input::Input::new(&mut sin);
let mut stdout = std::io::stdout();
let output = algo_lib::io::output::Output::new(&mut stdout);
    crate::solution::run(input, output);
}
