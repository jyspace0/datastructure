// 슬라이스에서 target의 위치를 찾는 함수
// &[T]: 읽기만 하므로, &mut이 아니라 그냥 빌림(&)
// T: PartialEq -> 두 값이 "같은지" 비교 (==) 할 수 있어야함
fn linear_search<T: PartialEq>(data: &[T], target: &T) -> Option<usize> {
    // 0번부터 끝까지 인덱스를 하나씩 돔
    for i in 0..data.len() {
        // i번째 값이 찾는 값과 같으면
        if &data[i] == target {
            // 위치 i를 Some으로 감싸서 바로 돌려줌 (함수 종료)
            return Some(i);
        }
    }
    // 끝까지 돌았는데 못 찾았다면, 찾는 값이 없음 (None)을 돌려줌
    None
}

fn find_first_even(data: &[i32]) -> Option<usize> {
    // 0번부터 끝까지 인덱스를 하나씩 돌아요
    for i in 0..data.len() {
        // i위치의 값이 짝수가 맞으면 위치 i를 돌려줌
        if data[i] % 2 == 0 {
            return Some(i);
        }
    }
    // 끝까지 돌아도 짝수가 없으면, None을 돌려줌
    None
}

// 정렬된 슬라이스체서 target의 위치를 찾는 함수
// T: Ord -> 크다/ 작다를 비교할 수 있어야 해요. (정렬도 Ord 필요)
fn binary_search<T: Ord>(data: &[T], target: &T) -> Option<usize> {
    // 후보 범위를 (left, right)로 잡음. right는 "포함하지 않는 끝"
    let mut left = 0;
    let mut right = data.len();

    // 후보가 하나라도 남아 있는 동안 반복
    while left < right {
        // 후보가 하나라도 남아 있는 동안 반복
        let mid = left + (right - left) / 2;

        if data[mid] == *target {
            // target을 찾았어요. 위치를 돌려 줘요
            return Some(mid);
        } else if data[mid] < *target {
            // 가운데 값이 작아요 -> 찾는 값은 오른쪽에 있어요
            // mid + 1에서 부터 right까지 다시 검사
            left = mid + 1;
        } else {
            // 가운데 값이 커요 -> 찾는 값은 왼쪽에 있어요
            right = mid;
        }
    }
    // 후보가 다 사라졌으면
    None
}

fn binary_search_i32(data: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = data.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if data[mid] == target {
            return Some(mid);
        } else if data[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    None
}

// 데이터에 들어갈 위치를 찾는 함수
// fn insert_position(data: &[i32], target: i32) -> Option<usize> {
fn insert_position(data: &[i32], target: i32) -> usize {
    // 데이터가 들어갈 위치 설정 (left, right)
    let mut left = 0;
    let mut right = data.len();

    // 후보군이 하나가 남을때까지 반복
    while left < right {
        let mid = left + (right - left) / 2;

        // if data[mid] == target {
        // 들어갈 값이 존재하면 None을 돌려줌
        // return None;
        // } else
        if data[mid] < target {
            // 들어갈 값이 작으면 -> 들어갈 위치는 오른쪽
            left = mid + 1;
        } else {
            // 들어갈 값이 크면 -> 들어갈 위치는 왼쪽
            right = mid;
        }
    }
    // Some(left)
    left
}

// 데이터에 들어갈 위치를 찾는 함수
// fn insert_position(data: &[i32], target: i32) -> Option<usize> {
fn insert_position2(data: &[i32], target: i32) -> usize {
    let mut left = 0;
    let mut right = data.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if data[mid] <= target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}

fn count_of(data: &[i32], target: i32) -> usize {
    insert_position2(data, target) - insert_position(data, target)
}

fn main() {
    let nums = [7, 3, 9, 4, 1];

    // 9를 찾음
    match linear_search(&nums, &9) {
        Some(idx) => println!("찾았어요! 위치: {}", idx), // 위치: 2
        None => println!("없어요"),
    }

    // 100을 찾음
    match linear_search(&nums, &100) {
        Some(idx) => println!("찾았어요! 위치: {}", idx),
        None => println!("없어요"),
    }

    // 처음 짝수가 있는 곳을 찾아요
    let nums = [1, 3, 8, 5, 6];
    match find_first_even(&nums) {
        Some(idx) => println!("처음 짝수가 있는 위치: {}", idx),
        None => println!("짝수가 없어요!"),
    }

    // 정렬된 데이터에서 9와 4를 찾아요
    let nums = [1, 3, 5, 7, 9, 11, 13];
    println!("{:?}", binary_search(&nums, &9));
    println!("{:?}", binary_search(&nums, &4));

    // 들어갈 위치 찾기
    let nums = [1, 3, 5, 7];
    println!("들어갈 위치= {}", insert_position(&nums, 5));
    println!("들어갈 위치= {}", insert_position(&nums, 4));
    println!("들어갈 위치= {}", insert_position(&nums, 0));
    println!("들어갈 위치= {}", insert_position(&nums, 10));

    // 들어갈 위치 찾기
    let nums = [1, 3, 3, 3, 7];
    println!("들어갈 위치= {}", insert_position(&nums, 3));
    println!("들어갈 위치2= {}", insert_position2(&nums, 3));
    let nums = [1, 3, 7];
    println!("들어갈 위치= {}", insert_position(&nums, 3));
    println!("들어갈 위치2= {}", insert_position2(&nums, 3));

    // 값 찾기
    let nums = [1, 3, 5, 7, 9];
    println!("찾는 값 위치= {:?}", binary_search_i32(&nums, 7));
    println!("찾는 값 위치= {:?}", binary_search_i32(&nums, 4));

    // 값 찾기
    let nums = [];
    println!("찾는 값 위치= {:?}", binary_search_i32(&nums, 7));
    println!("찾는 값 위치= {:?}", binary_search_i32(&nums, 4));

    // 값의 갯수
    let nums = [1, 3, 3, 3, 7];
    println!("찾는 값 개수= {}", count_of(&nums, 3));
    println!("찾는 값 위치= {}", count_of(&nums, 5));
}
