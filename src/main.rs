use std::ffi::{c_int, c_short};

#[repr(C)]
struct Pair {
    a: c_int,
    b: c_int,
}

#[repr(C)]
#[derive(Debug)]
struct Return4 {
    a: c_int,
    b: c_int,
    c: c_int,
    d: c_int,
}

#[repr(C)]
#[derive(Debug)]
struct Return5 {
    a: c_int,
    b: c_int,
    c: c_int,
    d: c_int,
    e: c_int,
}

extern "C" {
    fn abi_test_5(p1: c_int, p2: c_int, p3: c_int, p4: c_int, p5: Pair);
    fn abi_test_6(p1: c_int, p2: c_int, p3: c_int, p4: c_int, p5: c_int, p6: Pair);
    fn abi_test_7(p1: c_int, p2: c_int, p3: c_int, p4: c_int, p5: c_int, p6: c_int, p7: Pair);
    fn abi_test_8(p1: c_int, p2: c_int, p3: c_int, p4: c_int, p5: c_int, p6: c_int, p7: c_int, p8: Pair);
    fn abi_test_18(p1: c_int, p2: c_int, p3: c_int, p4: c_int, p5: c_int, p6: c_int, p7: c_int, p8: c_int,p9: c_int,p10: c_int,p11: c_int,p12: c_int,p13: c_int,p14: c_int,p15: c_int,p16: c_int,p17: c_int, p18: Pair);

    fn ret_abi_test_internal_4();
    fn ret_abi_test_internal_5();

    fn ret_abi_test_4() -> Return4;
    fn ret_abi_test_5() -> Return5;

    fn double_demo();

    fn abi_test_double(d: f64) -> f64;
    fn abi_test_float(f: f32)-> f32;

    fn internal_clang_test();

    fn test_parameters_issue_171(p1: c_short, p2: c_short, p3: c_short, p4: c_short, p5: c_short, p6: c_short, p7: c_short, p8: c_short, p9: c_short, p10: c_short) -> c_short;
}

fn main() {
    unsafe {
        abi_test_5(0, 0, 0, 0, Pair { a: 1, b: 2 });
        abi_test_6(0, 0, 0, 0, 0, Pair { a: 1, b: 2 });
        abi_test_7(0, 0, 0, 0, 0, 0, Pair { a: 1, b: 2 });
        abi_test_8(0, 0, 0, 0, 0, 0, 0, Pair { a: 1, b: 2 });
        abi_test_18(0, 0, 0, 0, 0, 0, 0, 0,0,0,0,0,0,0,0,0,0, Pair { a: 1, b: 2 });

        ret_abi_test_internal_4();
        ret_abi_test_internal_5();

        println!("ret_abi_test_4: {:?}", ret_abi_test_4());
        println!("ret_abi_test_5: {:?}", ret_abi_test_5());
        println!("ret_abi_test_5: {:?}", ret_abi_test_5());

        double_demo();

        println!("{}", abi_test_double(3.14f64));
        println!("{}", abi_test_float(3.14f32));

        println!("{}", (abi_test_double(3.14f64) * 2.0) / 3.0);
        println!("{}", (abi_test_float(3.14f32) * 2.0) / 3.0);

        println!("{}", (3.14f64 * 2.0) / 3.0);
        println!("{}", (3.14f32 * 2.0) / 3.0);

        internal_clang_test();

        println!("align_of c_short: {}", core::mem::align_of::<c_short>());
        println!("size_of of c_short: {}", core::mem::size_of::<c_short>());
        println!("Issue 171, result should be 800, it is {}", test_parameters_issue_171(100,200,300,400,500,600,700,800,900,1000));
    }
}


// not sure why this needed, binstart feature is passed?
#[export_name = "app_main"]
extern "C" fn dummy() {
    main()
}