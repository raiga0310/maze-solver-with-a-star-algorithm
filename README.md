# A-star Algorithm

## 解説
A* はヒューリスティックに基づいた経路探索アルゴリズムです｡
基本的な探索戦略は､情報の無い探索の基本要素と同じく
- 選択
- 展開
- 生成
に基づいています｡
ポイントは､選択の際に評価関数を用いてオープンリストから優先対象ノードを選択することです｡
ヒューリスティックな探索法における評価関数は候補ノードnについて､f(n)として定義され､A*では以下のようになります｡

f(n) = g(n) + h(n)

ここで､
- g(n) はスタートノードからノードnまでの実際のコスト
- h(n) はノードnからゴールノードまでの推定コスト
となります｡

f(n)が最小となるノードが次の探索対象となります｡

## このRepoで解く問題
シンプルな迷路探索問題です｡プログラムは現在地から4方向に移動可能とし､通路のみ移動可能で､壁は移動不可とします｡

### 実行方法
dungeonファイルを編集して迷路を作成してください｡
- ` `(空白)は通路､`*`は壁を表します｡
- `S`はスタート地点､`G`はゴール地点を表します｡

### コスト評価関数
- g(n)はスタート地点からノードnまでの移動コストです｡
- h(n)はノードnからゴール地点までの推定コストです｡
    - ここではマンハッタン距離を用いています｡