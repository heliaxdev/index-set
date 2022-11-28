use std::any::type_name;
use std::collections::{BTreeSet, HashSet};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use index_set::btree::BTreeIndexSet;
use index_set::vec::VecIndexSet;

trait Set {
    fn new() -> Self
    where
        Self: Sized;
    fn op_insert(&mut self, index: usize);
    fn op_remove(&mut self, index: usize);
    fn op_contains(&mut self, index: usize) -> bool;
}

impl Set for HashSet<usize> {
    fn new() -> Self {
        Self::new()
    }

    fn op_insert(&mut self, index: usize) {
        self.insert(index);
    }

    fn op_remove(&mut self, index: usize) {
        self.remove(&index);
    }

    fn op_contains(&mut self, index: usize) -> bool {
        self.contains(&index)
    }
}

impl Set for BTreeSet<usize> {
    fn new() -> Self {
        Self::new()
    }

    fn op_insert(&mut self, index: usize) {
        self.insert(index);
    }

    fn op_remove(&mut self, index: usize) {
        self.remove(&index);
    }

    fn op_contains(&mut self, index: usize) -> bool {
        self.contains(&index)
    }
}

impl Set for BTreeIndexSet<u64> {
    fn new() -> Self {
        BTreeIndexSet::<u64>::default()
    }

    fn op_insert(&mut self, index: usize) {
        self.insert(index);
    }

    fn op_remove(&mut self, index: usize) {
        self.remove(index);
    }

    fn op_contains(&mut self, index: usize) -> bool {
        self.contains(index)
    }
}

impl Set for BTreeIndexSet<u128> {
    fn new() -> Self {
        BTreeIndexSet::<u128>::default()
    }

    fn op_insert(&mut self, index: usize) {
        self.insert(index);
    }

    fn op_remove(&mut self, index: usize) {
        self.remove(index);
    }

    fn op_contains(&mut self, index: usize) -> bool {
        self.contains(index)
    }
}

impl Set for VecIndexSet<u64> {
    fn new() -> Self {
        VecIndexSet::<u64>::default()
    }

    fn op_insert(&mut self, index: usize) {
        self.insert(index);
    }

    fn op_remove(&mut self, index: usize) {
        self.remove(index);
    }

    fn op_contains(&mut self, index: usize) -> bool {
        self.contains(index)
    }
}

impl Set for VecIndexSet<u128> {
    fn new() -> Self {
        VecIndexSet::<u128>::default()
    }

    fn op_insert(&mut self, index: usize) {
        self.insert(index);
    }

    fn op_remove(&mut self, index: usize) {
        self.remove(index);
    }

    fn op_contains(&mut self, index: usize) -> bool {
        self.contains(index)
    }
}

fn bench<S, F, R>(c: &mut Criterion, op_name: &str, mut op: F)
where
    S: Set,
    F: FnMut(&mut S, usize) -> R,
{
    let bench_id = format!("{op_name} - {}", type_name::<S>());
    c.bench_function(&bench_id, |b| {
        let mut set = S::new();
        b.iter(|| {
            for i in 0..1000 {
                black_box(op(&mut set, i));
            }
        });
    });
}

fn bench_set_insert<S: Set>(c: &mut Criterion) {
    bench::<S, _, _>(c, "insert", Set::op_insert);
}

fn bench_set_remove<S: Set>(c: &mut Criterion) {
    bench::<S, _, _>(c, "remove", Set::op_remove);
}

fn bench_set_contains<S: Set>(c: &mut Criterion) {
    bench::<S, _, _>(c, "contains", Set::op_contains);
}

criterion_group!(
    benches,
    bench_set_insert::<HashSet<usize>>,
    bench_set_insert::<BTreeSet<usize>>,
    bench_set_insert::<BTreeIndexSet<u64>>,
    bench_set_insert::<BTreeIndexSet<u128>>,
    bench_set_insert::<VecIndexSet<u64>>,
    bench_set_insert::<VecIndexSet<u128>>,
    bench_set_remove::<HashSet<usize>>,
    bench_set_remove::<BTreeSet<usize>>,
    bench_set_remove::<BTreeIndexSet<u64>>,
    bench_set_remove::<BTreeIndexSet<u128>>,
    bench_set_remove::<VecIndexSet<u64>>,
    bench_set_remove::<VecIndexSet<u128>>,
    bench_set_contains::<HashSet<usize>>,
    bench_set_contains::<BTreeSet<usize>>,
    bench_set_contains::<BTreeIndexSet<u64>>,
    bench_set_contains::<BTreeIndexSet<u128>>,
    bench_set_contains::<VecIndexSet<u64>>,
    bench_set_contains::<VecIndexSet<u128>>,
);
criterion_main!(benches);
