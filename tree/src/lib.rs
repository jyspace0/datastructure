pub struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    // 새 잎 노드를 만드는 함수
    pub fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    // 빈 노드에 새 값을 넣는다
    pub fn insert(&mut self, value: i32) {
        // 입력 값이 작으면 왼쪽으로
        if value < self.value {
            match &mut self.left {
                // 자식이 있으면 밑으로
                Some(l) => l.insert(value),
                // 자식이 없으면 신규 값을 가진 노드를 단다
                None => self.left = Some(Box::new(Node::new(value))),
            }
        } else if value > self.value {
            match &mut self.right {
                Some(r) => r.insert(value),
                None => self.right = Some(Box::new(Node::new(value))),
            }
        }
    }

    pub fn in_order(&self) {
        if let Some(l) = &self.left {
            l.in_order();
        }
        println!("{}", self.value);

        if let Some(r) = &self.right {
            r.in_order();
        }
    }

    pub fn contains(&self, value: i32) -> bool {
        if value == self.value {
            true
        } else if value < self.value {
            match &self.left {
                Some(l) => l.contains(value),
                None => false,
            }
        } else {
            match &self.right {
                Some(r) => r.contains(value),
                None => false,
            }
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn check_insert() {
        let mut root = Node::new(3);

        root.insert(2);
        root.insert(5);
        root.insert(1);
        root.insert(4);
        root.insert(6);
        root.insert(7);
        println!("루트 왼쪽: {}", root.left.as_ref().unwrap().value);
        root.in_order();
        assert_eq!(2, root.left.as_ref().unwrap().value);
    }

    #[test]
    fn check_contains() {
        let mut root = Node::new(3);

        root.insert(2);
        root.insert(5);
        root.insert(1);
        root.insert(4);
        root.insert(6);
        root.insert(7);

        println!("{}", root.contains(6));
        println!("{}", root.contains(8));
        println!("{}", root.contains(1));
        assert_eq!(true, root.contains(6));
        assert_eq!(false, root.contains(8));
        assert_eq!(true, root.contains(1));
    }
}
