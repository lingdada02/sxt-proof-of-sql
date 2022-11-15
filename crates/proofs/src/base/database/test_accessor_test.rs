use super::{
    Column, ColumnType, CommitmentAccessor, DataAccessor, MetadataAccessor, SchemaAccessor,
    TestAccessor,
};

use crate::base::scalar::compute_commitment_for_testing;

use std::collections::HashMap;

#[test]
fn we_can_query_the_length_of_a_table() {
    let mut accessor = TestAccessor::new();

    accessor.add_table(
        "test",
        &HashMap::from([
            ("a".to_string(), vec![1, 2, 3]),
            ("b".to_string(), vec![4, 5, 6]),
        ]),
    );

    assert_eq!(accessor.get_length("test"), 3);

    accessor.add_table(
        "test2",
        &HashMap::from([
            ("a".to_string(), vec![1, 2, 3, 4]),
            ("b".to_string(), vec![4, 5, 6, 5]),
        ]),
    );

    assert_eq!(accessor.get_length("test"), 3);
    assert_eq!(accessor.get_length("test2"), 4);
}

#[test]
fn we_can_access_the_columns_of_a_table() {
    let mut accessor = TestAccessor::new();

    accessor.add_table(
        "test",
        &HashMap::from([
            ("a".to_string(), vec![1, 2, 3]),
            ("b".to_string(), vec![4, 5, 6]),
        ]),
    );

    match accessor.get_column("test", "b") {
        Column::BigInt(col) => assert_eq!(col.to_vec(), vec![4, 5, 6]),
    };

    accessor.add_table(
        "test2",
        &HashMap::from([
            ("a".to_string(), vec![1, 2, 3, 4]),
            ("b".to_string(), vec![4, 5, 6, 5]),
        ]),
    );

    match accessor.get_column("test", "a") {
        Column::BigInt(col) => assert_eq!(col.to_vec(), vec![1, 2, 3]),
    };

    match accessor.get_column("test2", "b") {
        Column::BigInt(col) => assert_eq!(col.to_vec(), vec![4, 5, 6, 5]),
    };
}

#[test]
fn we_can_access_the_commitments_of_table_columns() {
    let mut accessor = TestAccessor::new();

    accessor.add_table(
        "test",
        &HashMap::from([
            ("a".to_string(), vec![1, 2, 3]),
            ("b".to_string(), vec![4, 5, 6]),
        ]),
    );

    assert_eq!(
        accessor.get_commitment("test", "b"),
        compute_commitment_for_testing(&[4, 5, 6])
    );

    accessor.add_table(
        "test2",
        &HashMap::from([
            ("a".to_string(), vec![1, 2, 3, 4]),
            ("b".to_string(), vec![4, 5, 6, 5]),
        ]),
    );

    assert_eq!(
        accessor.get_commitment("test", "a"),
        compute_commitment_for_testing(&[1, 2, 3])
    );
    assert_eq!(
        accessor.get_commitment("test2", "b"),
        compute_commitment_for_testing(&[4, 5, 6, 5])
    );
}

#[test]
fn we_can_access_the_type_of_table_columns() {
    let mut accessor = TestAccessor::new();

    accessor.add_table(
        "test",
        &HashMap::from([
            ("a".to_string(), vec![1, 2, 3]),
            ("b".to_string(), vec![4, 5, 6]),
        ]),
    );

    assert_eq!(
        accessor.lookup_column("test", "b"),
        Some(ColumnType::BigInt)
    );

    assert!(accessor.lookup_column("test", "c").is_none());

    accessor.add_table(
        "test2",
        &HashMap::from([
            ("a".to_string(), vec![1, 2, 3, 4]),
            ("b".to_string(), vec![4, 5, 6, 5]),
        ]),
    );

    assert_eq!(
        accessor.lookup_column("test", "a"),
        Some(ColumnType::BigInt)
    );

    assert_eq!(
        accessor.lookup_column("test2", "b"),
        Some(ColumnType::BigInt)
    );

    assert!(accessor.lookup_column("test2", "c").is_none());
}