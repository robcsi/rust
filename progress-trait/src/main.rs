// https://www.youtube.com/watch?v=bnnacleqg6k

use std::{time::Duration, thread::sleep};

const CLEAR: &str = "\x1B[2J\x1b[1;1H";

// type statte - encode the state of the progress bar in the type system
struct Unbounded;
struct Bounded {
    bound: usize,
    delims: (char, char)
}

struct Progress<Iter, Bound> {
    iter: Iter,
    i: usize,
    bound: Bound
}

trait ProgressDisplay: Sized {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>);
}

impl ProgressDisplay for Unbounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}", "\u{2588}".repeat(progress.i));
    }
}

impl ProgressDisplay for Bounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {

        let percentage = (progress.i * 100) / self.bound;

        println!("{:3}% {}{}{}{}", percentage, self.delims.0, "\u{2588}".repeat(progress.i), " ".repeat(self.bound - progress.i), self.delims.1);
    }
}

impl<Iter> Progress<Iter, Unbounded> {
    pub fn new(iter: Iter) -> Self {
        Progress { iter, i: 0, bound: Unbounded }
    }
}

impl<Iter> Progress<Iter, Unbounded>
where Iter: ExactSizeIterator { // for when Iter is an ExactSizeIterator
    pub fn with_bound(self) -> Progress<Iter, Bounded> { // add with_bound method
        let bound = Bounded { bound: self.iter.len(), delims: ('[', ']') };
        Progress { i: self.i, iter: self.iter, bound }
    }
}

impl<Iter> Progress<Iter, Bounded> {
    pub fn with_delims(mut self, delims: (char, char)) -> Self {
        self.bound.delims = delims;
        self
    }
}

impl<Iter, Bound> Iterator for Progress<Iter, Bound>  // the Progress struct is now an Iterator, can be used in a for loop
where Iter: Iterator, Bound: ProgressDisplay {
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{}", CLEAR);
        self.bound.display(&self);
        self.i += 1;

        self.iter.next()
    }
}

trait ProgressIteratorExt: Sized { // trait with progress functionality
    fn progress(self) -> Progress<Self, Unbounded>;
}

impl<Iter> ProgressIteratorExt for Iter
// where Iter: Iterator // limit only to Iterator type
{ // implement trait above for all types Iter, for that quantified type
    fn progress(self) -> Progress<Self, Unbounded> {
        Progress::new(self)
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let brackets = ('[', ']');

    // let x = 1.progress();
    let v: Vec<i32> = (1..=10).collect();
    for n in v.iter().progress().with_bound().with_delims(brackets) { // trait ExactSizeIterator: Iterator - an interator that knows its exact length
        expensive_calculation(n);
    }

    // for n in (0 .. ).progress()
    //     //.with_bound().with_delims(brackets) // does not compile because of unsatisfied trait bounds
    // {
    //     expensive_calculation(&n);
    // }
}
