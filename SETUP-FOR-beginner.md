# 初心者向け：GitHub CodespacesでRustを始めるガイド

このガイドでは、プログラミング初心者の方でも、GitHub CodespacesでRustの開発環境を作る方法を丁寧に説明します。

## 必要なもの

- GitHubアカウント（無料で作成できます）
- インターネット接続
- Webブラウザ（Chrome、Firefox、Edgeなど）

**それだけです！**パソコンに何もインストールする必要はありません。

---

## ステップ1：GitHubアカウントを作成（持っていない場合）

1. https://github.com にアクセス
2. 「Sign up」をクリック
3. メールアドレス、パスワード、ユーザー名を入力
4. メールで届く認証コードを入力

---

## ステップ2：新しいリポジトリを作成

### 2-1. リポジトリ作成ページを開く

1. GitHubにログイン
2. 右上の **「+」ボタン** をクリック
3. **「New repository」** を選択

### 2-2. リポジトリの設定

以下のように入力してください：

- **Repository name**（リポジトリ名）: `rust-practice`
  - 好きな名前でOKですが、英数字とハイフンのみ使えます
  - 日本語は使えません

- **Description**（説明）: `Rustの学習用リポジトリ`
  - 空欄でもOKです

- **Public / Private**:
  - **Public**: 誰でも見られる（推奨）
  - **Private**: 自分だけが見られる

- ✅ **「Add a README file」に必ずチェック**
  - これがないとCodespacesが起動しにくいです

- 「Add .gitignore」と「Choose a license」は選択不要

### 2-3. リポジトリを作成

緑色の **「Create repository」** ボタンをクリック

→ リポジトリが作成されました！🎉

---

## ステップ3：Codespacesを起動

### 3-1. Codespacesを開く

1. 作成したリポジトリのページで、緑色の **「<> Code」** ボタンをクリック
2. **「Codespaces」** タブをクリック
3. **「Create codespace on main」** をクリック

### 3-2. 起動を待つ

- 初回は1〜2分かかります
- 「Setting up your codespace...」と表示されます
- ブラウザ内でVSCode（コードエディタ）が開きます

**画面の見方：**
- 左側：ファイル一覧
- 中央：コードを書く場所
- 下部：ターミナル（黒い画面）

---

## ステップ4：Dev Containerを設定（次回から自動セットアップ）

この設定をすると、次回から自動的にRust環境が整います。

### 4-1. .devcontainerフォルダを作成

1. 左側のファイル一覧の空白部分を **右クリック**
2. **「New Folder」** を選択
3. `.devcontainer` と入力（最初の`.`を忘れずに！）
4. Enterキーを押す

### 4-2. devcontainer.jsonファイルを作成

1. 作成した `.devcontainer` フォルダを **右クリック**
2. **「New File」** を選択
3. `devcontainer.json` と入力
4. Enterキーを押す

### 4-3. 設定内容を貼り付け

開いた`devcontainer.json`ファイルに、以下をコピー＆ペーストしてください：

```json
{
  "name": "Rust",
  "image": "mcr.microsoft.com/devcontainers/rust:latest",
  "customizations": {
    "vscode": {
      "extensions": [
        "rust-lang.rust-analyzer"
      ]
    }
  }
}
```

### 4-4. ファイルを保存

- Windowsの場合: **Ctrl + S**
- Macの場合: **Cmd + S**

または、メニューの「File」→「Save」

### 4-5. GitHubに保存（コミット＆プッシュ）

下部のターミナルで、以下のコマンドを1行ずつコピー＆ペーストして、Enterキーを押してください：

```bash
git add .devcontainer/devcontainer.json
```

```bash
git commit -m "Add dev container for Rust"
```

```bash
git push
```

**成功メッセージが表示されればOK！**

---

## ステップ5：Rustをインストール（初回のみ）

### 5-1. インストールコマンドを実行

