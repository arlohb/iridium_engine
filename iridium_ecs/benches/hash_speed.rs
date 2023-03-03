//! Tests between hashing the type id or the type name.
//!
//! This is justify the use of calulating `StableTypeId` at compile time,
//! instead of just hashing the type name.

#![allow(missing_docs)]
#![allow(clippy::inline_always)]

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iridium_ecs_macros::HasStableTypeId;
use iridium_reflect::HasStableTypeId;

#[derive(HasStableTypeId)]
struct Test;

fn calc_hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

#[inline(always)]
fn hash_type_id() {
    calc_hash(&std::any::TypeId::of::<Test>());
}

#[inline(always)]
fn hash_type_name() {
    calc_hash(&std::any::type_name::<Test>());
}

#[inline(always)]
fn pre_hash_type_name(pre_hashed: u64) {
    calc_hash(&pre_hashed);
}

#[inline(always)]
fn const_hash_type_name() {
    calc_hash(&Test::stable_type_id());
}

fn criterion_benchmark(c: &mut Criterion) {
    let hashed_type_name = calc_hash(&std::any::type_name::<Test>());

    let mut group = c.benchmark_group("Hash comparisons");

    group.bench_function("TypeId", |b| b.iter(black_box(hash_type_id)));
    group.bench_function("TypeName", |b| b.iter(black_box(hash_type_name)));
    group.bench_function("PreHashedTypeName", |b| {
        b.iter(black_box(|| pre_hash_type_name(hashed_type_name)));
    });
    group.bench_function("ConstHashedTypeName", |b| {
        b.iter(black_box(const_hash_type_name));
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
