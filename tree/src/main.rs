// 트리의 한 칸(노드)을 나타내는 구조체
struct Node {
    value: i32,               // 이 노드가 들고 있는 값
    left: Option<Box<Node>>,  //왼쪽자식 : 없으면 None, 있으면 Some(상자)
    right: Option<Box<Node>>, // 오른쪽 자식: 업으면 None, 있으면 Some(상자)
}

impl Node {
    // &self: 노드를 빌려서 읽기만 한다 (소유권을 가져가지 않음)
    // 중위 순회: 왼쪽 -> 나 -> 오른쪽
    fn in_order(&self) {
        // 1) 왼쪽 자식이 있으면, 그 자식에게 똑같이 순회를 시킨다.
        if let Some(l) = &self.left {
            l.in_order(); // 재귀호출: 왼쪽 아래로 내려감
        }

        // 2) 나 자신을 방문한다.
        println!("{}", self.value);

        // 3) 오른쪽 자식이 있으면, 똑같이 순회를 시킨다.
        if let Some(r) = &self.right {
            r.in_order(); // 재귀호출: 오른쪽 아래로 내려감
        }
    }

    // 전위 순회: 나 -> 왼쪽 -> 오른쪽
    fn pre_order(&self) {
        // 2) 나 자신을 방문한다.
        println!("{}", self.value);

        // 1) 왼쪽 자식이 있으면, 그 자식에게 똑같이 순회를 시킨다.
        if let Some(l) = &self.left {
            l.pre_order(); // 재귀호출: 왼쪽 아래로 내려감
        }

        // 3) 오른쪽 자식이 있으면, 똑같이 순회를 시킨다.
        if let Some(r) = &self.right {
            r.pre_order(); // 재귀호출: 오른쪽 아래로 내려감
        }
    }

    // 후위 순회: 왼쪽 -> 오른쪽 -> 나
    fn post_order(&self) {
        // 1) 왼쪽 자식이 있으면, 그 자식에게 똑같이 순회를 시킨다.
        if let Some(l) = &self.left {
            l.post_order(); // 재귀호출: 왼쪽 아래로 내려감
        }

        // 3) 오른쪽 자식이 있으면, 똑같이 순회를 시킨다.
        if let Some(r) = &self.right {
            r.post_order(); // 재귀호출: 오른쪽 아래로 내려감
        }

        // 2) 나 자신을 방문한다.
        println!("{}", self.value);
    }
}

fn main() {
    let left_right = Node {
        value: 5,
        left: None,
        right: None,
    };

    let left = Node {
        value: 1,
        left: None,
        right: Some(Box::new(left_right)),
    };
    let right_left = Node {
        value: 4,
        left: None,
        right: None,
    };

    let right = Node {
        value: 3,
        left: Some(Box::new(right_left)),
        right: None,
    };

    // 부모노드: 자식을 Box::new로 힙에 올린 뒤 Some으로 감싼다.
    let root = Node {
        value: 2,
        left: Some(Box::new(left)),
        right: Some(Box::new(right)),
    };

    println!("루트: {}", root.value);

    if let Some(l) = &root.left {
        println!("왼쪽 자식: {}", l.value);
    }

    if let Some(r) = &root.right {
        println!("오른쪽 자식: {}", r.value);
    }

    if let Some(left) = &root.left {
        if let Some(lr) = &left.right {
            println!("왼쪽(1)의 오른쪽 자식: {}", lr.value);
        }
    }

    if let Some(right) = &root.right {
        if let Some(rl) = &right.left {
            println!("오른쪽(3)의 왼쪽 자식: {}", rl.value);
        }
    }

    // 트리 순회: 중위
    println!("중위 순회");
    root.in_order();

    // 트리 순회: 전위
    println!("전위 순회");
    root.pre_order();

    // 트리 순회: 후위
    println!("후위 순회");
    root.post_order();

    let n5 = Node {
        value: 5,
        left: None,
        right: None,
    };
    let n4 = Node {
        value: 4,
        left: None,
        right: None,
    };
    let n1 = Node {
        value: 1,
        left: Some(Box::new(n5)),
        right: None,
    };
    let n3 = Node {
        value: 3,
        left: Some(Box::new(n4)),
        right: None,
    };
    let root2 = Node {
        value: 2,
        left: Some(Box::new(n1)),
        right: Some(Box::new(n3)),
    };

    println!("==========2번째 ================");
    // 트리 순회: 중위
    println!("중위 순회");
    root2.in_order();

    // 트리 순회: 전위
    println!("전위 순회");
    root2.pre_order();

    // 트리 순회: 후위
    println!("후위 순회");
    root2.post_order();

    // 트리는 2의
    // 왼쪽이 1(오른쪽에 5),
    // 오른쪽이 3(왼쪽에 4, 오른쪽에 6)이에요.
    let n7 = Node {
        value: 7,
        left: None,
        right: None,
    };
    let n6 = Node {
        value: 6,
        left: None,
        right: Some(Box::new(n7)),
    };
    let n4 = Node {
        value: 4,
        left: None,
        right: None,
    };
    let n3 = Node {
        value: 5,
        left: Some(Box::new(n4)),
        right: Some(Box::new(n6)),
    };
    let n5 = Node {
        value: 1,
        left: None,
        right: None,
    };
    let n1 = Node {
        value: 2,
        left: Some(Box::new(n5)),
        right: None,
    };
    let root3 = Node {
        value: 3,
        left: Some(Box::new(n1)),
        right: Some(Box::new(n3)),
    };

    println!("==========3번째 ================");
    // 트리 순회: 중위
    println!("중위 순회");
    root3.in_order();

    // 트리 순회: 전위
    println!("전위 순회");
    root3.pre_order();

    // 트리 순회: 후위
    println!("후위 순회");
    root3.post_order();
}
