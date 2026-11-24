use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// hash table 구조체: (키, 값) 쌍으로 받음
pub struct HashTable {
    buckets: Vec<Vec<(usize, i32)>>,
}

impl HashTable {
    // 빈 HashTable 생성
    pub fn new(size: usize) -> Self {
        HashTable {
            buckets: vec![Vec::new(); size],
        }
    }

    // 키가 들어갈 칸
    pub fn index(&self, key: usize) -> usize {
        key % self.buckets.len()
    }

    // (키, 값) 넣음. 기존 키 존재하면 덮어쓰고, 없으면 추가
    pub fn insert(&mut self, key: usize, value: i32) {
        let idx = self.index(key);

        for pair in self.buckets[idx].iter_mut() {
            // 같은 키가 있으면, 덮어쓴다
            if pair.0 == key {
                pair.1 = value;
                return;
            }
        }
        self.buckets[idx].push((key, value)); // 없으면 추가

        // 적재율(load_factor)이 2를 넘으면 2배 키움
        if self.load_factor() > 2.0 {
            let new_size = self.buckets.len() * 2;
            self.resize(new_size);
        }
    }

    // 해당 key의 값을 읽어온다
    pub fn get(&self, key: usize) -> Option<i32> {
        let idx = self.index(key);

        for pair in self.buckets[idx].iter() {
            if pair.0 == key {
                return Some(pair.1);
            }
        }
        None
    }

    // 해당 key, 값을 제거하고, 그 값을 돌려준다
    pub fn remove(&mut self, key: usize) -> Option<i32> {
        let idx = self.index(key);
        let bucket = &mut self.buckets[idx];

        for i in 0..bucket.len() {
            if bucket[i].0 == key {
                let (_, v) = bucket.remove(i);
                return Some(v);
            }
        }
        None
    }

    // 테이블에 들어있는 쌍의 총 개수
    pub fn len(&self) -> usize {
        let mut total = 0;

        for bucket in &self.buckets {
            total += bucket.len();
        }
        total
    }

    // 적재율 = 쌍의 개수 / 칸수
    pub fn load_factor(&self) -> f64 {
        self.len() as f64 / self.buckets.len() as f64
    }

    // bucket 크기를 키우고, 재 배열
    // 칸 수를 new_size로 바꾸고, 모든 쌍을 다시 나눠 담기
    pub fn resize(&mut self, new_size: usize) {
        let mut new_buckets: Vec<Vec<(usize, i32)>> = vec![Vec::new(); new_size];

        for bucket in &self.buckets {
            for &(k, v) in bucket {
                let idx = k % new_size;
                new_buckets[idx].push((k, v));
            }
        }
        self.buckets = new_buckets; // 새 바구니로 바꾸기
    }

    // Hash를 구현한 타입이면 무엇이든 받아서 u64 숫자로 바꾸기
    // pub fn hash_of<T: Hash>(key: &T) -> u64 {
    //     let mut hasher = DefaultHasher::new();
    //     key.hash(&mut hasher);
    //     hasher.finish()
    // }
}

// Hash를 구현한 타입이면 무엇이든 받아서 u64 숫자로 바꾸기
pub fn hash_of<T: Hash>(key: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

pub struct StrTable {
    buckets: Vec<Vec<(String, i32)>>, // 키가 String인 (키, 값) 쌍
}

impl StrTable {
    pub fn new(size: usize) -> Self {
        StrTable {
            buckets: vec![Vec::new(); size],
        }
    }

    // 문자열 키 -> 해시 숫자 -> 칸 번호
    pub fn index(&self, key: &str) -> usize {
        // u64로 변환 뒤 usize로 변환
        (hash_of(&key) % self.buckets.len() as u64) as usize
    }

    pub fn insert(&mut self, key: &str, value: i32) {
        let idx = self.index(key); // 칸 번호
        let bucket = &mut self.buckets[idx];

        for pair in bucket.iter_mut() {
            if pair.0 == key {
                pair.1 = value;
                return;
            }
        }
        bucket.push((key.to_string(), value)); // key가 없으면 추가
    }

    pub fn get(&self, key: &str) -> Option<i32> {
        let idx = self.index(key);

        for (k, v) in &self.buckets[idx] {
            if k == key {
                return Some(*v);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_load_factor() {
        let mut table = HashTable::new(7);
        for k in 0..10 {
            table.insert(k, 1);
        }

        println!("==== Test No 1 ====");
        println!("{}", table.len()); // 쌍의 개수
        println!("{}", table.load_factor()); // 적재율
        assert_eq!(10, table.len());
        assert_eq!(1.4285714285714286, table.load_factor());
    }

    #[test]
    fn check_resize() {
        let mut table = HashTable::new(7);
        table.insert(25, 90);
        table.insert(32, 85);

        table.resize(14);
        println!("==== Test No 2 ====");
        println!("{:?}", table.buckets[11]);
        println!("{:?}", table.get(25));
        println!("{:?}", table.get(32));
        assert_eq!(Some(90), table.get(25));
        assert_eq!(Some(85), table.get(32));
    }

    #[test]
    fn check_insert() {
        let mut table = HashTable::new(2);
        println!("==== Test No 3 ====");
        for k in 0..9 {
            table.insert(k, 1);
            println!("키 {} 삽입 후: 칸 {}개", k, table.buckets.len());
        }
        assert_eq!(8, table.buckets.len());
    }

    #[test]
    fn check_hash_of() {
        let h1 = hash_of(&"apple");
        let h2 = hash_of(&"apple");
        let idx = (h1 % 7) as usize;
        println!("==== Test No 4 ====");
        println!("{}", h1 == h2);
        println!("{} {}", idx, idx < 7);

        let h1 = hash_of(&"banana");
        let h2 = hash_of(&"apple");
        let idx = (h1 % 7) as usize;
        let idx1 = (h2 % 7) as usize;
        println!("banana: {}, apple: {}", idx, idx1);
        println!("{}", h1 == h2);
        assert_eq!(false, h1 == h2);
    }

    #[test]
    fn check_str_key() {
        let mut t = StrTable::new(7);

        t.insert("apple", 3);
        t.insert("banana", 5);
        t.insert("apple", 5);

        println!("==== Test No 5 ====");
        println!("{:?}", t.get("apple"));
        println!("{:?}", t.get("cherry"));
        assert_eq!(Some(5), t.get("apple"));
        assert_eq!(None, t.get("cherry"));

        t.insert("cherry", 7);
        println!("{:?}", t.get("cherry"));
        assert_eq!(Some(7), t.get("cherry"));
    }
}