ターミナル（下部の黒い画面）で、以下をコピー＆ペーストしてEnterキーを押してください：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
```

インストールには1〜2分かかります。

### 5-2. 環境を読み込む

続けて以下を実行：

```bash
source $HOME/.cargo/env
```

### 5-3. 確認

以下を実行して、バージョン番号が表示されればOK：

```bash
rustc --version
```

例: `rustc 1.XX.X` のように表示されます

---

## ステップ6：最初のRustプログラムを作る

### 6-1. プロジェクトを作成

ターミナルで以下を実行：

```bash
cargo new hello_rust
```

### 6-2. プロジェクトフォルダに移動

```bash
cd hello_rust
```

### 6-3. プログラムを実行

```bash
cargo run
```

**「Hello, world!」と表示されれば大成功！** 🎉

---

## ステップ7：コードを編集してみる

### 7-1. ファイルを開く

左側のファイル一覧で：
1. **`hello_rust`** フォルダをクリック
2. **`src`** フォルダをクリック
3. **`main.rs`** ファイルをクリック

### 7-2. コードを変更

中央の画面に表示されているコードを以下のように変更してみましょう：

```rust
fn main() {
    println!("こんにちは、Rustの世界へようこそ！");
    println!("これが私の最初のRustプログラムです！");
}
```

### 7-3. 保存して実行

1. ファイルを保存（Ctrl+S または Cmd+S）
2. ターミナルで実行：

```bash
cargo run
```

変更した内容が表示されれば成功です！

---

## よくあるトラブルと解決方法

### Q1. ターミナルが表示されない

**解決方法：**
- 上部メニューの「View」→「Terminal」をクリック
- または、Ctrl+` (バッククォート) を押す

### Q2. 「git index.lock」エラーが出る

**解決方法：**

```bash
cd /workspaces/rust-practice
rm -f .git/index.lock
```

その後、再度git操作を実行してください。

### Q3. Codespacesが重い・遅い

**解決方法：**
- ブラウザのタブを減らす
- 他のアプリケーションを閉じる
- 少し待ってから操作する

### Q4. コマンドを間違えた

**解決方法：**
- ターミナルで **Ctrl+C** を押すと、実行中の処理を中断できます
- 間違えてもパソコンが壊れることはないので安心してください

---

## 次のステップ

セットアップが完了したら、`STUDY.md`を読んで学習を進めましょう！

### おすすめの学習順序

1. **変数と型** - データの扱い方を学ぶ
2. **関数** - コードを整理する方法
3. **制御構文** - if、loop、forなど
4. **所有権** - Rustの核心概念（最重要！）
5. **構造体とEnum** - データを構造化する
6. **エラー処理** - 安全なプログラムの書き方

---

## 便利なショートカット

### ファイル操作
- **Ctrl+S** (Cmd+S): 保存
- **Ctrl+N** (Cmd+N): 新規ファイル

### ターミナル操作
- **Ctrl+C**: 実行中の処理を中断
- **↑キー**: 前のコマンドを表示
- **Ctrl+L**: ターミナルをクリア

### コード編集
- **Ctrl+Z** (Cmd+Z): 元に戻す
- **Ctrl+/** (Cmd+/): コメントの切り替え
- **Ctrl+Space**: コード補完

---

## 重要な注意点

### Codespacesの無料枠

GitHubの無料アカウントでは、月に一定時間まで無料でCodespacesを使えます。

**節約のコツ：**
- 使わないときは停止する
- ブラウザのタブを閉じても、Codespacesは動き続けます
- 手動で停止する方法：
  1. 左下の緑色の「Codespaces」ボタンをクリック
  2. 「Stop Current Codespace」を選択

### データの保存

- Codespacesで作ったファイルは自動的にGitHub上に残ります
- ただし、**必ずgit commit & pushをしてください**
- そうしないと、Codespacesを削除したときに消えてしまいます

---

## まとめ

お疲れ様でした！これで以下ができるようになりました：

- ✅ GitHubリポジトリの作成
- ✅ Codespacesの起動
- ✅ Dev Containerの設定
- ✅ Rustのインストール
- ✅ 最初のRustプログラムの実行

次は実際にRustを学んでいきましょう！🦀

---

## 困ったときは

- 公式ドキュメント: https://www.rust-lang.org/ja
- Rust日本語フォーラム: https://rust-jp.rs/
- GitHub Docs: https://docs.github.com/ja

頑張ってください！