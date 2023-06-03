use sodium_proc_macros::jni_export;

fn main() {
    println!("main");
}

// #[jni_export(com.sodium.Test.testFunction)]
// pub fn test_snake_case(h: *mut i32) {
//     println!("test {:?}", h);
// }

#[jni_export(com.sodium.Test)]
mod sdfjhsdf {
    pub fn testSnakeCase(h: *mut i32) {
        println!("test {:?}", h);
    }
}
