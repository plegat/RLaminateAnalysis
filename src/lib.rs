# [no_mangle]
pub extern "stdcall" fn hello (count: i32)->i32 {
    if count<0 {
        return 0;
    }
    return count*2;
}
# [no_mangle]
pub extern "stdcall" fn hello_float32 (val1: f32,val2: f32)->f32 {
    println!("val1= {}",val1);
    println!("val2= {}",val2);
    
    return val1*val2;
}
# [no_mangle]
pub extern "stdcall" fn hello_float64 (val1: f64,val2: f64)->f64 {
    println!("val1= {}",val1);
    println!("val2= {}",val2);
    
    return val1*val2;
}
# [no_mangle]
pub extern "stdcall" fn hello2 () {
    println!("hello 2 ok");
}
# [no_mangle]
pub extern "stdcall" fn hello3_f32(val: f32, tab: &mut [f32;6]) {
    
    for i in 0..6 {
        tab[i]=val*(i+1) as f32;
    }

}
# [no_mangle]
pub extern "stdcall" fn hello3_f64(val: f64, tab: &mut [f64;6]) {
    
    for i in 0..6 {
        tab[i]=val*(i+1) as f64;
    }

}

