# 2. 変数と型の練習

このガイドでは、Rustの変数と型について学びます。

## 準備：プロジェクトを作成

ターミナルで以下を実行：

```bash
cd /workspaces/Rust-practice
cargo new variables
cd variables
```

---

## 練習1：不変な変数（immutable）

Rustの変数はデフォルトで**変更不可（不変）**です。

### コード

`src/main.rs`を開いて、以下のコードに書き換えてください：

```rust
fn main() {
    let x = 5;
    println!("xの値は: {}", x);
    
    // x = 6;  // エラー！デフォルトでは変更できない
}
```

### 実行

```bash
cargo run
```

### 結果

```
xの値は: 5
```

### 学んだこと

- `let`で変数を宣言
- デフォルトで不変（変更できない）
- これにより安全なコードが書ける

### 試してみよう

コメント（`//`）を外して`x = 6;`を実行してみてください。どんなエラーが出るか確認しましょう。

---

## 練習2：可変な変数（mutable）

変数を変更したい場合は`mut`キーワードを使います。

### コード

```rust
fn main() {
    let mut x = 5;
    println!("最初のxの値は: {}", x);
    
    x = 6;  // mutをつければ変更できる！
    println!("変更後のxの値は: {}", x);
}
```

### 実行

```bash
cargo run
```

### 結果

```
最初のxの値は: 5
変更後のxの値は: 6
```

### 学んだこと

- `mut`（ミュータブル）をつけると変更可能
- 変更する必要がある時だけ使う
- 明示的に書くことでコードが分かりやすくなる

---

## 練習3：データ型

Rustには様々なデータ型があります。

### コード

```rust
fn main() {
    // 整数型
    let age: i32 = 25;
    println!("年齢: {}", age);
    
    // 浮動小数点型
    let height: f64 = 170.5;
    println!("身長: {}cm", height);
    
    // 真偽値型
    let is_student: bool = true;
    println!("学生ですか？: {}", is_student);
    
    // 文字型
    let grade: char = 'A';
    println!("成績: {}", grade);
    
    // 文字列型
    let name: String = String::from("太郎");
    println!("名前: {}", name);
}
```

### 実行

```bash
cargo run
```

### Rustの主な型

| 型 | 説明 | 例 |
|---|---|---|
| `i8`, `i16`, `i32`, `i64` | 符号付き整数 | `42`, `-10` |
| `u8`, `u16`, `u32`, `u64` | 符号なし整数（正の数のみ） | `42`, `100` |
| `f32`, `f64` | 浮動小数点数 | `3.14`, `2.5` |
| `bool` | 真偽値 | `true`, `false` |
| `char` | 文字（シングルクォート） | `'あ'`, `'A'` |
| `String` | 文字列（ダブルクォート） | `String::from("Hello")` |
| `&str` | 文字列スライス | `"Hello"` |

### 学んだこと

- 型を明示的に指定するには`: 型名`を使う
- 数値、文字、真偽値など様々な型がある
- 型を指定することでエラーを防げる

---

## 練習4：型推論

Rustは賢いので、型を書かなくても自動的に推論してくれます。

### コード

```rust
fn main() {
    // 型を書かなくても自動的に推論される
    let x = 42;          // i32と推論
    let y = 3.14;        // f64と推論
    let z = true;        // boolと推論
    let c = 'R';         // charと推論
    let s = "Hello";     // &strと推論
    
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);
    println!("c = {}", c);
    println!("s = {}", s);
}
```

### 実行

```bash
cargo run
```

### 学んだこと

- 多くの場合、型を書かなくてもRustが自動判定
- コードが簡潔になる
- 必要な時だけ型を明示すればOK

---

## 練習5：シャドーイング

同じ変数名を再利用できる機能です。

### コード

```rust
fn main() {
    let x = 5;
    println!("最初のx: {}", x);
    
    let x = x + 1;  // 新しいxを作成
    println!("2回目のx: {}", x);
    
    let x = x * 2;  // また新しいxを作成
    println!("3回目のx: {}", x);
    
    // 型も変えられる！
    let x = "今度は文字列";
    println!("4回目のx: {}", x);
}
```

### 実行

```bash
cargo run
```

### 結果

```
最初のx: 5
2回目のx: 6
3回目のx: 12
4回目のx: 今度は文字列
```

### シャドーイングと可変変数の違い

| | シャドーイング (`let`) | 可変変数 (`mut`) |
|---|---|---|
| 書き方 | `let x = 新しい値;` | `x = 新しい値;` |
| 型の変更 | できる | できない |
| 実態 | 新しい変数を作る | 同じ変数を変更 |

### 学んだこと

- `let`を使うと同じ変数名で新しい変数を作れる
- 型を変えることも可能
- 計算の途中結果を保存するのに便利

---

## 練習6：定数

プログラム全体で使う変更不可な値を定義します。

### コード

```rust
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.14159;

fn main() {
    println!("最大ポイント: {}", MAX_POINTS);
    println!("円周率: {}", PI);
    
    // MAX_POINTS = 200_000;  // エラー！定数は変更不可
}
```

