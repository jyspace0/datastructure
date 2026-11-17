use std::alloc::{Layout, LayoutError, alloc, dealloc, realloc};
use std::error::Error;
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};
use std::slice;

#[derive(Debug)]
enum MyError {
    UsizeOverflow,
    InvalidLayout(LayoutError),
    OsAllocationFailed(usize),
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::UsizeOverflow => write!(f, "배열 용량 계산 중 usize 최대값을 초과했습니다."),
            MyError::OsAllocationFailed(num) => write!(
                f,
                "운영체제에서 배열의 {} 용량 메모리 할당에 실패했습니다. ",
                num
            ),
            MyError::InvalidLayout(err) => write!(f, "잘못된 메모리 레이아웃 규격입니다. {}", err),
        }
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MyError::InvalidLayout(err) => Some(err),
            _ => None,
        }
    }
}

impl From<LayoutError> for MyError {
    fn from(err: LayoutError) -> Self {
        MyError::InvalidLayout(err)
    }
}

struct MyVec<T> {
    ptr: NonNull<T>, // 데이터 저장 위치
    len: usize,      // 실제 저장된 데이타 갯수
    cap: usize,      // 데이터를 저장할 수 있는 용량
}

impl<T> MyVec<T> {
    fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn capacity(&self) -> usize {
        self.cap
    }

    fn push(&mut self, value: T) -> Result<(), MyError> {
        if self.cap == self.len {
            self.grow()?;
        }

        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), value);
        }
        self.len += 1;

        Ok(())
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        unsafe { Some(ptr::read(self.ptr.as_ptr().add(self.len))) }
    }

    // TODO: expect -> MyError로 변경 처리
    fn grow(&mut self) -> Result<(), MyError> {
        let (new_cap, raw) = if self.cap == 0 {
            let new_cap = 4;
            let layout = Layout::array::<T>(new_cap)?;
            (new_cap, unsafe { alloc(layout) })
        } else {
            let new_cap = self.cap.checked_mul(2).ok_or(MyError::UsizeOverflow)?;
            let old_layout = Layout::array::<T>(self.cap)?;
            let new_size = new_cap * std::mem::size_of::<T>();

            (new_cap, unsafe {
                realloc(self.ptr.as_ptr() as *mut u8, old_layout, new_size)
            })
        };

        self.cap = new_cap;
        self.ptr = NonNull::new(raw as *mut T).ok_or(MyError::OsAllocationFailed(new_cap))?;

        Ok(())
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while let Some(_) = self.pop() {}
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<T> Deref for MyVec<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for MyVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

struct Noisy(String);
impl Drop for Noisy {
    fn drop(&mut self) {
        println!("Noisy({}) 정리됨", self.0);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut v: MyVec<i32> = MyVec::new();
    println!("MyVec 생성: capacity= {}, len={}", v.capacity(), v.len());

    v.push(10)?;
    v.push(20)?;
    v.push(30)?;
    println!(
        "MyVec push 3개: capacity= {}, len={}",
        v.capacity(),
        v.len()
    );

    println!();
    println!("MyVec pop 1개: {:?}", v.pop());
    println!("MyVec pop 2개: {:?}", v.pop());
    println!("MyVec pop 3개: {:?}", v.pop());
    println!("MyVec pop 4개: {:?}", v.pop());
    println!("MyVec pop 4개: capacity= {}, len={}", v.capacity(), v.len());

    for i in (1..11).rev() {
        v.push(i)?;
    }

    println!();
    println!(
        "MyVec push 10개: capacity= {}, len={}",
        v.capacity(),
        v.len()
    );
    println!(
        "MyVec pop 1개: {:?}, capacity= {}, len={}",
        v.pop(),
        v.capacity(),
        v.len()
    );

    v.sort();
    println!("MyVec sort: {} -> {} -> {}...{}", v[0], v[1], v[2], v[8]);
    println!("MyVec sum: {}", v.iter().sum::<i32>());

    let mut v2: MyVec<Noisy> = MyVec::new();
    v2.push(Noisy("오늘은".to_owned()))?;
    v2.push(Noisy("비가".to_owned()))?;
    v2.push(Noisy("오는 날".to_owned()))?;
    println!();
    println!(
        "MyVec Noisy 3개: capacity= {}, len={}",
        v2.capacity(),
        v2.len()
    );

    Ok(())
}
