/// 값을 당기지 않는 빠른 Queue 구조체
struct FastQueue<T> {
    data: Vec<Option<T>>, // 저장할 값
    head: usize,          // 맨 앞 값이 있는 칸 번호
    len: usize,           // 저장된 갯수
}

impl<T> FastQueue<T> {
    /// FastQueue 생성
    fn new() -> Self {
        let mut data: Vec<Option<T>> = Vec::new();
        for _ in 0..4 {
            data.push(None);
        }

        Self {
            data,
            head: 0,
            len: 0,
        }
    }

    /// FastQueue 맨 끝에 데이타 넣기
    fn push_back(&mut self, value: T) {
        // 더 들어갈 수 없을 경우 용량 늘리기
        if self.len == self.data.len() {
            self.grow();
        }

        let cap = self.data.len(); // capcity

        // 맨 뒤에 데이터가 들어갈 위치 계산
        // 들어갈 위치 = (head + len) % cap
        let tail = (self.head + self.len) % cap;
        self.data[tail] = Some(value);
        self.len += 1;
    }

    // 꺼내기: 값을 당기지 않고 head만 옮긴다
    fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None; // 비어있으면 None
        }

        // capacity 계산
        let cap = self.data.len();

        // 꺼낼 위치 계산
        // 꺼낼 위치 = head % cap
        let out_index = self.head % cap;
        let out = self.data[out_index].take(); // head 칸의 값을 꺼낸다

        // head 위치를 한 칸 증가
        // 꺼낼 위치 저장 (head + 1) % cap
        self.head = (self.head + 1) % cap;
        self.len -= 1; // 저장 갯수를 줄인다
        out
    }

    fn grow(&mut self) {
        let old_cap = self.data.len(); // 예전 capacity
        let new_cap = old_cap * 2; // 용량 2배
        let mut new_data: Vec<Option<T>> = Vec::new(); // 새 칸들

        for _ in 0..new_cap {
            new_data.push(None); // 빈칸으로 채움
        }

        // 이전 데이타를 새 칸으로 이전
        for i in 0..self.len {
            // new_data[i] = self.pop_front();

            new_data[i] = self.data[(self.head + i) % old_cap].take();
        }

        self.data = new_data; // 새 칸으로 교체
        self.head = 0;
    }
}

fn main() {
    let mut fq: FastQueue<i32> = FastQueue::new();

    fq.push_back(10);
    fq.push_back(20);
    fq.push_back(30);

    println!(
        "1'st pop: value = {:?}, head = {}, len = {}",
        fq.pop_front(),
        fq.head,
        fq.len
    );
    println!(
        "2'nd pop: value = {:?}, head = {}, len = {}",
        fq.pop_front(),
        fq.head,
        fq.len
    );
    println!(
        "3'rd pop: value = {:?}, head = {}, len = {}",
        fq.pop_front(),
        fq.head,
        fq.len
    );
    println!(
        "4'th pop: value = {:?}, head = {}, len = {}",
        fq.pop_front(),
        fq.head,
        fq.len
    );

    fq.push_back(10);
    fq.push_back(20);

    println!(
        "5'th pop: value = {:?}, head = {}, len = {}",
        fq.pop_front(),
        fq.head,
        fq.len
    );
    println!(
        "6'th pop: value = {:?}, head = {}, len = {}",
        fq.pop_front(),
        fq.head,
        fq.len
    );
    println!(
        "7'th pop: value = {:?}, head = {}, len = {}",
        fq.pop_front(),
        fq.head,
        fq.len
    );

    grow_keeps_order();
}

fn grow_keeps_order() {
    let mut q = FastQueue::new(); // 용량 4짜리 큐 (new는 직접 만들어 보세요)
    q.push_back(1);
    q.push_back(2);
    q.pop_front(); // head를 앞으로 옮겨서 값이 "휘어지게" 만든다
    q.push_back(3);
    q.push_back(4);
    q.push_back(5); // 여기서 꽉 차고, 다음 push에서 grow가 일어난다
    q.push_back(6); // grow 발생!
    // 기대하는 꺼내기 순서를 assert_eq!로 적어 보세요.
    // assert_eq!(q.pop_front(), Some(2));
    println!(
        "test pop: value = {:?}, head = {}, len = {}",
        q.pop_front(),
        q.head,
        q.len
    );
}
