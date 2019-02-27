//! This module provides crud for all our models, as well as populating fake data into each
//! table. It also contains the module for creating our pg pool.

/// Categories CRUD and random data populating functions.
pub mod categories;
/// Choices CRUD and random data populating functions.
pub mod choices;
/// Fences CRUD and random data populating functions.
pub mod fences;
/// Has one important public function, generate_pool, that returns a pg pool for use.
pub mod pg_pool;
/// Questions CRUD and random data populating functions.
pub mod questions;
/// Surveys CRUD and random data populating functions.
pub mod surveys;
/// Users CRUD and random data populating functions.
pub mod users;
/// Votes CRUD and random data populating functions.
pub mod votes;
