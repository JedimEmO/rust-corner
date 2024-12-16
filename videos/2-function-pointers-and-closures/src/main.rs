#![feature(async_closure)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]

use crate::manually_implemented_closures::manual_closures_demo;

mod manually_implemented_closures;

fn main() {
    // Call the manual closure implementation demo
    manual_closures_demo();

    println!(
        r#"Mathias' rust corner part 2 - function pointers and closures

This video focuses on function pointers and closures.

If you want to test the ideas from the video in practice, you can try making the various test suites work!

To run them, use the following cargo commands:

cargo test --features "function-pointers"
cargo test --features "closures"
cargo test --features "async-closures"

Then simply implement the features needed for the tests to pass!
"#
    );
}

#[cfg(all(test, feature = "function-pointers"))]
mod test_function_pointers {
    fn transform_string(input: &str, transform_fn: fn(char) -> char) -> String {
        todo!()
    }

    #[test]
    fn ensure_transform() {
        // For this one, you should implement the transform string so that the following test case pass
        // hint: you can collect() strings from a bunch of chars!
        let uppercase_mathias = transform_string("mathias!", |c| {
            c.to_uppercase().to_string().chars().next().unwrap()
        });
        assert_eq!(uppercase_mathias, "MATHIAS!");
    }

    #[derive(Eq, PartialEq, Debug)]
    struct Context {
        some_value: u32,
    }

    fn context_id(ctx: Context) -> Context {
        ctx
    }

    fn compose_context_fns(
        input: Context,
        filter_into_fn: fn(Context) -> Option<Context>,
        double_fn: fn(Context) -> Context,
        add_fn: fn(Context) -> Context,
    ) -> Option<Context> {
        filter_into_fn(input).map(add_fn).map(double_fn)
    }

    fn process_context(ctx: Context) -> Option<Context> {
        // Hint: Maybe pass different functions here
        compose_context_fns(ctx, |_| None, context_id, context_id)
    }

    #[test]
    fn provide_some_functions() {
        // Provide function pointers by modifying the process_context function so that only contexts with even values are let through,
        // and so that the resulting context has the value of 14
        let out = process_context(Context { some_value: 2 });

        assert_eq!(out, Some(Context { some_value: 14 }));

        let out = process_context(Context { some_value: 3 });
        assert!(out.is_none());
    }
}

#[cfg(all(test, feature = "closures"))]
mod test_closures {
    use std::rc::Rc;
    use std::sync::Mutex;

    fn use_closure(f: impl Fn(&str) + 'static) {
        f("Bob");
    }

    #[test]
    fn statically() {
        let mut name = Rc::new(Mutex::new("Mathias".to_string()));

        // Make this test pass by passing a closure to the use_closure function!
        // hint: ownership is for ever
        // use_closure(???);

        assert_eq!(*name.lock().unwrap(), "Bob");
    }
}

#[cfg(all(test, feature = "async-closures"))]
mod test_async_closures {
    use std::future::{ready, Future};
    use std::ops::AsyncFn;

    async fn compute_sequence<T: AsyncFn(i32) -> String>(
        producer: impl AsyncFn(Box<dyn Future<Output = i32> + Unpin>) -> T,
    ) -> Vec<String> {
        let mut out = vec![];

        for i in 0..10 {
            let f = producer(Box::new(ready(i))).await;
            out.push(f(i).await);
        }

        out
    }

    #[tokio::test]
    async fn async_factory() {
        // Make this test pass by injecting an async closure.
        // hint: it's hip to be square
        let sequence = compute_sequence(async |f: Box<dyn Future<Output = i32> + Unpin>| {
            async |_| "-".to_string()
        })
        .await;

        assert_eq!(sequence, vec!["0", "1", "4", "9", "16", "25", "36", "49", "64", "81"])
    }
}
