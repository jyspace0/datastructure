use std::alloc::{Layout, alloc, dealloc, realloc};
use std::ops::{Deref, DerefMut};
use std::ptr;
use std::ptr::NonNull;
use std::slice;

struct PtrVec<T> {
    ptr: NonNull<T>, // 칸들이 시작되는 주소 (null이 아님을 보장)
    cap: usize,      // 칸 수 (용량)
    len: usize,      // 채워진 개수
}

impl<T> PtrVec<T> {
    // 메모리를 아직 빌리지 않은 빈 상태로 시작
    fn new() -> Self {
        PtrVec {
            ptr: NonNull::dangling(), // 가짜 주소 : 아직 쓰면 안됨
            cap: 0,
            len: 0,
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn capacity(&self) -> usize {
        self.cap
    }

    fn grow(&mut self) {
        let (new_cap, raw) = if self.cap == 0 {
            let new_cap = 4; // 처음엔 4칸을 빌림
            let layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, unsafe { alloc(layout) })
        } else {
            let new_cap = self.cap * 2; // 두배로 빌림
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let new_size = new_cap * std::mem::size_of::<T>();

            (new_cap, unsafe {
                realloc(self.ptr.as_ptr() as *mut u8, old_layout, new_size)
            })
        };

        self.cap = new_cap;
        self.ptr = NonNull::new(raw as *mut T).expect("메모리 못 빌림");
    }

    fn push(&mut self, value: T) {
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), value);
        }
        self.len += 1; // 개수 1 증가
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1; // self.len의 위치는 들어갈 위치
        unsafe { Some(ptr::read(self.ptr.as_ptr().add(self.len))) }
    }
}

impl<T> Drop for PtrVec<T> {
    fn drop(&mut self) {
        // cap == 0은 빌린 적이 없다는 뜻임
        if self.cap != 0 {
            // String, Box등의 해제(Drop)를 위해 값들을 꺼냄
            if std::mem::needs_drop::<T>() {
                while let Some(_) = self.pop() {}
            }

            let layout = Layout::array::<T>(self.cap).unwrap(); // 빌릴 때와 똑 같은 주문서
            unsafe {
                dealloc(self.ptr.as_ptr() as *mut u8, layout); // 주소와 주문서를 함께 돌려준다.
            }
        }
    }
}

impl<T> Deref for PtrVec<T> {
    type Target = [T]; // 이 타입은 [T] 처럼 보인다

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for PtrVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

struct Noisy(i32); // 정리될때 알려주는 타입
impl Drop for Noisy {
    fn drop(&mut self) {
        println!("Noisy({}) 정리됨", self.0);
    }
}

fn main() {
    let mut v: PtrVec<i32> = PtrVec::new();
    println!("len={}, cap={}", v.len, v.cap); // len 0, cap 0

    v.push(10);
    v.push(20);
    v.push(30);
    println!("len={}, cap={}", v.len(), v.capacity()); // len 3, cap 4

    println!();
    println!("{:?}", v.pop());
    println!("{:?}", v.pop());
    println!("{:?}", v.pop());
    println!("{:?}", v.pop());
    println!("len={}, cap={}", v.len(), v.capacity()); // len 0, cap 4

    let mut v1: PtrVec<i32> = PtrVec::new();

    for i in 1..=10 {
        v1.push(i);
    }
    println!();
    println!("len={}, cap={}", v1.len(), v1.capacity()); // len 0, cap 4
    println!("{:?}", v1.pop());
    println!("len={}, cap={}", v1.len(), v1.capacity()); // len 0, cap 4

    let mut v2: PtrVec<Noisy> = PtrVec::new();
    v2.push(Noisy(1));
    v2.push(Noisy(2));
    v2.push(Noisy(3));
    println!();
    println!("범위 끝");

    let mut v: PtrVec<i32> = PtrVec::new();
    v.push(3);
    v.push(1);
    v.push(2);
    v.sort();
    println!();
    println!("after sort: {} -> {} -> {}", v[0], v[1], v[2]);

    let mut v: PtrVec<i32> = PtrVec::new();
    v.push(10);
    v.push(20);
    print_all(&v); // &PtrVec이 &[i32]로 자동 변환

    // i32 4칸짜리 주문서를 만든다 (실패하면 unwrap이 패닉)

    // let layout = Layout::array::<i32>(4).unwrap();

    // // 주문서대로 메모리를 빌린다. (unsafe: 결과를 우리가 책임져야함)
    // let raw = unsafe { alloc(layout) } as *mut i32;

    // // 빌리기에 실패하면 null이 와요. null이 아닌지 확인하고 NonNull로 감쌈
    // let ptr = NonNull::new(raw).expect("메모리를 빌리지 못했어요");

    // println!("빌렸어요: {:?}", ptr);

    // std::mem::drop.ptr;
}

fn print_all(arr: &[i32]) {
    for x in arr {
        println!("{}", x);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ptrvec_grows() {
        let mut v: PtrVec<i32> = PtrVec::new();

        for i in 0..10 {
            v.push(i);
        }

        assert_eq!(v.len(), 10);
        assert_eq!(v.capacity(), 16);
        assert_eq!(v.pop(), Some(9));
    }

    #[test]
    fn ptrvec_drops_values() {
        let mut v: PtrVec<String> = PtrVec::new();
        v.push("a".to_string());
        v.push("b".to_string());
        assert_eq!(v.pop(), Some("b".to_string()));
    }

    #[test]
    fn ptrvec_ref_values() {
        let mut v: PtrVec<i32> = PtrVec::new();
        v.push(10);
        v.push(20);
        v.push(30);
        assert_eq!(20, v[1]);
        assert_eq!(3, v.len());
        assert_eq!(60, v.iter().sum());
    }
}
