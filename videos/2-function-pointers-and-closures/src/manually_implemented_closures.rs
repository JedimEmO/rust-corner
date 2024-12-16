fn use_mut_closure(mut f: impl FnMut(&str)) {
    f("Hello");
}

fn use_closure(f: impl Fn(&str)) {
    f("Hello");
}

fn use_closure_once(f: impl FnOnce(&str)) {
    f("Goodbye");
}

pub fn manual_closures_demo() {
    let name = "Mathias".to_string();

    struct NonAnonymousClosureStruct<'a> {
        name: &'a String,
    }

    impl<'a> FnOnce<(&str,)> for NonAnonymousClosureStruct<'a> {
        type Output = ();

        extern "rust-call" fn call_once(self, args: (&str,)) -> Self::Output {
            println!("Final {} to {}", args.0, self.name);
        }
    }

    impl<'a> FnMut<(&str,)> for NonAnonymousClosureStruct<'a> {
        extern "rust-call" fn call_mut(&mut self, args: (&str,)) -> Self::Output {
            println!("Mutably {} to {}", args.0, self.name);
        }
    }

    impl<'a> Fn<(&str,)> for NonAnonymousClosureStruct<'a> {
        extern "rust-call" fn call(&self, args: (&str,)) -> Self::Output {
            println!("Immutably {} to {}", args.0, self.name);
        }
    }

    use_mut_closure(NonAnonymousClosureStruct { name: &name });
    use_closure(NonAnonymousClosureStruct { name: &name });
    use_closure_once(NonAnonymousClosureStruct { name: &name });
}
