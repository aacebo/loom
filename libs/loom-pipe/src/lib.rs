pub mod operators;
pub mod pipeline;
mod source;
mod transformer;

pub use pipeline::*;
pub use source::*;
pub use transformer::*;
pub use operators::*;

pub trait Operator<Input> {
    type Output;

    fn apply(self, src: Source<Input>) -> Source<Self::Output>;
}

pub trait Pipe<Input> {
    fn pipe<Op: Operator<Input>>(self, op: Op) -> Source<Op::Output>;
}

pub trait Build {
    type Output;

    fn build(self) -> Self::Output;
}