### 実行

```bash
cargo run
```

### 定数のルール

- `const`キーワードを使う
- **必ず型を書く**
- 大文字とアンダースコアで命名（`SCREAMING_SNAKE_CASE`）
- プログラム全体で使える
- 関数の外でも宣言できる

### 定数と不変変数の違い

| | 定数 (`const`) | 不変変数 (`let`) |
|---|---|---|
| 型の指定 | 必須 | 省略可能 |
| スコープ | グローバル可能 | ローカルのみ |
| 命名規則 | 大文字 | 小文字 |
| 計算 | コンパイル時のみ | 実行時も可 |

---

## 練習7：数値リテラルとアンダースコア

大きな数字を読みやすくする方法です。

### コード

```rust
fn main() {
    // アンダースコアで桁を区切れる（見やすい！）
    let million = 1_000_000;
    let decimal = 3.141_592_653;
    
    // 2進数、8進数、16進数も書ける
    let binary = 0b1111_0000;      // 2進数
    let octal = 0o77;               // 8進数
    let hex = 0xff;                 // 16進数
    let byte = b'A';                // バイトリテラル
    
    println!("100万: {}", million);
    println!("円周率: {}", decimal);
    println!("2進数 0b1111_0000 = {}", binary);
    println!("8進数 0o77 = {}", octal);
    println!("16進数 0xff = {}", hex);
    println!("バイト 'A' = {}", byte);
}
```

### 実行

```bash
cargo run
```

### 学んだこと

- `_`で桁を区切ると読みやすい
- 様々な進数で数値を表現できる
- コンパイル後は同じ値になる

---

## 練習問題

自分でコードを書いて試してみましょう！

### 問題1：自己紹介プログラム

```rust
fn main() {
    // 以下の変数を自分の情報に書き換えてください
    let name = "あなたの名前";
    let age = 20;
    let height = 170.5;
    let is_student = true;
    
    println!("=== 自己紹介 ===");
    println!("名前: {}", name);
    println!("年齢: {}歳", age);
    println!("身長: {}cm", height);
    println!("学生: {}", is_student);
}
```

### 問題2：計算プログラム

```rust
fn main() {
    let a = 10;
    let b = 3;
    
    // 以下の計算結果を表示してください
    let sum = a + b;
    let difference = a - b;
    let product = a * b;
    let quotient = a / b;
    let remainder = a % b;
    
    println!("{} + {} = {}", a, b, sum);
    println!("{} - {} = {}", a, b, difference);
    println!("{} × {} = {}", a, b, product);
    println!("{} ÷ {} = {}", a, b, quotient);
    println!("{} % {} = {}", a, b, remainder);
}
```

### 問題3：温度変換プログラム

摂氏から華氏への変換式：`F = C × 9/5 + 32`

```rust
fn main() {
    let celsius = 25.0;
    
    // 華氏に変換してください
    let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    
    println!("摂氏{}度は華氏{}度です", celsius, fahrenheit);
}
```

---

## チャレンジ問題

### タプルを使ってみよう

タプルは複数の値をまとめて扱えます。

```rust
fn main() {
    // タプル：複数の型をまとめる
    let person: (&str, i32, f64) = ("太郎", 25, 170.5);
    
    // 分解して取り出す
    let (name, age, height) = person;
    
    println!("名前: {}", name);
    println!("年齢: {}歳", age);
    println!("身長: {}cm", height);
    
    // インデックスでアクセス
    println!("名前: {}", person.0);
    println!("年齢: {}歳", person.1);
}
```

### 配列を使ってみよう

配列は同じ型の値を複数格納できます。

```rust
fn main() {
    // 配列：同じ型の値を複数
    let numbers = [1, 2, 3, 4, 5];
    
    println!("最初の要素: {}", numbers[0]);
    println!("2番目の要素: {}", numbers[1]);
    
    // 同じ値で初期化
    let zeros = [0; 5];  // [0, 0, 0, 0, 0]
    println!("zeros: {:?}", zeros);
}
```

---

## まとめ

この章で学んだこと：

- ✅ 変数はデフォルトで不変（`let`）
- ✅ `mut`をつけると可変になる
- ✅ 様々な型（i32, f64, bool, char, String）
- ✅ 型推論が便利
- ✅ シャドーイングで変数名を再利用
- ✅ `const`で定数を定義
- ✅ 数値リテラルとアンダースコア
- ✅ タプルと配列

---

## 次のステップ

変数と型の練習が終わったら、次は「3. 関数の練習」に進みましょう！

```bash
cd /workspaces/Rust-practice
cargo new functions
cd functions
```

---

## 参考資料

- [The Rust Book - 変数と可変性](https://doc.rust-jp.rs/book-ja/ch03-01-variables-and-mutability.html)
- [The Rust Book - データ型](https://doc.rust-jp.rs/book-ja/ch03-02-data-types.html)

🦀 Happy Coding!