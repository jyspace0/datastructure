use std::ptr::NonNull;

/// 양방향 링크드 리스트
/// - DList는 전체 리스트 구조체로 head, tail, len을 갖는다.
/// - DNode는 노드 구조체로 elem, next, prev를 갖는다.
/// - 연결은 DLink Type으로 Option<NonNull<DNode<T>>>이다.
struct DList<T> {
    head: DLink<T>,
    tail: DLink<T>.
    len: usize,
}

type DLink<T> = Option<NonNull<DNode<T>>>;
 
struct DNode<T> {
    elem: T,
    next: DLink<T>,
    prev: DLink<T>,
}

fn main() {
    let mut list: DList<i32> = DList::new();
}
