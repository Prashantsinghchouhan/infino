// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: Copyright The Infino Authors

//! Smoke test that Arrow types are reachable through the crate-root
//! re-exports (`infino::arrow_array`, `infino::arrow_schema`) without
//! depending on `arrow-array` or `arrow-schema` as direct imports.

use std::sync::Arc;

use infino::{
    arrow_array::{LargeStringArray, RecordBatch},
    arrow_schema::{DataType, Field, Schema},
};

#[test]
fn arrow_types_reexported_at_crate_root() {
    let schema = Arc::new(Schema::new(vec![Field::new(
        "x",
        DataType::LargeUtf8,
        false,
    )]));
    let batch = RecordBatch::try_new(schema, vec![Arc::new(LargeStringArray::from(vec!["ok"]))])
        .expect("valid batch");
    assert_eq!(batch.num_rows(), 1);
}
