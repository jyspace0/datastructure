use std::collections::HashMap;

// 숫자 키를 칸번호로 바꾸는 가장 단순한 해시 함수
fn hash(key: usize, size: usize) -> usize {
    key % size // 키를 칸 개수로 나눈 나머지가 곧 칸 번호
}

// 해시 테이블 구조체: 바구니들을 모아 둔 Vec
struct HashTable {
    buckets: Vec<Vec<(usize, i32)>>, // 칸 마다 (키, 값) 쌍을 담는 바구니
}
impl HashTable {
    // 칸 size개 짜리 빈 테이블 만들기
    fn new(size: usize) -> Self {
        HashTable {
            buckets: vec![Vec::new(); size], // 빈 바구니 size 개
        }
    }

    // 키를 칸 번호로 바꾸는 해시함수
    fn index(&self, key: usize) -> usize {
        key % self.buckets.len()
    }

    // (키, 값) 쌍을 알맞는 바구니에 넣기, 같은 키는 덮어쓰고, 없으면 새로 추가
    fn insert(&mut self, key: usize, value: i32) {
        let idx = self.index(key); // 몇 번 칸인지 계산
        // 바꿀 수 있게 key가 들어갈 바구니 빌리기
        let bucket = &mut self.buckets[idx];

        for pair in bucket.iter_mut() {
            // for pair in self.buckets[idx].iter_mut() {
            if pair.0 == key {
                pair.1 = value;
                return;
            }
        }
        bucket.push((key, value));

        // self.buckets[idx].push((key, value)); // 그 바구니 맨뒤에 쌍 추가
    }

    // (키, 값) 쌍을 찾기, 없으면 None
    fn get(&self, key: usize) -> Option<i32> {
        let idx = self.index(key); // 몇 번 칸인지 계산

        for &(k, v) in &self.buckets[idx] {
            // 바구니의 키, 값 훝기
            if k == key {
                // 키가 같으면
                return Some(v); // 값을 돌려준다
            }
        }
        None // 끝까지 없으면 None
    }

    // (키, 값) 쌍을 지우고, 값을 돌려주기. 없으면 None
    fn remove(&mut self, key: usize) -> Option<i32> {
        let idx = self.index(key); // 몇 번 칸인지 계산

        // 지울 값이 있는 바구니를 지울 수 있게 빌리기
        let bucket = &mut self.buckets[idx];

        for i in 0..bucket.len() {
            // 바구니의 키, 값 훝기
            if bucket[i].0 == key {
                let (_, v) = bucket.remove(i);
                // 키가 같으면
                return Some(v); // 값을 돌려준다
            }
        }
        None // 끝까지 없으면 None
    }
}

fn main() {
    let size = 10; // 사물함(칸)이 10개

    println!("{}", hash(20237, size)); // 20237 % 10
    println!("{}", hash(20233, size)); // 20233 % 10
    println!("{}", hash(15, size)); // 15 % 10

    println!("{}", hash(25, 7)); // 25 % 7
    println!("{}", hash(32, 7)); // 32 % 7

    // 칸 7개, 각 칸은 비어 있는 바구니 (Vec)
    let mut table: Vec<Vec<usize>> = vec![Vec::new(); 7];

    // 25의 칸 번호를 구해서 그 칸의 바구니에 넣기
    let idx = 25 % 7; // 4번 칸
    table[idx].push(25); // 4번 바구니에 25추가

    // 32도 같은 방식으로 넣기
    let idx = 32 % 7; // 4번 칸
    table[idx].push(32); // 같은 4번 바구니에 32 추가

    // 4도 같은 방식으로 넣기
    let idx = 4 % 7; // 4번 칸
    table[idx].push(4); // 같은 4번 바구니에 32 추가

    println!("{:?}", table[4]);

    // Hash Table Check
    let mut table = HashTable::new(7); // 칸7짜리, table 생성

    table.insert(25, 90); // 키 25, 값 90
    table.insert(32, 85); // 키 32, 값 85
    table.insert(4, 70); // 키 4, 값 70
    table.insert(4, 80); // 동일 키 값 입력 (키 4, 값 70)
    println!("{:?}", table.buckets[4]);

    println!("{:?}", table.get(32)); // 있는 키
    println!("{:?}", table.get(11)); // 없는 키
    println!("{:?}", table.get(11)); // 없는 키
    println!("{:?}", table.get(4)); // 있는 키
    println!("{:?}", table.get(4)); // 있는 키

    println!("self. remove");
    println!("{:?}", table.remove(32)); // 있는 키 지우기
    println!("{:?}", table.buckets[4]); // 4번 바구니 확인
    println!("{:?}", table.remove(32)); // 이미 지운 키 다시 지우기
    println!("{:?}", table.remove(4)); // 있는 키 다시 지우기
    println!("{:?}", table.remove(4)); // 이미 지운 키 다시 지우기

    println!("덮어쓰기 insert");
    let mut table = HashTable::new(7);
    table.insert(25, 90);
    table.insert(25, 100); // 같은 키, 새 값

    println!("{:?}", table.get(25));
    println!("{:?}", table.buckets[4]);

    println!("======== 표준 해시맵 =========");
    let mut map = HashMap::new();

    map.insert("apple", 3);
    map.insert("banana", 5);
    map.insert("apple", 10);

    println!("{:?}", map.get("apple"));
    println!("{:?}", map.get("cherry"));
    println!("{}", map.len());
    println!("{:?}", map.remove("apple"));
    println!("{}", map.len());

    let words = ["apple", "banana", "apple", "apple", "banana", "cherry"];
    let mut counts: HashMap<&str, i32> = HashMap::new();

    for w in &words {
        let c = counts.entry(w).or_insert(0);
        *c += 1;
    }

    println!("{:?}", counts.get("apple"));
    println!("{:?}", counts.get("banana"));
    println!("{}", counts.len());
    println!("{:?}", counts.get("cherry"));
    println!("{}", counts.len());

    let mut map: HashMap<&str, String> = HashMap::new();
    map.insert("name", String::from("민수"));
    map.insert("city", String::from("서울"));
    map.insert("name", String::from("지영"));

    if let Some(v) = map.get("name") {
        println!("{}", v);
    }
    println!("{:?}", map.get("name"));
    println!("{}", map.len());

    let mut counts: HashMap<char, i32> = HashMap::new();
    for ch in "hello".chars() {
        let c = counts.entry(ch).or_insert(0);
        *c += 1;
    }
    println!("{:?}", counts.get(&'l'));
}
