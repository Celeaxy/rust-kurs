fn main() {
    let _int8: i8 = 0;
    let _int16: i16 = 0;
    let _int32: i32 = 0;
    let _int64: i64 = 0;
    let _int128: i128 = 0;
    
    let _uint8: u8 = 0;
    let _uint16: u16 = 0;
    let _uint32: u32 = 0;
    let _uint64: u64 = 0;
    let _uint128: u128 = 0;

    println!("{}, {}, {}, {}", u8::MAX, i16::MIN, u128::MAX, isize::MAX);

    let _dec = 255;
    let _hex = 0xff;
    let _bin = 0b11111111;
    println!("dec: {}\nhex: {}\nbin: {}", _dec, _hex, _bin);

    let _f32: f32 = 2.0;
    let _f64: f64 = 3.0;

    let _f32_2 = 2.0_f32;
    let _f64_2 = 3.0_f64;


    let _bool = false;
    let _bool2: bool = true;
    
}
