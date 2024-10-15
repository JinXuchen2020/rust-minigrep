use std::{mem, slice::{self, from_raw_parts}};
use std::arch::asm;

fn get_memory_location() -> (usize, usize) {
  let string = "Hello World!";
  let pointer = string.as_ptr() as usize;
  let length = string.len();
  (pointer, length)
}

pub const unsafe fn from_utf8_unchecked(v: &[u8]) -> &str {
  // SAFETY: the caller must guarantee that the bytes `v` are valid UTF-8.
  // Also relies on `&str` and `&[u8]` having the same layout.
  mem::transmute(v)
}

fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
  unsafe { from_utf8_unchecked(from_raw_parts(pointer as *const u8, length)) }
}

pub fn unsafe_raw_pointer() {
  let mut num = 5;

  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;

  let (pointer, length) = get_memory_location();
  let message = get_str_at_location(pointer, length);
  println!(
    "The {} bytes at 0x{:X} stored: {}",
    length, pointer, message
  );

  let a: Box<i32> = Box::new(10);
  // 需要先解引用a
  let b: *const i32 = &*a as *const i32;

  // 使用 into_raw 来创建
  let c: *const i32 = Box::into_raw(a);

  let mut v = vec![1, 2, 3, 4, 5, 6];

  let r = &mut v[..];

  let (part1, part2) = split_at_mut(r, 3);

  println!("part1 is: {:?}", part1);
  println!("part2 is: {:?}", part2);

  extern_ffi();

  asm();

  unsafe {
    *r2 = 10;

    println!("r1 is: {}", *r1);
    println!("r1 address is: {:p}", r1);

    println!("r1 is: {}", *r1);
    println!("r1 address is: {:p}", r1);    

    println!("b is: {}", *b);
    println!("b address is: {:p}", b);   

    println!("c is: {}", *c);
    println!("c address is: {:p}", c);
  }
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
  let len = slice.len();
  let ptr = slice.as_mut_ptr();

  assert!(mid <= len);

  unsafe {
      (
          slice::from_raw_parts_mut(ptr, mid),
          slice::from_raw_parts_mut(ptr.add(mid), len - mid),
      )
  }
}

fn extern_ffi() {
  extern "C" {
    fn abs(input: i32) -> i32;
  }

  unsafe {
    println!("Absolute value of -3: {}", abs(-3));
  }
}

fn asm() {
  let x: u64;
  unsafe {
    asm!("mov {}, 5", out(reg) x);
  }
  println!("x is: {}", x);

  let i: u64 = 3;
  let o: u64;
  let j: u64;
  unsafe {
      asm!(
          "mov {0}, {1}",
          "add {0}, 5",
          out(reg) o,
          in(reg) i,
      );
      asm!("add {0}, 5", inout(reg) o => j);
  }

  println!("o is: {}", o);
  println!("j is: {}", j);

  mul(o, j);
  clobbered();
}

fn mul(a: u64, b: u64) -> u128 {
  let lo: u64;
  let hi: u64;

  unsafe {
      asm!(
          // The x86 mul instruction takes rax as an implicit input and writes
          // the 128-bit result of the multiplication to rax:rdx.
          "mul {}",
          in(reg) a,
          inlateout("rax") b => lo,
          lateout("rdx") hi
      );
  }

  ((hi as u128) << 64) + lo as u128
}

fn clobbered() {
  let mut name_buf = [0_u8; 12];
    // String is stored as ascii in ebx, edx, ecx in order
    // Because ebx is reserved, the asm needs to preserve the value of it.
    // So we push and pop it around the main asm.
    // (in 64 bit mode for 64 bit processors, 32 bit processors would use ebx)

  unsafe {
      asm!(
          "push rbx",
          "cpuid",
          "mov [rdi], ebx",
          "mov [rdi + 4], edx",
          "mov [rdi + 8], ecx",
          "pop rbx",
          // We use a pointer to an array for storing the values to simplify
          // the Rust code at the cost of a couple more asm instructions
          // This is more explicit with how the asm works however, as opposed
          // to explicit register outputs such as `out("ecx") val`
          // The *pointer itself* is only an input even though it's written behind
          in("rdi") name_buf.as_mut_ptr(),
          // select cpuid 0, also specify eax as clobbered
          inout("eax") 0 => _,
          // cpuid clobbers these registers too
          out("ecx") _,
          out("edx") _,
      );
  }

  let name = core::str::from_utf8(&name_buf).unwrap();
  println!("CPU Manufacturer ID: {}", name);
}