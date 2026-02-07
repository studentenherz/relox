use std::fmt::{Debug, Display};
use std::mem::MaybeUninit;

pub struct Stack<T, const C: usize> {
    data: [MaybeUninit<T>; C],
    size: usize,
    max_size: usize,
}

pub enum StackError {
    Undeflow,
    Overflow,
}

impl<T, const C: usize> Stack<T, C> {
    pub fn new() -> Self {
        Self {
            data: [const { MaybeUninit::uninit() }; C],
            size: 0,
            max_size: 0,
        }
    }

    pub fn push(&mut self, value: T) -> Result<(), StackError> {
        if self.size < C {
            if self.size < self.max_size {
                unsafe {
                    self.data[self.size].assume_init_drop();
                }
            }

            self.data[self.size].write(value);
            self.size += 1;

            self.max_size = self.max_size.max(self.size);

            Ok(())
        } else {
            Err(StackError::Overflow)
        }
    }

    pub fn pop(&mut self) -> Result<T, StackError> {
        if self.size > 0 {
            self.size -= 1;
            Ok(unsafe { std::ptr::read(&self.data[self.size]).assume_init() })
        } else {
            Err(StackError::Undeflow)
        }
    }

    pub fn top(&mut self) -> Result<&mut T, StackError> {
        if self.size > 0 {
            Ok(unsafe { self.data[self.size - 1].assume_init_mut() })
        } else {
            Err(StackError::Undeflow)
        }
    }
}

impl<T: Debug, const C: usize> Debug for Stack<T, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.size {
            write!(f, "[ {:?} ]", unsafe { self.data[i].assume_init_ref() })?;
        }

        Ok(())
    }
}

impl Display for StackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Overflow => write!(f, "Stack overflow"),
            Self::Undeflow => write!(f, "Stack underflow"),
        }
    }
}
