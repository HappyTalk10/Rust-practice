# Rust学習ガイド

## Rustとは？

Rustは、**安全性**、**速度**、**並行性**を重視したシステムプログラミング言語です。2015年にバージョン1.0がリリースされ、Mozillaによって開発されました。

### Rustの特徴

**1. メモリ安全性**
- ガベージコレクタなしでメモリ安全を保証
- コンパイル時にメモリエラーを検出
- バッファオーバーフローやヌルポインタ参照を防ぐ

**2. 高速性**
- C/C++に匹敵する実行速度
- ゼロコスト抽象化
- 効率的なメモリ使用

**3. 並行処理の安全性**
- データ競合をコンパイル時に防ぐ
- 安全なマルチスレッドプログラミング

**4. 優れた開発体験**
- 親切なエラーメッセージ
- 強力なパッケージマネージャ（Cargo）
- 充実したドキュメント

## なぜRustを学ぶのか？

### 1. 需要の高まり

- Stack Overflow調査で**最も愛される言語**に8年連続選出（2016-2023）
- Microsoft、Amazon、Google、Meta等の大企業が採用
- システムプログラミング、組み込み、Web開発など幅広い分野で利用

### 2. キャリアの選択肢が広がる

**活用分野：**
- オペレーティングシステム開発
- Webブラウザエンジン（Firefox等）
- ゲームエンジン
- ブロックチェーン
- IoT・組み込みシステム
- WebAssembly
- CLI（コマンドラインツール）
- バックエンドAPI開発

### 3. プログラミングの本質的理解

Rustを学ぶことで以下の概念が深く理解できます：

- **所有権（Ownership）** - メモリ管理の仕組み
- **型システム** - 安全なプログラム設計
- **並行処理** - マルチスレッドプログラミング
- **ライフタイム** - リソース管理

これらの知識は他の言語にも応用できる普遍的なスキルです。

### 4. セキュリティとパフォーマンスの両立

- メモリ安全性のバグはセキュリティ脆弱性の約70%を占める
- Rustはこれらを言語レベルで防止
- 高速性を犠牲にせず安全性を実現

## 学習の進め方

### 1. Hello World

```bash
cargo new hello_world
cd hello_world
cargo run
```

### 2. 変数と型

```bash
cargo new variables
cd variables
# src/main.rsを編集
cargo run
```

### 3. 関数

```bash
cargo new functions
cd functions
# src/main.rsを編集
cargo run
```

### 4. 制御構文

```bash
cargo new control_flow
cd control_flow
# if、loop、while、forなどを練習
cargo run
```

### 5. 所有権（最重要！）

```bash
cargo new ownership
cd ownership
# Rustの核心概念を理解する
cargo run
```

## プロジェクト一覧

作成したプロジェクトをここにリストアップしていきましょう。

- [ ] hello_world - 最初のプログラム
- [ ] variables - 変数と型の練習
- [ ] functions - 関数の練習
- [ ] control_flow - 制御構文の練習
- [ ] ownership - 所有権の理解
- [ ] structs - 構造体の練習
- [ ] enums - 列挙型の練習
- [ ] error_handling - エラー処理
- [ ] collections - コレクション（Vec, HashMap等）
- [ ] traits - トレイトの練習

## 参考資料

### 日本語リソース
- [The Rust Programming Language (日本語版)](https://doc.rust-jp.rs/book-ja/) - 公式の入門書
- [Rust by Example (日本語版)](https://doc.rust-jp.rs/rust-by-example-ja/) - 実例で学ぶ
- [公式ドキュメント](https://www.rust-lang.org/ja)

### 英語リソース
- [The Rust Book](https://doc.rust-lang.org/book/) - 公式入門書（原文）
- [Rustlings](https://github.com/rust-lang/rustlings) - 対話的な学習ツール
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

## 学習のコツ

1. **コンパイラと友達になる** - エラーメッセージをよく読む
2. **小さく始める** - 完璧を求めず、まず動くコードを書く
3. **所有権を理解する** - Rustの最も重要な概念
4. **公式ドキュメントを活用** - 非常に充実している
5. **実際に手を動かす** - 読むだけでなく、書いて試す

## メモ

学習中に気づいたことや、覚えておきたいことをここに記録しましょう。

---

🦀 Happy Coding with Rust!