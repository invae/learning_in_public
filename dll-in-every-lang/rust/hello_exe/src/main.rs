// function signature is brought into scope
#[link(name = "hello_dll.dll", kind="dylib")]
extern {
	fn add(left: usize, right: usize) -> usize;  
}

fn main() {
   unsafe {
        println!("2+2={}", add(2,2));
   }
}
