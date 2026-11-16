/// T는 순서를 비교(Ord)할 수 있고, 복사(Clone)할 수 있어야 한다.
fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len()); // 결과를 담을 Vec

    let (mut i, mut j) = (0, 0); // 두 배열의 맨 앞 위치
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i].clone()); // 왼쪽 값을 복사해 결과에 담는다
            i += 1; // 왼쪽을 한 칸 이동
        } else {
            result.push(right[j].clone()); // 오른쪽 값을 복사해 결과에 담는다
            j += 1; // 오른쪽을 한 칸 이동
        }
    }

    // 한쪽이 끝났으니, 남은 쪽을 통째로 뒤에 붙인다
    result.extend_from_slice(&left[i..]); // 왼쪽 남은 것 붙이기
    result.extend_from_slice(&right[j..]); // 오른쪽 남은 것 붙이기
    result
}

fn merge_sort<T: Ord + Clone>(v: &[T]) -> Vec<T> {
    if v.len() <= 1 {
        return v.to_vec(); // 기저 조건: 복사해서 리턴
    }

    let mid = v.len() / 2; // 가운데 위치
    let left = merge_sort(&v[..mid]); // 왼쪽 절반 재귀 정렬
    let right = merge_sort(&v[mid..]); // 오른쪽 절반 재귀 정렬

    merge(&left, &right)
}

/// 신규 Vec 생성하지 않고, &mut [T]로 정렬해서 되돌리기
/// v[..mid], v[mid..]는 각각 정렬되어 있음, v 전체를 정렬상태로 만듬
fn merge_into<T: Ord + Clone>(v: &mut [T], mid: usize) {
    let left = v[..mid].to_vec(); // 왼쪽 복사
    let right = v[mid..].to_vec(); // 오른쪽 복사

    let (mut i, mut j) = (0, 0); // 두 복사 버퍼의 맨 앞 위치
    let mut k = 0; // v에 써 넣을 위치

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            v[k] = left[i].clone();
            i += 1;
        } else {
            v[k] = right[j].clone();
            j += 1;
        }
        k += 1; // 다음에 넣을 위치
    }

    // 왼쪽 남은 값 넣기
    while i < left.len() {
        v[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    // 오른쪽 남은 값 넣기
    while j < right.len() {
        v[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

fn merge_into_sort<T: Ord + Clone>(v: &mut [T]) {
    // 기저 조건: 값이 0 ~ 1개면 이미 정렬된 상태
    if v.len() <= 1 {
        return;
    }

    let mid = v.len() / 2;
    merge_into_sort(&mut v[..mid]);
    merge_into_sort(&mut v[mid..]);

    merge_into(v, mid);
}

fn main() {
    let nums = [5, 1, 4, 2, 7, 9];
    let words = ["pear", "apple", "fig", "mango", "bannana"];

    println!("nums sorting: {:?}", merge_sort(&nums));
    println!("words sorting: {:?}", merge_sort(&words));

    let mut data = [1, 4, 6, 2, 3, 7];
    merge_into(&mut data, 3);
    println!("merge into: {:?}", data);

    let mut data = [5, 1, 4, 2, 8];
    merge_into_sort(&mut data);
    println!("merge into sort: {:?}", data);
}
