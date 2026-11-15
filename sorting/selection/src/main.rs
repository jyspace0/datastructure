fn selection_sort(v: &mut [i32]) {
    let n = v.len(); // 전체 길이

    if n < 2 {
        return; // 길이가 0이면 비었거나, 1이면 이미 정렬된 상태
    }

    // start: 아직 정렬 안된 구간의 시작 위치
    for start in 0..n - 1 {
        let mut min_idx = start; // 최소값의 위치를 start로 가정
        for i in start + 1..n {
            if v[i] < v[min_idx] {
                min_idx = i;
            }
        }
        v.swap(start, min_idx); // 최소값과 자리 맞바꿈
    }
}

fn main() {
    let mut data = [5, 1, 4, 2, 8]; // 정렬할 배열
    selection_sort(&mut data);

    println!("{:?}", data); // [1, 2, 4, 5, 8]

    let mut data = [1, 2, 5, 3, 8]; // 정렬할 배열
    selection_sort(&mut data); // 한 단계만 실행

    println!("{:?}", data); // [1, 2, 3, 5, 8]
}
