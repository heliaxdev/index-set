use std::any::type_name;
use std::collections::{BTreeSet, HashSet};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use index_set::btree::BTreeIndexSet;
use index_set::vec::VecIndexSet;
use index_set::IndexSet;

#[derive(Copy, Clone)]
enum SetupFor {
    Insert,
    Remove,
    Contains,
}

impl SetupFor {
    fn name(&self) -> &'static str {
        match self {
            Self::Insert => "insert",
            Self::Remove => "remove",
            Self::Contains => "contains",
        }
    }
}

trait Set {
    fn setup(setup_for: SetupFor) -> Self
    where
        Self: Sized;
    fn op_insert(&mut self, index: usize);
    fn op_remove(&mut self, index: usize);
    fn op_contains(&mut self, index: usize) -> bool;
}

impl Set for HashSet<usize> {
    fn setup(setup_for: SetupFor) -> Self {
        match setup_for {
            SetupFor::Insert => Self::new(),
            SetupFor::Remove | SetupFor::Contains => {
                let mut set = Self::new();

                for i in 0..1000 {
                    set.insert(i);
                }

                set
            }
        }
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
    fn setup(setup_for: SetupFor) -> Self {
        match setup_for {
            SetupFor::Insert => Self::new(),
            SetupFor::Remove | SetupFor::Contains => {
                let mut set = Self::new();

                for i in 0..1000 {
                    set.insert(i);
                }

                set
            }
        }
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
    fn setup(setup_for: SetupFor) -> Self {
        match setup_for {
            SetupFor::Insert => Self::new(),
            SetupFor::Remove | SetupFor::Contains => {
                let mut set = Self::new();

                for i in 0..1000 {
                    set.insert(i);
                }

                set
            }
        }
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
    fn setup(setup_for: SetupFor) -> Self {
        match setup_for {
            SetupFor::Insert => Self::new(),
            SetupFor::Remove | SetupFor::Contains => {
                let mut set = Self::new();

                for i in 0..1000 {
                    set.insert(i);
                }

                set
            }
        }
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
    fn setup(setup_for: SetupFor) -> Self {
        match setup_for {
            SetupFor::Insert => Self::new(),
            SetupFor::Remove | SetupFor::Contains => {
                let mut set = Self::new();

                for i in 0..1000 {
                    set.insert(i);
                }

                set
            }
        }
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
    fn setup(setup_for: SetupFor) -> Self {
        match setup_for {
            SetupFor::Insert => Self::new(),
            SetupFor::Remove | SetupFor::Contains => {
                let mut set = Self::new();

                for i in 0..1000 {
                    set.insert(i);
                }

                set
            }
        }
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

fn bench<S, F, R>(c: &mut Criterion, setup_for: SetupFor, mut op: F)
where
    S: Set,
    F: FnMut(&mut S, usize) -> R,
{
    let bench_id = format!("{} - {}", setup_for.name(), type_name::<S>());
    c.bench_function(&bench_id, |b| {
        b.iter(|| {
            let mut set = S::setup(setup_for);

            for i in 0..1000 {
                black_box(op(&mut set, i));
            }
        });
    });
}

fn bench_set_insert<S: Set>(c: &mut Criterion) {
    bench::<S, _, _>(c, SetupFor::Insert, Set::op_insert);
}

fn bench_set_remove<S: Set>(c: &mut Criterion) {
    bench::<S, _, _>(c, SetupFor::Remove, Set::op_remove);
}

fn bench_set_contains<S: Set>(c: &mut Criterion) {
    bench::<S, _, _>(c, SetupFor::Contains, Set::op_contains);
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
