/// Bubble Sort : 이웃한 쌍끼지 비교해서 순서가 틀리면 서로 바꾼다
///               한 번 비교할 때마다 가장 큰 값이 맨뒤에 확정됨
/// - 슬라이스를 가변으로 빌려서(&mut) 값을 바꿀 수 있음
/// - 바깥 loop 범위 : 0..len()-1 직전까지 (이웃 비교, usize 음수 방지)
/// - 안쪽 loop 범위 : (0..len()-1) - 바깥 비교 index, (재 비교 방지)
/// - 이웃한 값 비교 : [i] 와 [i+1] 비교, 순서 틀리면 swap
///    
fn bubble_sort(v: &mut [i32]) {
    let n = v.len(); // 전체 길이

    // 길이 0은 정렬할 데이터 없음, 길이 1은 이미 정렬 완료
    if n < 2 {
        return;
    }

    for pass in 0..n - 1 {
        let mut swapped = false; // 바꾼 적이 있는지 기재 (매 pass마다 false로 시작)
        // 뒤쪽 pass개는 이미 확정이므로 그 앞까지만 비교
        for i in 0..n - 1 - pass {
            // 이웃한 쌍은 (0, 1), (1, 2), ... 이므로 v[i], v[i+1] 비교
            // 현재 값이 뒤의 값보다 크면, 자리를 바꿈
            if v[i] > v[i + 1] {
                // 두 자리의 값을 바꾼다 (index)
                v.swap(i, i + 1);
                swapped = true; // "바꿨다"고 표시
            }
        }
        if !swapped {
            break; // 한번도 안 바꿨다면 정렬 완료이니 반복문 빠져 나감
        }
    }
}

fn main() {
    let mut data = [5, 1, 4, 2, 8]; // 정렬할 배열
    bubble_sort(&mut data); // 한번만 정렬

    println!("{:?}", data); // 정렬한 결과
    println!();

    let mut data = [4, 3, 2, 1]; // 정렬할 배열
    bubble_sort(&mut data); // 한번만 정렬

    println!("{:?}", data); // 정렬한 결과

    let mut data = [1, 2, 4, 3, 5]; // 정렬할 배열
    bubble_sort(&mut data); // 한번만 정렬

    println!("{:?}", data); // 정렬한 결과

    let mut data = [1, 2, 3, 4, 5]; // 정렬할 배열
    bubble_sort(&mut data); // 한번만 정렬

    println!("{:?}", data); // 정렬한 결과
}
