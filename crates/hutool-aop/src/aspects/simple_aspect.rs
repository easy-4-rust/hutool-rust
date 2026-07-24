//! Hutool-aligned before/after aspects.

use crate::Method;
use parking_lot::Mutex;
use std::{
    any::type_name,
    collections::HashMap,
    fmt,
    sync::Arc,
    thread::{self, ThreadId},
    time::{Duration, Instant},
};

use super::aspect::Aspect;

/// An aspect whose three callbacks all allow normal processing.
#[derive(Debug, Default, Clone, Copy)]
pub struct SimpleAspect;

impl<T, A, R, E> Aspect<T, A, R, E> for SimpleAspect {}
