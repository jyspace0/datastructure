struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    fn new() -> Self {
        Self { root: None }
    }

    // 이진 탐색할 수 있도록 값을 넣는다 (동일 값 배제)
    fn insert(&mut self, value: i32) {
        match &mut self.root {
            Some(node) => node.insert(value),
            None => self.root = Some(Box::new(Node::new(value))),
        }
    }

    // 값이 있는지 확인
    fn contains(&self, value: i32) -> bool {
        match &self.root {
            Some(node) => node.contains(value),
            None => false,
        }
    }

    // 중위 순회 출력
    fn in_order(&self) {
        match &self.root {
            Some(node) => node.in_order(),
            None => return,
        }
    }

    // 전위 순회 출력
    fn pre_order(&self) {
        match &self.root {
            Some(node) => node.pre_order(),
            None => (),
        }
    }

    // 후위 순회 출력
    fn post_order(&self) {
        if let Some(node) = &self.root {
            node.post_order();
        }
    }

    // 최소값 구하기 (빈 트리: None)
    fn min(&self) -> Option<i32> {
        match &self.root {
            Some(node) => Some(node.min()),
            None => None,
        }
    }

    // 최대값 구하기 (빈트리: None)
    fn max(&self) -> Option<i32> {
        match &self.root {
            Some(node) => Some(node.max()),
            None => None,
        }
    }

    // 칸 삭제하기
    fn delete(&mut self, value: i32) {
        delete(&mut self.root, value);
    }
}

struct Node {
    value: i32,               // 이 노드가 가지고 있는 값
    left: Option<Box<Node>>,  // 왼쪽 자식 (없으면 None)
    right: Option<Box<Node>>, // 오른쪽 자식 (없으면 None)
}

