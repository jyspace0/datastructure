fn insertion_sort(v: &mut [i32]) {
    // len(): 1..0이라서 반복이 없음, overflow 걱정도 없음
    // 0번은 값 자체라 정렬되어 있어, 1번 부터 시작
    for start in 1..v.len() {
        let mut i = start; // 끼워 넣을 값의 현재 위치
        // 왼쪽에 이웃이 있고, 그 이웃이 나보다 크면 계속 왼쪽으로 이동
        while i > 0 && v[i - 1] > v[i] {
            v.swap(i - 1, i); // 왼쪽과 자리 바꿈
            i -= 1; // 내 위치 이동
        }
    }
}

fn main() {
    let mut data = [5, 1, 4, 2, 8]; // 정렬할 배열
    insertion_sort(&mut data);

    println!("{:?}", data);
}
