# pdfclean

AI-specialized markdown cleaner for PDF-extracted content. PDFから抽出されたMarkdownの空白やフォーマットの問題をクリーンアップしつつ、画像参照や表構造を保護します。

## 特徴

- **PDFから抽出されたMarkdown専用設計**: AI生成コンテンツでよくある問題に対応
- **AST直接操作**: `markdown::mdast::Node`を直接操作して高速処理
- **コンテンツ保護**: 画像参照、表、コードブロックのフォーマットを保持
- **スマートな空白処理**: 不要な空白を除去しつつ構造化コンテンツは保護
- **モジュラー設計**: NodeProcessorによる拡張可能なアーキテクチャ

## 使用例

```bash
# ファイルをクリーンアップ（ファイル間処理）
pdfclean input.md output.md

# ファイルから標準出力
pdfclean input.md

# 標準入出力を使用
pdfclean < input.md > output.md

# パイプ処理
cat messy.md | pdfclean > clean.md
```

## アーキテクチャ

### NodeProcessor トレイト

```rust
pub trait NodeProcessor: Send + Sync {
    fn should_process(&self, node: &Node) -> bool;
    fn process_node(&self, node: Node, context: &ProcessContext) -> Result<Option<Node>>;
    fn name(&self) -> &str;
}
```

### プロセッサの種類

1. **WhitespaceProcessor**: 空白・改行の正規化（画像と表は除外）
2. **ImageProcessor**: 画像参照 `![alt](url)` を完全保護
3. **TableProcessor**: Markdownテーブルと疑似テーブルを保護

## ライブラリとして使用

```rust
use pdfclean::{MarkdownCleaner, WhitespaceProcessor, ImageProcessor, TableProcessor};
use std::sync::Arc;

let mut cleaner = MarkdownCleaner::new();

// プロセッサを追加
cleaner.add_processor(Arc::new(WhitespaceProcessor::new()));
cleaner.add_processor(Arc::new(ImageProcessor::new()));
cleaner.add_processor(Arc::new(TableProcessor::new()));

let cleaned = cleaner.process(markdown_content)?;
```

## プロジェクト構造

```
src/
├── processors/         # NodeProcessor実装
│   ├── mod.rs         # モジュール定義
│   ├── traits.rs      # NodeProcessor トレイトとProcessContext定義
│   ├── whitespace_processor.rs  # 空白処理プロセッサ
│   ├── image_processor.rs       # 画像保護プロセッサ
│   └── table_processor.rs       # 表保護プロセッサ
├── cleaner.rs          # メインクリーナーエンジン
├── lib.rs             # ライブラリエントリポイント
└── main.rs            # CLI実行ファイル
```

## テスト

```bash
# 統合テスト実行
cargo test

# 基本的な動作確認
cargo run -- tests/fixtures/basic_text.md /tmp/output.md
```

## 特徴的な処理

### 画像参照の保護
```markdown
# 処理前
![  画像   ](  image.jpg  )

# 処理後（保護されて変更なし）
![  画像   ](  image.jpg  )
```

### 表構造の保護
```markdown
# 処理前・後（保護されて変更なし）
| 列1    | 列2     |
|--------|---------|
| データ | データ  |
```

### スマートな空白処理
```markdown
# 処理前
段落1


段落2　　　です。

# 処理後
段落1

段落2です。
```

### コードブロックの保護
````markdown
# 処理前・後（内部の空白は保護）
```
コード　　　例　　　です
空白　　　保持
```
````