impl Node {
    // 신규 Node 생성
    fn new(value: i32) -> Node {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    // 이진 탐색할 수 있도록 값을 넣는다 (동일 값 배제)
    fn insert(&mut self, value: i32) {
        // 들어갈 값이 작으면 왼쪽자식으로 이동
        if value < self.value {
            match &mut self.left {
                // 자식이 있으면 재귀로 넣을 위치 재 탐색
                Some(l) => l.insert(value),
                // 자식이 없으면 새로운 값을 넣는다
                None => self.left = Some(Box::new(Node::new(value))),
            }
        // 들어갈 값이 크면 오른쪽자식으로 이동 (같을 경우 아무것도 안함)
        } else if value > self.value {
            match &mut self.right {
                // 자식이 있으면 재귀로 넣을 위치 재 탐색
                Some(r) => r.insert(value),
                // 자식이 없으면 새로운 값을 넣는다
                None => self.right = Some(Box::new(Node::new(value))),
            }
        }
    }

    // 전위 순회 (pre-order: 나 -> 왼쪽 -> 오른쪽)
    fn pre_order(&self) {
        println!("{}", self.value); // 나 먼저 출력

        // 왼쪽 자식 출력
        if let Some(l) = &self.left {
            l.pre_order();
        }

        // 오른쪽 자식 출력
        if let Some(r) = &self.right {
            r.pre_order();
        }
    }

    // 중위 순회 (in-order: 왼쪽 -> 나 -> 오른쪽)
    fn in_order(&self) {
        // 왼쪽 자식 출력
        if let Some(l) = &self.left {
            l.in_order();
        }

        // 나 출력
        println!("{}", self.value);

        // 오른쪽 자식 출력
        if let Some(r) = &self.right {
            r.in_order();
        }
    }

    // 후위 순회 (post-order: 왼쪽 -> 오른쪽 -> 나)
    fn post_order(&self) {
        // 왼쪽 자식 출력
        if let Some(l) = &self.left {
            l.post_order();
        }

        // 오른쪽 자식 출력
        if let Some(r) = &self.right {
            r.post_order();
        }

        // 나 출력
        println!("{}", self.value);
    }

    // 값이 있는지 확인
    fn contains(&self, value: i32) -> bool {
        if value == self.value {
            true
        } else if value < self.value {
            // 작으면 왼쪽 자식으로 이동 검사
            match &self.left {
                Some(l) => l.contains(value),
                None => false,
            }
        } else {
            match &self.right {
                // 크면 오른쪽 자식으로 이동 검사
                Some(r) => r.contains(value),
                None => false,
            }
        }
    }

    // 최소값 구하기
    fn min(&self) -> i32 {
        match &self.left {
            Some(l) => l.min(),
            None => self.value,
        }
    }

    // 최대값 구하기
    fn max(&self) -> i32 {
        match &self.right {
            Some(r) => r.max(),
            None => self.value,
        }
    }
}

// 칸 (slot)을 &mut로 받아 value를 삭제한다
fn delete(slot: &mut Option<Box<Node>>, value: i32) {
    match slot {
        None => {} // 빈칸: 지울게 없음
        Some(node) => {
            if value < node.value {
                delete(&mut node.left, value);
            } else if value > node.value {
                delete(&mut node.right, value);
            } else {
                // 찾음: 자식 수에 따른 처리
                match (node.left.take(), node.right.take()) {
                    // 자식이 없으면, slot 삭제
                    (None, None) => *slot = None,
                    // 왼쪽 자식만 있으면, slot을 왼쪽 자식이 대신
                    (Some(l), None) => *slot = Some(l),
                    // 오른쪽 자식만 있으면, slot을 오른쪽 자식이 대신
                    (None, Some(r)) => *slot = Some(r),
                    // 자식이 둘 있는 경우 처리 필요
                    (Some(l), Some(r)) => {
                        node.left = Some(l);
                        node.right = Some(r);
                        let min = node.right.as_ref().unwrap().min();
                        node.value = min;
                        delete(&mut node.right, min);
                        // if let Some(n) = &node.right {
                        //     node.value = n.min();
                        // }
                        // delete(&mut node.right, node.value);
                    }
                }
            }
        }
    }
}

fn main() {
    let mut tree = Tree::new();

    println!("{}", tree.contains(3));

    tree.insert(3);
    tree.insert(2);
    println!("{}", tree.contains(3));

    tree.insert(5);
    tree.insert(1);
    tree.insert(4);
    tree.insert(6);
    tree.insert(7);

    println!("in_order 순회");
    tree.in_order();

    println!("pre_order 순회");
    tree.pre_order();

    println!("post_order 순회");
    tree.post_order();

    println!("최소값: {:?}", tree.min());
    println!("최대값: {:?}", tree.max());

    let mut tree = Tree::new();
    println!("{:?}", tree.max());

    tree.insert(3);
    tree.insert(7);
    tree.insert(5);
    println!("{:?}", tree.max());

    let mut a: Option<i32> = Some(7);
    let b = a.take();
    let c = a.take();
    println!("{:?} {:?} {:?}", a, b, c);

    println!("\n");
    println!("delete 연습");
    let mut root = Some(Box::new(Node::new(3)));

    if let Some(n) = &mut root {
        n.insert(2);
        n.insert(5);
    }

    println!("2 삭제 -> 3, 5 출력");
    delete(&mut root, 2);
    if let Some(n) = &root {
        println!("{}", n.value);
        if let Some(l) = &n.left {
            println!("{:?}", l.value);
        }
        if let Some(r) = &n.right {
            println!("{:?}", r.value);
        }
    }

    println!("재 점검");
    let mut root = Some(Box::new(Node::new(3)));

    println!("3 삭제: 5, 2 출력");
    if let Some(n) = &mut root {
        n.insert(2);
        n.insert(5);
    }
    delete(&mut root, 3);
    if let Some(n) = &root {
        println!("{}", n.value);
        if let Some(l) = &n.left {
            println!("{:?}", l.value);
        }
        if let Some(r) = &n.right {
            println!("{:?}", r.value);
        }
    }

    println!("\n =================\n");
    let mut root = Tree::new();

    root.insert(3);
    root.insert(2);
    root.insert(5);
    root.insert(4);
    root.insert(6);
    root.in_order();

    println!("\n =================\n");
    delete(&mut root.root, 3);
    root.in_order();

    println!("\n ======= end1 ==========\n");
    let mut root = Tree::new();

    root.insert(3);
    root.insert(2);
    root.insert(5);
    root.insert(4);
    root.insert(6);
    root.in_order();

    println!("\n ======= end1 삭제 후 ==========\n");
    root.delete(3);
    root.in_order();

    println!("\n ======= end2 ==========\n");
    let mut root = Tree::new();
    root.insert(5);
    root.insert(3);
    root.insert(8);
    root.insert(1);
    root.insert(4);
    root.insert(7);
    root.insert(9);
    root.in_order();
    println!("\n ======= end2 삭제 후 ==========\n");
    root.delete(5);
    root.in_order();
}
