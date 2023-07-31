use zhl_algorithm_template_rs::random::generator::{IntGenerator, MT19937_64, Pseudorandom64};
fn main() {
    let mut gen = IntGenerator::new(&MT19937_64::new(None));
    for i in 0..100000000 {
        let x = gen.gen_range_u8(100u8..200u8);
        assert!(x >= 100 && x < 200);
        let x = gen.gen_range_u16(10000u16..20000u16);
        assert!(x >= 10000 && x < 20000);
        let x = gen.gen_range_u32(1000000u32..2000000u32);
        assert!(x >= 1000000 && x < 2000000);
        let x = gen.gen_range_u64(100000000u64..200000000u64);
        assert!(x >= 100000000 && x < 200000000);
        let x = gen.gen_range_u128(10000000000u128..20000000000u128);
        assert!(x >= 10000000000 && x < 20000000000);
        let x = gen.gen_range_i8(-100i8..100i8);
        assert!(x >= -100 && x < 100);
        let x = gen.gen_range_i16(-10000i16..10000i16);
        assert!(x >= -10000 && x < 10000);
        let x = gen.gen_range_i32(-1000000i32..1000000i32);
        assert!(x >= -1000000 && x < 1000000);
        let x = gen.gen_range_i64(-100000000i64..100000000i64);
        assert!(x >= -100000000 && x < 100000000);
        let x = gen.gen_range_i128(-10000000000i128..10000000000i128);
        assert!(x >= -10000000000 && x < 10000000000);
        let x = gen.gen_range_usize(100usize..200usize);   
        assert!(x >= 100 && x < 200);
    }

    println!("gen_range test passed!");

    println!("Gen a u8: {}", gen.gen_u8());
    println!("Gen a u16: {}", gen.gen_u16());
    println!("Gen a u32: {}", gen.gen_u32());
    println!("Gen a u64: {}", gen.gen_u64());
    println!("Gen a u128: {}", gen.gen_u128());
    println!("Gen a i8: {}", gen.gen_i8());
    println!("Gen a i16: {}", gen.gen_i16());
    println!("Gen a i32: {}", gen.gen_i32());
    println!("Gen a i64: {}", gen.gen_i64());
    println!("Gen a i128: {}", gen.gen_i128());
    println!("Gen a usize: {}", gen.gen_usize());

    println!("Gen a u8 in range 100u8..200u8: {}", gen.gen_range_u8(100u8..200u8));
    println!("Gen a u16 in range 10000u16..20000u16: {}", gen.gen_range_u16(10000u16..20000u16));
    println!("Gen a u32 in range 1000000u32..2000000u32: {}", gen.gen_range_u32(1000000u32..2000000u32));
    println!("Gen a u64 in range 100000000u64..200000000u64: {}", gen.gen_range_u64(100000000u64..200000000u64));
    println!("Gen a u128 in range 10000000000u128..20000000000u128: {}", gen.gen_range_u128(10000000000u128..20000000000u128));
    println!("Gen a i8 in range -100i8..100i8: {}", gen.gen_range_i8(-100i8..100i8));
    println!("Gen a i16 in range -10000i16..10000i16: {}", gen.gen_range_i16(-10000i16..10000i16));
    println!("Gen a i32 in range -1000000i32..1000000i32: {}", gen.gen_range_i32(-1000000i32..1000000i32));
    println!("Gen a i64 in range -100000000i64..100000000i64: {}", gen.gen_range_i64(-100000000i64..100000000i64));
    println!("Gen a i128 in range -10000000000i128..10000000000i128: {}", gen.gen_range_i128(-10000000000i128..10000000000i128));
    println!("Gen a usize in range 100usize..200usize: {}", gen.gen_range_usize(100usize..200usize));


    println!("Gen 10 u8 in range 100u8..200u8: {:?} no repeat", gen.gen_range_k_u8(100u8..200u8, 10, false));
    println!("Gen 10 u16 in range 10000u16..20000u16: {:?} no repeat", gen.gen_range_k_u16(10000u16..20000u16, 10, false));
    println!("Gen 10 u32 in range 1000000u32..2000000u32: {:?} no repeat", gen.gen_range_k_u32(1000000u32..2000000u32, 10, false));
    println!("Gen 10 u64 in range 100000000u64..200000000u64: {:?} no repeat", gen.gen_range_k_u64(100000000u64..200000000u64, 10, false));
    println!("Gen 10 u128 in range 10000000000u128..20000000000u128: {:?} no repeat", gen.gen_range_k_u128(10000000000u128..20000000000u128, 10, false));
    println!("Gen 10 i8 in range -100i8..100i8: {:?} no repeat", gen.gen_range_k_i8(-100i8..100i8, 10, false));
    println!("Gen 10 i16 in range -10000i16..10000i16: {:?} no repeat", gen.gen_range_k_i16(-10000i16..10000i16, 10, false));
    println!("Gen 10 i32 in range -1000000i32..1000000i32: {:?} no repeat", gen.gen_range_k_i32(-1000000i32..1000000i32, 10, false));
    println!("Gen 10 i64 in range -100000000i64..100000000i64: {:?} no repeat", gen.gen_range_k_i64(-100000000i64..100000000i64, 10, false));
    println!("Gen 10 i128 in range -10000000000i128..10000000000i128: {:?} no repeat", gen.gen_range_k_i128(-10000000000i128..10000000000i128, 10, false));
    println!("Gen 10 usize in range 100usize..200usize: {:?} no repeat", gen.gen_range_k_usize(100usize..200usize, 10, false));

    println!("Gen 10 u8 in range 100u8..110u8: {:?} repeat", gen.gen_range_k_u8(100u8..110u8, 10, true));
    println!("Gen 10 u16 in range 10000u16..10010u16: {:?} repeat", gen.gen_range_k_u16(10000u16..10010u16, 10, true));
    println!("Gen 10 u32 in range 1000000u32..1000010u32: {:?} repeat", gen.gen_range_k_u32(1000000u32..1000010u32, 10, true));
    println!("Gen 10 u64 in range 100000000u64..100000010u64: {:?} repeat", gen.gen_range_k_u64(100000000u64..100000010u64, 10, true));
    println!("Gen 10 u128 in range 10000000000u128..10000000010u128: {:?} repeat", gen.gen_range_k_u128(10000000000u128..10000000010u128, 10, true));
    println!("Gen 10 i8 in range -100i8..-90i8: {:?} repeat", gen.gen_range_k_i8(-100i8..-90i8, 10, true));
    println!("Gen 10 i16 in range -10000i16..-9990i16: {:?} repeat", gen.gen_range_k_i16(-10000i16..-9990i16, 10, true));
    println!("Gen 10 i32 in range -1000000i32..-999990i32: {:?} repeat", gen.gen_range_k_i32(-1000000i32..-999990i32, 10, true));
    println!("Gen 10 i64 in range -100000000i64..-99999990i64: {:?} repeat", gen.gen_range_k_i64(-100000000i64..-99999990i64, 10, true));
    println!("Gen 10 i128 in range -10000000000i128..-9999999990i128: {:?} repeat", gen.gen_range_k_i128(-10000000000i128..-9999999990i128, 10, true));
    println!("Gen 10 usize in range 100usize..110usize: {:?} repeat", gen.gen_range_k_usize(100usize..110usize, 10, true));

    let d = (0..10).into_iter().collect::<Vec<usize>>(); 
    
    println!("Gen from vector{:?}: {}", d, gen.gen_from_vec(&d));
    
    println!("Gen 5 elements from vector{:?}: {:?} repeat", d, gen.gen_from_vec_k(&d, 5, true));
    println!("Gen 5 elements from vector{:?}: {:?} no repeat", d, gen.gen_from_vec_k(&d, 5, false));
    
    let mut d_rs = d.clone();
    gen.random_shuffle(&mut d_rs);
    println!("Random shuffle the vector{:?}: {:?}", d, d_rs);
    
    println!("Split the vector {:?} into 5 parts: {:?} empty", d, gen.split_vec_k(&d, 5, true));
    println!("Split the vector {:?} into 5 parts: {:?} no empty", d, gen.split_vec_k(&d, 5, false));
}   

