/*
ゴール：
1. データをコピーしなくても、greetを呼び出せるようにしてください
2. "Hello dear rustecians"ではなく、"Hello rustecians"となるように、
   2回目のgreetの呼び出しを行なってください
3. 2をスライスを使って実現してください
*/

fn main() {
    let name = format!("dear rustecians");
    greet(name.clone());
    greet(name);
}

fn greet(name: String) {
    println!("Hello {}", name);
}
