/// 동적배열: 데이터의 크기에 맞춰 크기를 변경하는 배열
/// 1단계: Box[Option<T>]로 구현

struct MyVec<T> {
    data: Vec<Option<T>>, // Some(값) 또는 None(빈칸)
    len: usize,           // 실제로 들어 있는 개수
}

impl<T> MyVec<T> {
    /// 용량 4짜리 빈 배열을 생성한다
    fn new() -> Self {
        let mut data = Vec::new();
        for _ in 0..4 {
            data.push(None);
        }
        MyVec { data, len: 0 }
    }

    /// 현재 용량(최대 저장 갯수)을 돌려준다
    fn capacity(&self) -> usize {
        self.data.len()
    }

    /// 비어 있는 칸에 data를 넣는다
    fn push(&mut self, value: T) {
        // 꽉 찼는지 먼저 확인
        if self.len == self.capacity() {
            self.grow();
        }
        self.data[self.len] = Some(value); // 데이터 삽입
        self.len += 1; // 길이 1 증가
    }

    /// 현재 용량(최대 저장 갯수)를 2배 늘린다.
    fn grow(&mut self) {
        let old_cap = self.capacity(); // 지금 칸 수를 기억
        let new_cap = 2 * old_cap; // 두배로 늘린다

        for _ in old_cap..new_cap {
            self.data.push(None);
        }
    }

    /// 인덱스의 값을 빌려서 보여준다
    fn get(&self, index: usize) -> Option<&T> {
        // 범위 점검 (저장된 갯 수 이내), 범위 밖 None
        if index < self.len {
            self.data[index].as_ref()
        } else {
            None
        }
    }

    /// 맨 뒤 값을 꺼내서 돌려준다. 비어 있으면 None
    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            self.data[self.len].take() // self.len은 채울 칸의 번호
        }
    }
}

struct Stack<T> {
    item: MyVec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self { item: MyVec::new() }
    }

    // 맨 위에 값을 넣는다
    fn push(&mut self, value: T) {
        self.item.push(value);
    }

    // 맨 위 갓을 꺼낸다 (비어 있으면 None)
    fn pop(&mut self) -> Option<T> {
        self.item.pop()
    }
}

fn main() {
    let mut v: MyVec<i32> = MyVec::new();
    println!("len = {}, cap = {}", v.len, v.capacity());

    v.push(10);
    v.push(20);
    v.push(30);
    v.push(40);
    println!("len = {}, cap = {}", v.len, v.capacity());
    v.push(50);
    v.push(60);
    v.push(70);
    v.push(80);
    println!("len = {}, cap = {}", v.len, v.capacity());
    v.push(90);
    println!("len = {}, cap = {}", v.len, v.capacity());

    let mut v: MyVec<i32> = MyVec::new();
    v.push(10);
    v.push(20);
    println!("{:?}", v.get(0));
    println!("{:?}", v.get(1));
    println!("{:?}", v.get(2));

    let mut v: MyVec<i32> = MyVec::new();
    v.push(10);
    v.push(20);
    println!("{:?}", v.pop());
    println!("{:?}", v.pop());
    println!("{:?}", v.pop());

    let mut s: Stack<i32> = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    println!("{:?}", s.pop());
    println!("{:?}", s.pop());
    println!("{:?}", s.pop());
}
