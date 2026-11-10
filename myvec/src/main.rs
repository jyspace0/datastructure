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
}
