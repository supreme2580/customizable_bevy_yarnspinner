//! Thin newtypes around existing collections to better express their intent in regards to the corresponding dotnet types.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, fmt::Debug};

/// Represents a FIFO (First-In, First-Out) collection.
///
/// Models the behaviour of <https://learn.microsoft.com/en-us/dotnet/api/system.collections.generic.queue-1>
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Queue<T: Debug + Clone>(pub VecDeque<T>);

impl<T: Debug + Clone> Queue<T> {
    pub fn enqueue(&mut self, value: T) {
        self.0.push_back(value)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.0.pop_front()
    }
}

/// Represents a FILO (First-In, Last-Out) collection.
///
/// Models the behaviour of <https://learn.microsoft.com/en-us/dotnet/api/system.collections.generic.stack-1>
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Stack<T: Debug + Clone>(pub Vec<T>);

impl<T: Debug + Clone> Stack<T> {
    pub fn push(&mut self, value: T) {
        self.0.push(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.0.last()
    }
}

// Somehow auto-derive does not work with those types so a manual impl is needed?

impl<T> Default for Queue<T>
where
    T: 'static + Send + Sync + Debug + Clone,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T> Default for Stack<T>
where
    T: 'static + Send + Sync + Debug + Clone,
{
    fn default() -> Self {
        Self(Default::default())
    }
}
