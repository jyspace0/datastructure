/// 동적배열 구현 : 공간이 부족하면 늘림
/// len: 실제 데이타가 들어있는 갯수
/// // capacity: 실제로 데이터를 담을 수 있는 최대 개수 -> data.len()
struct MyVec<T> {
    data: Vec<Option<T>>,
    len: usize,
}

impl<T> MyVec<T> {
    fn new() -> Self {
        let mut data = Vec::new();

        for _ in 0..4 {
            data.push(None);
        }

        Self { data, len: 0 }
    }

    fn capacity(&self) -> usize {
        self.data.len()
    }

    fn push(&mut self, value: T) {
        if self.len == self.capacity() {
            self.grow();
        }
        self.data[self.len] = Some(value);
        self.len += 1;
    }

    fn grow(&mut self) {
        let old_cap = self.capacity();
        for _ in old_cap..(old_cap * 2) {
            self.data.push(None);
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            self.data[index].as_ref()
        } else {
            None
        }
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1; // self.len은 들어갈 위치이기 때문에 먼저 줄임
            let out = self.data[self.len].take(); // 소유권 때문에 take()
            out
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

    fn push(&mut self, value: T) {
        self.item.push(value);
    }

    fn pop(&mut self) -> Option<T> {
        self.item.pop()
    }
}

struct Queue<T> {
    item: MyVec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self { item: MyVec::new() }
    }

    fn push(&mut self, value: T) {
        self.item.push(value);
    }

    fn remove_front(&mut self) -> Option<T> {
        if self.item.len == 0 {
            None
        } else {
            let out = self.item.data[0].take(); // take() ?
            for i in 1..self.item.len {
                self.item.data.swap(i - 1, i);
            }
            self.item.len -= 1;
            out
        }
    }
}

struct FastQueue<T> {
    data: Vec<Option<T>>, // 데이터
    head: usize,          // 맨 앞 값이 있는 번호
    len: usize,           // 데이터 개수
}

impl<T> FastQueue<T> {
    fn new() -> Self {
        let mut data = Vec::new();
        for _ in 0..4 {
            data.push(None);
        }
        Self {
            data,
            head: 0,
            len: 0,
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let cap = self.data.len();
        let out = self.data[self.head].take();
        self.head = (self.head + 1) % cap;
        self.len -= 1;
        out
    }

    fn push_back(&mut self, value: T) {
        if self.len == self.data.len() {
            self.grow();
        }

        let cap = self.data.len();
        self.data[(self.head + self.len) % cap] = Some(value);
        self.len += 1;
    }

    fn grow(&mut self) {
        let old_cap = self.data.len();
        let new_cap = old_cap * 2;

        let mut new_vec = Vec::new();
        for _ in 0..new_cap {
            new_vec.push(None);
        }

        // 데이터 이동
        for i in 0..self.len {
            new_vec[i] = self.data[(self.head + i) % old_cap].take();
        }

        self.head = 0;
        self.data = new_vec;
    }
}

fn main() {
    // MyVec 점검
    let mut v: MyVec<i32> = MyVec::new();
    println!(
        "data={:?}, len={}, capacity={}",
        v.data,
        v.len,
        v.capacity()
    );

    v.push(10);
    v.push(20);
    println!(
        "data={:?}, len={}, capacity={}",
        v.data,
        v.len,
        v.capacity()
    );

    for i in 0..9 {
        v.push(i);
        println!(
            "data={:?}, len={}, capacity={}",
            v.data,
            v.len,
            v.capacity()
        );
    }

    println!(
        "data={:?}, len={}, capacity={}",
        v.get(0),
        v.len,
        v.capacity()
    );
    println!(
        "data={:?}, len={}, capacity={}",
        v.get(5),
        v.len,
        v.capacity()
    );
    println!(
        "data={:?}, len={}, capacity={}",
        v.get(10),
        v.len,
        v.capacity()
    );
    println!(
        "\ndata={:?}, len={}, capacity={}",
        v.pop(),
        v.len,
        v.capacity()
    );
    println!(
        "data={:?}, len={}, capacity={}",
        v.pop(),
        v.len,
        v.capacity()
    );
    println!(
        "data={:?}, len={}, capacity={}",
        v.pop(),
        v.len,
        v.capacity()
    );

    // Stack 점검
    let mut s: Stack<i32> = Stack::new();

    s.push(1);
    s.push(2);
    s.push(3);

    println!();
    println!("{:?}", s.pop());
    println!("{:?}", s.pop());
    println!("{:?}", s.pop());
    println!("{:?}", s.pop());

    // Queue 점검
    let mut q: Queue<i32> = Queue::new();

    q.push(1);
    q.push(2);
    q.push(3);

    println!();
    println!("{:?}", q.remove_front());
    println!("{:?}", q.remove_front());
    println!("{:?}", q.remove_front());
    println!("{:?}", q.remove_front());

    // FastQueue 검증
    let mut fq: FastQueue<i32> = FastQueue::new();

    fq.push_back(1);
    fq.push_back(2);
    fq.push_back(3);
    println!();
    println!("{:?}", fq.pop_front());
    println!("{:?}", fq.pop_front());
    fq.push_back(4);
    fq.push_back(5);
    fq.push_back(6);
    println!("{:?}", fq.pop_front());
    println!("{:?}", fq.pop_front());
    println!("{:?}, len={}", fq.pop_front(), fq.len);

    // FastQueue grow 점검
    let mut fq1: FastQueue<i32> = FastQueue::new();

    fq1.push_back(1);
    fq1.push_back(2);
    println!();
    println!(
        "{:?}, len= {}, cap = {}",
        fq1.pop_front(),
        fq1.len,
        fq1.data.len()
    );
    fq1.push_back(3);
    fq1.push_back(4);
    fq1.push_back(5);
    fq1.push_back(6);
    println!(
        "{:?}, len= {}, cap = {}",
        fq1.pop_front(),
        fq1.len,
        fq1.data.len()
    );
}
