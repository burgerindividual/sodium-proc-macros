use sodium_proc_macros::jni_export;

pub type JEnv = u64;
pub type JClass = u64;

fn main() {
    println!("main");
}

// #[jni_export(com.sodium.Test.testFunction)]
// pub fn test_snake_case(h: *mut i32) {
//     println!("test {:?}", h);
// }

#[jni_export(com.sodium.Test.testOne)]
pub fn test_one_snake_case(h: *mut i32) {
    println!("test 1 {:?}", h);
}

#[jni_export(com.sodium.Test)]
mod javastuffidklol {
    use crate::JClass;
    use crate::JEnv;

    pub fn testTwo(h: *mut i32) {
        println!("test 2 {:?}", h);
    }

    pub fn testThree(h: *mut i32) {
        println!("test 3 {:?}", h);
    }
}
