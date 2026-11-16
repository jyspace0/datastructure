/// merge short: 정렬한 데이터를 반으로 분할하여 정렬 후 합침
fn merge_sort(v: &[i32]) -> Vec<i32> {
    // 기저 조건: 값이 0 ~ 1개면 이미 정렬된 상태이므로 복사해서 돌려줌
    if v.len() <= 1 {
        return v.to_vec();
    }

    let mid = v.len() / 2; // 가운데 위치로 가름
    let left = merge_sort(&v[..mid]); // 왼쪽 절반 재귀로 분할
    let right = merge_sort(&v[mid..]); // 오른쪽 절반 재귀로 분할

    merge(&left, &right) // 정렬된 두 절반을 정렬하여 합침
}

// 정렬된 두 슬라이스를 읽기만 하고(&), 합친 새 Vec을 돌려줘요
fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    // 결과를 담을 Vec임. 크기를 미리 잡아두면 더 효율적임
    let mut result = Vec::with_capacity(left.len() + right.len());

    let (mut i, mut j) = (0, 0); // 왼쪽 줄, 오른쪽 줄의 "맨 앞" 위치

    // 두 줄 모두 남아 있는 동안 반복
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]); // 왼쪽이 더 작거나 같으면 왼쪽을 넣어요
            i += 1;
        } else {
            result.push(right[j]); // 오른쪽이 더 작으면 오른쪽을 넣어요
            j += 1;
        }
    }

    // 한쪽이 끝났으니, 남은 쪽을 통째로 뒤에 붙임
    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    result // 합친 결과 돌려줌
}

fn main() {
    let a = [1, 4, 6]; // 정렬된 왼쪽
    let b = [2, 3, 7]; // 정렬된 오른쪽

    println!("{:?}", merge(&a, &b));

    // let data = [5, 1, 4, 2];
    // println!("{:?}", merge_sort(&data));

    let data = [3, 1, 2];
    println!("{:?}", merge_sort(&data));
}
