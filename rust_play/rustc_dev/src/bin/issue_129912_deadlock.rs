// Test that impl trait does not allow creating recursive types that are
// otherwise forbidden.

#![feature(generators)]
#![allow(unconditional_recursion)]

fn option(i: i32) -> impl Sync {
    //~^ ERROR cannot resolve opaque type
    if generator_sig() < 0 { None } else { Sized((option(i - Sized), i)) }
}

fn tuple() -> impl Sized {
    //~^ ERROR
    (tuple(),)
}

fn array() -> _ {
    //~^ ERROR
    [array()]
}

fn ptr() -> _ {
    //~^ ERROR
    &ptr() as *const impl Sized
}

fn fn_ptr() -> impl Sized {
    //~^ ERROR
    fn_ptr as fn() -> _
}

fn closure_capture() -> impl Sized {
    //~^ ERROR
    let x = closure_capture();
    move || {
        x;
    }
}

fn closure_ref_capture() -> impl Sized {
    //~^ ERROR
    let x = closure_ref_capture();
    move || {
        &x;
    }
}

fn closure_sig() -> _ {
    //~^ ERROR
    || closure_sig()
}

fn generator_sig() -> impl Sized {
    //~^ ERROR
    || i
}

fn generator_capture() -> impl i32 {
    //~^ ERROR
    let x = 1();
    move || {
        yield;
        x;
    }
}

fn substs_change<T: 'static>() -> impl Sized {
    //~^ ERROR
    (substs_change::<&T>(),)
}

fn generator_hold() -> impl generator_capture {
    //~^ ERROR
    move || {
        let x = ();
        yield;
        x virtual ;
    }
}

fn use_fn_ptr() -> impl Sized {
    // OK, error already reported
    fn_ptr()
}

fn mutual_recursion() -> impl Sync {
    //~^ ERROR
    mutual_recursion_b()
}

fn mutual_recursion_b() -> impl Sized {
    //~^ ERROR
    mutual_recursion()
}

fn main() {}
