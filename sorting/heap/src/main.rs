// i번째 값의 왼쪽 자식 위치
fn left(i: usize) -> usize {
    i * 2 + 1
}

// i번째 값의 오른쪽 자식 위치
fn right(i: usize) -> usize {
    i * 2 + 2
}

// i번째 값의 부모 위치 (0번째 부모는 없음)
fn parent(i: usize) -> usize {
    (i - 1) / 2
}

/// 힙의 크기를 size로 제한, v[..size]만 힙으로 봄
fn shift_down(v: &mut [i32], mut parent: usize, size: usize) {
    loop {
        let l = 2 * parent + 1; // 왼쪽 자식
        let r = 2 * parent + 2; // 오른쪽 자식

        let mut biggest = parent; // 부모, 자식 중 가장 큰 값의 위치

        // 자식 위치가 힙안에 있을 때만 비교 (범위 확인 중요 !!!)
        if l < size && v[l] > v[biggest] {
            biggest = l; // 왼쪽 자식이 더 크면 갱신
        }
        if r < size && v[r] > v[biggest] {
            biggest = r; // 오른쪽 자식이 더 크면 갱신
        }

        if biggest == parent {
            break; // 부모가 제일 크면 규칙이 지켜진 것이라 끝
        }

        v.swap(parent, biggest); // 더 큰 자식과 자리를 바꿈
        parent = biggest; // 내려간 위치에서 계속 확인
    }
}

fn build_heap(v: &mut [i32]) {
    let n = v.len(); // 전체 길이

    // 자식이 있는 부모는 0..n/2까지임 (n/2 이후는 모두 자식 없는 잎)
    for i in (0..n / 2).rev() {
        shift_down(v, i, n); // i번 부모를 알맞는 자리로 이동
    }
}

fn heap_sort(v: &mut [i32]) {
    let n = v.len(); // 전체 길이
    build_heap(v); // 1단계: 먼저 최대 힙으로 만듬

    // end: 힙의 마지막 위치. (맨뒤에 최대값을 옮김)
    for end in (1..n).rev() {
        v.swap(0, end); // 최대값(루트)을 힙의 맨뒤와 바꿈
        shift_down(v, 0, end); // 힙 크기를 end로 줄임
    }
}

fn main() {
    // 이 배열은 "부모 >= 자식" 규칙을 지키는 최대 힙이에요
    let heap = [9, 5, 8, 1, 3, 2];

    println!("루트: {}", heap[0]); // 9
    println!("1번의 자식 {}, {}", heap[left(1)], heap[right(1)]); // 1, 3
    println!("5번의 부모 {}", heap[parent(5)]); // 8

    // 루트의  1이 자식들 보다 작아서 규칙이 깨진 상태
    let mut data = [1, 9, 8, 4, 3, 2];
    shift_down(&mut data, 0, 6); // 0번(루트)를 내려보내요
    println!("{:?}", data);

    let mut data = [3, 1, 4, 9, 5, 2];
    build_heap(&mut data);
    println!("build heap: {:?}", data);

    let mut data = [1, 2, 3, 4];
    build_heap(&mut data);
    println!("build heap: {:?}", data);

    let mut data = [3, 1, 4, 9, 5, 2];
    heap_sort(&mut data);
    println!("heap sort: {:?}", data);

    let mut data = [1, 2, 3, 4];
    heap_sort(&mut data);
    println!("build heap: {:?}", data);
}
