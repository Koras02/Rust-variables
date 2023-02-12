// fn main() {

// let guess: u32 = "42".parse().expect("Not a number!");
//     println!("Hello, world!");
    
// }

// 부동 소수점 타입
// fn main() {
//     let x = 2.0; // f64

//     let y: f32 = 3.0; // f32
// }

// 수학적 연산들
// fn main() {
//     // addtion 
//     let sum = 5 + 10;

//     // subtraction        
//     let difference = 95.5 - 4.3;

//     // multiplication
//     let product = 4 * 50;

//     // division 
//     let quotitent = 45.5 / 23.1;

//     // remainder
//     let remainer = 50 % 4
// }

// Boolean 타입

// fn main() {
//     let t = true;

//     let f: bool = false; // with explicit type annotation
// }

// 문자 타입
// fn main() {
//     let c = 'z';
//     let z = 'Z';
//     let heart_eyed_cat = '🐈';

// }

// 복합 타입들

// 1. 값들을 집합시켜서 튜플확
// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
// }

// 2. 튜플 구조해체
// fn main() {
//     let tup = (500, 6.4, 1);

//     let (x , y, z) = tup;

//     println!("The value of y is: {}", y);
// }

// 3.튜플 색인 넣기
// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_bundred = x.0;

//     let six_point_four = x.1;

//     let one = x.2;

//     println!("The value of six: {}", six_point_four);
// }

// 4.배열
// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     let months = ["January", "February", "March", "April", "May", "June", "July",
//               "August", "September", "October", "November", "December"];
// }

// 배열 요소에 접근하기
// fn main() {
//     let a = [1,2,3,4,5];

//     let first = a[0];
//     let second = a[1];

// }

// 유효하지 않은 배열 요소에 대한 접근
fn main() {
    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
