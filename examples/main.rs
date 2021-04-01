

struct Foo(String);

fn main() {
    {
        use sharedptr::Rc::SharedPtr;

        let mut ptr = SharedPtr::<Foo>::zeroed();
        assert_eq!(ptr.is_null(), true);

        ptr.write(Foo("122".to_string()));

        println!("{}", ptr.0);

        let mut ptr2 = ptr.clone();
        println!("{}", ptr2.0);

        ptr2.set_null();
        println!("{}", ptr.0);

        if ptr2.weak().is_none() {
            println!("ptr 2 weak fail")
        }
        let wk = ptr.weak().unwrap();
        println!("{}", wk.upgrade().unwrap().0);

        let rc = wk.upgrade().unwrap();

        let ptr2: SharedPtr<Foo> = rc.into();
        println!("{}", ptr2.0);

        drop(ptr);

        // if let Some(rc) = ptr.assume_init() {
        //     println!("{}", rc.0);
        // }

        if wk.upgrade().is_none() {
            println!("ptr is drop");
        }
    }
    {
        use sharedptr::Arc::SharedPtr;

        let mut ptr = SharedPtr::<Foo>::zeroed();
        assert_eq!(ptr.is_null(), true);

        ptr.write(Foo("122".to_string()));

        println!("{}", ptr.0);

        let mut ptr2 = ptr.clone();
        println!("{}", ptr2.0);

        ptr2.set_null();
        println!("{}", ptr.0);

        if ptr2.weak().is_none() {
            println!("ptr 2 weak fail")
        }
        let wk = ptr.weak().unwrap();
        println!("{}", wk.upgrade().unwrap().0);

        let rc = wk.upgrade().unwrap();

        let ptr2: SharedPtr<Foo> = rc.into();
        println!("{}", ptr2.0);

        drop(ptr);

        // if let Some(rc) = ptr.assume_init() {
        //     println!("{}", rc.0);
        // }

        if wk.upgrade().is_none() {
            println!("ptr is drop");
        }
    }
}