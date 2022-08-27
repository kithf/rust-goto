#![allow(named_asm_labels)]

macro_rules! label {
  ($l:ident) => {
    unsafe {
        std::arch::asm!(concat!(stringify!($l),":"))
    }
  }
}

macro_rules! goto {
  ($l:ident) => {
    unsafe {
        std::arch::asm!(concat!("jmp ", stringify!($l)))
    }
  }
}

fn main() {
  goto!(test_test);
  println!("Dont print");
  label!(test_test);
  println!("Print");
}
