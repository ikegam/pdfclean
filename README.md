# Markdown Filter

`markdown-rs`を使ってMarkdownを解析し、正規表現でフィルタをかけるRustプログラムです。

## 特徴

- **ユニット単位での処理**: Markdownを見出し、段落、コードブロック、リスト、引用などのユニットに分割して処理
- **抽象化されたハンドラシステム**: `Handler`トレイトによる柔軟なフィルタリング機能
- **特化型ハンドラ**: 見出し専用、段落専用などの特化したハンドラを提供
- **組み合わせ可能**: 複数のハンドラを組み合わせて複雑なフィルタリングが可能

## アーキテクチャ

### ハンドラトレイト

```rust
pub trait Handler: Send + Sync {
    fn can_handle(&self, unit: &MarkdownUnit) -> bool;
    fn handle(&self, unit: MarkdownUnit, context: &HandlerContext) -> Result<Option<MarkdownUnit>>;
    fn name(&self) -> &str;
}
```

### ハンドラの種類

1. **RegexHandler**: 汎用的な正規表現ハンドラ
2. **HeadingHandler**: 見出し専用ハンドラ（RegexHandlerをラップ）
3. **ParagraphHandler**: 段落専用ハンドラ（RegexHandlerをラップ）

## 使用例

```rust
use std::sync::Arc;
use markdown_filter::{MarkdownProcessor, HeadingHandler, ParagraphHandler, RegexHandler};

let mut processor = MarkdownProcessor::new();

// 見出しの "Hello" を "Hi" に置換
let heading_filter = HeadingHandler::new(r"Hello", "Hi".to_string())?;
processor.add_handler(Arc::new(heading_filter));

// 段落内の太字テキストにフィルタ情報を追加
let paragraph_filter = ParagraphHandler::new(
    r"\*\*(.+?)\*\*",
    "**$1 (FILTERED)**".to_string()
)?;
processor.add_handler(Arc::new(paragraph_filter));

// すべてのユニットで斜体を別の形式に変換
let general_filter = RegexHandler::new(
    "italic_filter".to_string(),
    r"\*(.+?)\*",
    "_$1_".to_string()
)?;
processor.add_handler(Arc::new(general_filter));

let result = processor.process(markdown_text)?;
```

## プロジェクト構造

```
src/
├── handlers/           # ハンドラ関連
│   ├── mod.rs         # モジュール定義
│   ├── traits.rs      # Handler トレイトと MarkdownUnit 定義
│   ├── regex_handler.rs    # 汎用正規表現ハンドラ
│   ├── heading_handler.rs  # 見出し専用ハンドラ
│   └── paragraph_handler.rs # 段落専用ハンドラ
├── processor/          # Markdown処理器
│   ├── mod.rs
│   └── markdown_processor.rs # メインの処理エンジン
├── lib.rs             # ライブラリエントリポイント
└── main.rs            # 実行例
```

## 実行

```bash
cargo run
```

サンプルMarkdownに対して各種フィルタを適用した結果が表示されます。