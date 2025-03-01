<p align="center">
  <img src="https://github.com/Equim-chan/Mortal/raw/main/docs/src/assets/logo.png" width="550" />
</p>

# Mortal
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/Equim-chan/Mortal/libriichi.yml?branch=main)](https://github.com/Equim-chan/Mortal/actions/workflows/libriichi.yml)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/Equim-chan/Mortal/docs.yml?branch=main&label=docs)](https://mortal.ekyu.moe)
[![dependency status](https://deps.rs/repo/github/Equim-chan/Mortal/status.svg)](https://deps.rs/repo/github/Equim-chan/Mortal)
![GitHub top language](https://img.shields.io/github/languages/top/Equim-chan/Mortal)
![Lines of code](https://www.aschey.tech/tokei/github/Equim-chan/Mortal)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/Equim-chan/Mortal)
[![license](https://img.shields.io/github/license/Equim-chan/Mortal)](https://github.com/Equim-chan/Mortal/blob/main/LICENSE)

[![Donate](https://img.shields.io/badge/Donate-%E2%9D%A4%EF%B8%8F-blue?style=social)](https://mortal.ekyu.moe/donate.html)

Mortal ([凡夫](https://www.mdbg.net/chinese/dictionary?wdqb=%E5%87%A1%E5%A4%AB)) is a free and open source AI for Japanese mahjong, powered by deep reinforcement learning.

Read the [**Documentation**](https://mortal.ekyu.moe) for everything about this work.

## Tsumogiri Self-Play

### コンパイル方法

Tsumogiri自己対戦プログラムをコンパイルするには、以下のコマンドを実行します：

```bash
# Python依存関係なしでビルド
cargo build --bin simple_rule_self_play --no-default-features
```

### 実行方法

#### 単一実行

1回の実行で2つのゲームを処理します：

```bash
./target/debug/simple_rule_self_play
```

実行すると、`game_logs`ディレクトリに以下の形式でゲームログが生成されます：
- `game_{timestamp}_{game_index}.json`

#### 複数回実行

複数回の実行を安定して行うには、付属のシェルスクリプトを使用します：

```bash
# スクリプトに実行権限を付与
chmod +x run_multiple.sh

# スクリプトを実行
./run_multiple.sh
```

このスクリプトは、各実行の間に1秒の遅延を入れることで、リソース競合を回避します。

### ゲームログの形式

生成されるゲームログは[mjai形式](https://github.com/mjx-project/mjx/blob/master/docs/mjai.md)のJSONファイルで、各行が1つのイベント（配牌、ツモ、打牌など）を表します。これらのログは、麻雀AIの学習データとして使用できます。

### 注意事項

- bashのfor文で直接複数回実行すると、ハングアップする可能性があります。その場合は`run_multiple.sh`スクリプトを使用してください。
- 各実行は独立しており、前の実行の結果が次の実行に影響することはありません。

## SimpleRuleAgent Self-Play

SimpleRuleAgentは基本的なルールベースの戦略を実装した麻雀AIで、以下の特徴があります：
- できるだけ早くテンパイに到達し、リーチを宣言する
- 他のプレイヤーがリーチを宣言した場合は防御的に打牌する

### コンパイル方法

SimpleRuleAgent自己対戦プログラムをコンパイルするには、以下のコマンドを実行します：

```bash
# Python依存関係なしでビルド
cargo build --bin simple_rule_vs_simple_rule --no-default-features
```

### 実行方法

#### 単一実行

付属のシェルスクリプトを使用して実行します：

```bash
# スクリプトに実行権限を付与
chmod +x run_simple_rule_vs_simple_rule.sh

# スクリプトを実行
./run_simple_rule_vs_simple_rule.sh
```

実行すると、`game_logs`ディレクトリに以下の形式でゲームログが生成されます：
- `game_{timestamp}_{game_index}.json`

### ゲームログの形式

Tsumogiri Self-Playと同様に、生成されるゲームログは[mjai形式](https://github.com/mjx-project/mjx/blob/master/docs/mjai.md)のJSONファイルです。これらのログは、より高度な麻雀AIの学習データとして使用できます。

### カスタマイズ

ソースコード（`libriichi/src/bin/simple_rule_vs_simple_rule.rs`）を編集することで、以下のようなカスタマイズが可能です：

- プレイヤーの配置を変更する（indexesの配列を編集）
- 異なるゲームバリエーションのためにシードを変更する
- ゲーム数を増やす（indexesとseedsを追加）

## Okay cool now give me the weights!
Read [this post](https://gist.github.com/Equim-chan/cf3f01735d5d98f1e7be02e94b288c56) for details regarding this topic.

## License
### Code
[![AGPL-3.0-or-later](https://github.com/Equim-chan/Mortal/raw/main/docs/src/assets/agpl.png)](https://github.com/Equim-chan/Mortal/blob/main/LICENSE)

Copyright (C) 2021-2022 Equim

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.

### Logo and Other Assets
[![CC BY-SA 4.0](https://github.com/Equim-chan/Mortal/raw/main/docs/src/assets/by-sa.png)](https://creativecommons.org/licenses/by-sa/4.0/)
