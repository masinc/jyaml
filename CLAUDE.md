# CLAUDE.md

このファイルは、このリポジトリでコードを扱う際のClaude Code (claude.ai/code)へのガイダンスを提供します。

## プロジェクト概要

JYAML (JSON-YAML Adaptive Markup Language) は、JSONとYAMLの機能を組み合わせたハイブリッド形式を定義する**データフォーマット仕様**プロジェクトです。このリポジトリには現在、仕様書のみが含まれており、実装コードはまだ存在しません。

## 主な特徴

- **JSONに対して上位互換**: すべての有効なJSONは有効なJYAML
- **YAMLに対して下位互換**: すべての有効なJYAMLは有効なYAML
- **現在のバージョン**: 0.1 (プレリリース)
- **主な目的**: JSONのシンプルさを維持しながら、YAMLの問題点を解決

## リポジトリ構造

リポジトリには仕様書のみが含まれています：
- `spec.md` - 英語仕様書（主要リファレンス）
- `spec.ja.md` - 日本語仕様書
- `README.md` - プロジェクトの簡単な紹介

## JYAMLフォーマットの機能

### データ型
- String（シングル/ダブルクォート必須、`|`と`>`による複数行サポート）
- Number（10進表記、科学的記数法サポート）
- Boolean（`true`/`false`のみ、YAMLエイリアスなし）
- Array（`-`によるブロックスタイルまたは`[]`によるフロースタイル）
- Object（ブロックスタイルまたは`{}`によるフロースタイル、キーは引用符付き文字列必須）
- Null

### JSON/YAMLとの主な違い
- `#`によるコメントサポート
- 配列とオブジェクトのブロックスタイル
- YAMLスタイルの`|`と`>`による複数行文字列
- オブジェクトキーは常に引用符が必要（YAMLと異なる）
- アンカー、エイリアス、代替ブール表現などのYAML固有機能はなし

### ファイル拡張子
- 主要: `.jyml`, `.jyaml`
- YAML相互運用用: `.j.yml`, `.j.yaml`

## 開発ノート

これは仕様のみのリポジトリで実装がないため：
- ビルド、テスト、リントコマンドは存在しません
- 将来の実装は`spec.md`の仕様に従うべきです
- パーサー、バリデーター、コンバーターは別プロジェクトまたはモジュールとして実装することを検討してください