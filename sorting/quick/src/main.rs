// 마지막 값을 피벗으로 삼아 분할하고, 피벗의 최종 위치를 돌려줌
fn partition(v: &mut [i32]) -> usize {
    let last = v.len() - 1; // 피벗 (마지막 값)의 위치 (빈 슬라이스는 안됨)
    let mut store = 0; // 피벗보다 작은 값 구역의 끝 (다음 빈자리)
    for i in 0..last {
        // 피벗보다 작은 값을 만나면
        if v[i] < v[last] {
            v.swap(i, store); // 작은 값 구역의 끝으로 보냄
            store += 1;
        }
    }
    v.swap(store, last); // 피벗을 작은 값 구역 바로 뒤, 제자리에 놓음
    store
}

fn quick_sort(v: &mut [i32]) {
    // 기저 조건: 길이가 0 ~ 1개면 이미 정렬된 상태 (partition 빈슬롯 방지)
    if v.len() <= 1 {
        return;
    }

    // let p = partition(v); // 피벗이 p 위치에 확정됨
    let p = partition_mid(v); // 피벗이 p 위치에 확정됨
    quick_sort(&mut v[..p]);
    quick_sort(&mut v[p + 1..]);
}

fn partition_mid(v: &mut [i32]) -> usize {
    let mid = v.len() / 2;
    let last = v.len() - 1;
    v.swap(mid, last);
    partition(v)
}

fn main() {
    let mut data = [5, 1, 4, 2, 3];
    let p = partition(&mut data);
    println!("after partition: {:?}, 피벗 위치: {}", data, p);

    let mut data = [5, 1, 4, 2, 3];
    quick_sort(&mut data);
    println!("after sorting: {:?}", data);

    let mut data = [1, 2, 3, 4, 5];
    quick_sort(&mut data);
    println!("after sorting: {:?}", data);

    let mut data = [5, 3, 2, 1, 4];
    quick_sort(&mut data);
    println!("after sorting: {:?}", data);
}
