# CKKAファイルの仕様（書きかけ）

2020年10月15日に書きはじめた jekto.vatimeliju

## 概要

この仕様書は、「机戦」「セーケ」「パイグ将棋」「セッカイク」などの名で知られている遊戯の棋譜を記述するための標準形式として、CKKA (cet2 kaik kia1 ak1) ファイルの仕様を定めるものである。この仕様は、人間が自然に書くことができ、かつ現状存在する複数の棋譜機構・棋譜データとの互換性をなるべく保つことのできるように定めたものである。

## 文字とエンコード
CKKAファイルはUTF-8でエンコードされていなければならない。バイトオーダーマークは許容しない。

### 改行
改行は、CR+LFとLFの両方を許容する。

### スペース
スペースは、少なくともU+0009（タブ文字）、U+0020（スペース）、U+00A0（ノーブレークスペース）、U+3000（全角スペース）が許容される。

### 句読点
U+002C（カンマ）、U+002E（ピリオド）、U+3001（読点）、U+3002（句点）、U+FF0C（全角カンマ）、U+FF0E（全角ピリオド）、U+FF61（半角句点）、U+FF64（半角読点）が許容される。

## ヘッダ部

ヘッダ部は、対局情報の列である。対局情報は、`{` と `}` で囲まれた任意の文字列である。文字列自身に `}` の文字を用いたいときには、 `#{` と `}#`　で挟む。文字列自身に `}#` を用いたいときには、 `##{` と `}##` で挟む。0以上の任意の個数の `#` を使ってこれができる。

`{キー:値}` の形で書かれた対局情報は、処理系により特殊な扱いを受けるかもしれない。

|     構文         |         ありそうな意味                               |
|--------------|---------------------------------------------------|
| `{律:硬皇力}`| ルールが硬皇力であることを処理系に伝えるかもしれない |
| `{始時:2020-10-14T19:39:05+09:00}`| 対戦開始時刻が日本標準時2020年10月14日19時39分5秒であることを処理系に伝えるかもしれない |
| `{黒名:kuai1}`| 黒を指す人の名前がkuai1であるということを処理系に伝えるかもしれない |
| `{一位色:赤}` | 先手の色が赤であることを処理系に伝えるかもしれない |
| `{季:夏}` | 試合が夏であることを伝えるかもしれない |

ヘッダ部の最後に、例えば`[筆]十二 [翰]二十八` と記載することで、「先手の名前が『筆』であり12点を所持していること」「後手の名前が『翰』であり28点を所持していること」を表すことができる。点数を両方省略した場合は、どちらも20点を持っているものとする。点数を片方省略した場合は、合計が40点になるようにもう片方の点数が推論される。名前自身に `]` の文字を用いたいときには、 `#[` と `]#`　で挟む。名前自身に `]#` を用いたいときには、 `##[` と `]##` で挟む。0以上の任意の個数の `#` を使ってこれができる。

どのように数を表現することが許容されるかについては、後述する。

## ボディ部
`K`, `L`, `N`, `T`, `Z`, `X`, `C`, `M`, `P`, `"` のどれかが行頭（ただしスペースを無視して考える）にある行に遭遇した段階でヘッダ部は終了し、残りはボディ部となる。

### 表記法①
例えば `CI兵XIXU` のように表記する。各々はスペースまたは改行によって分断するのが普通であるが、句読点での分断も許される。
持ち駒を打つのでなければ、`兵` などの職業名の代わりに `片` と書くことを許容する。

|     構文     |  意味     |
|--------------|-----------|
| `ME弓MIMU四` | MEの弓がMIを踏んでMUに進んだ。裁は四。 |
| `ME弓MIMU` | MEの弓がMIを踏んでMUに進んだ。裁は不明だが判定に成功はしている。 |
| `ME弓MIMY或` | MEの弓がMIを踏んでMYに進んだ。裁は不明だが判定に成功はしている。 |
| `ME弓MIMY或ME` | MEの弓がMIを踏んでMYに進もうとした。裁は不明だが判定に失敗し、MEに戻った。 |
| `ME弓MIMY無` | MEの弓がMIを踏んでMYに進もうとした。裁は不明だが判定に失敗し、MEに戻った。 |
| `ME弓MIMY一此無` | MEの弓がMIを踏んでMYに進もうとした。一が出たので判定に失敗している。 |
| `ME弓MIMY無此無` | MEの弓がMIを踏んでMYに進もうとした。ゼロが出たので判定に失敗している。 |
| `LY弓ZY無`   | LYの弓がZYに入水しようとしたが、入水判定に失敗した。|
| `LY弓ZY此無`   | LYの弓がZYに入水しようとしたが、入水判定に失敗した。|
| `LY弓ZY一`   | LYの弓がZYに入水しようとしたが、一が出たので入水判定に失敗した。|
| `LY弓ZY一此無`   | LYの弓がZYに入水しようとしたが、一が出たので入水判定に失敗した。|
| `LY弓ZY一LY`   | LYの弓がZYに入水しようとしたが、一が出たので入水判定に失敗した。|
| `LY弓ZY無此無`   | LYの弓がZYに入水しようとしたが、ゼロが出たので入水判定に失敗した。|
| `LO弓NOCO四五`   | LOの弓がNOを踏んで、四を出してCOに入水しようとし、五を出し入水判定に成功した。|
| `黒弓MY`     | 黒の弓を手元からMYに打った。 |
| `或車CI` | なんらかの色の車をCIに打った。|
| `PAU皇CAIMAU` | PAUの皇がMAUに移動する途中でCAIを踏んだ。 |
| `PAU皇[MAU]CAIMAU` | PAUの皇がMAUに行き、そのあとCAIを踏んでMAUに移動した。 |

### 表記法②
例えば `TU dau2 XY` のように表記する。各々はピリオドによって区切られるのが普通であるが、句読点または改行によっても分断される。

### 表記法③
例えば `"mauAmaimy1"` のように表記される。各々はJSONの仕様により規定される文字列リテラルから構成され、スペース、改行、または句読点（U+3002）によって分断される。

## 例1

```
LAU弓LAILY無{定弓失敗} LE弓LILU{定弓}
LAU弓LAILY無{もう一度失敗} ME弓XEZE{王の守り}
NAI兵LAILY{抗弓①} NI兵NE{皇処之巫狙い}
LAU弓NAU{抗弓②} ZO皇NU{此善}
```

## 例2

```
CI兵XIXU LAU弓LAILY
LE弓LILU MAI兵MY
PE巫MECE XAU虎CAIXAU
CE巫CI ZAU王TAUNAU
XI兵XUXY XAU虎ZAIXY
XU兵XY NAI兵NY
CI巫KIALIA 再行 LY弓LU
LIA巫NIANAU 終季 手二十
```

## 例3

```
{律:硬皇力}
{2018年4月8日 18:00頃}
[SY] [補集合]
TE虎NITU TAI兵TY
ZI船ZU TAU虎ZAIXY
ZO皇XICE  XIA将ZAU
LE弓ZE TY兵TU
TI兵TU ZIA王XIA
ME弓MIMU四 XY虎MU
PE巫ME LIA馬TAI
MI兵MU LAU弓LAILY
ZE弓TE LY弓ZY無
TU兵TY ZAI船ZU
TY兵TAI
[SY]為獣而手三
終季
```

## 例4
```
{律:硬皇力}
{2018年4月8日 17:30頃}
[補集合] [hsjoihs]
TAI kauk2 TY. XI kauk2 XU.
XAI kauk2 XY. ZI nuak1 ZU.
ZAI nuak1 ZY. ZO tam2 ZU XI.
XAU dau2 ZAI. XA uai1 XE ZI.
MAU gua2 ZAU. LE gua2 TE ZE ap1.
XIA uai1 XAU. ZA io TE NE.
PAU tuk2 PAI PY. XE dau2 ZI TU.
NAI kauk2 NY. TA uai1 ZE XE.
CAI kauk2 CY. XU kauk2 XY.
ZAI dau2 XY. TU dau2 XY.
TAU dau2 ZAI mun1. ME gua2 MI MU.
MAI kauk2 MY. XY dau2 PIA.
TAU dau2 NAI. ZU nuak1 ZY.
ZAU gua2 ZY. MU gua2 MY MIA.
CIA kaun1 XAU ZAI et2. MIA gua2 ZIA.
[hsjoihs] zau io hop1 om2.
ta xot1.
```

## 例5
```
{2018年4月8日 01:30頃}
[JV]二十 [SY]二十
TAU虎NAITY XE虎CIXU
LIA馬TAIXO XI兵XUXO無
ZAI船ZY ME弓ZE
XIA将XAUZAI無 ZO皇ZAI
CAI兵CY XI兵XUXO無
XIA将XAUCAI XI兵XUXO無
CAI将XY XU虎TY
TAI兵TY TE虎ZITU
XY将XAICAU TU虎MAU
CIA車MAU ZAI皇XAIXY
CAU将CAI ZI船ZY
XY皇ZAU KE巫LENE
黒虎TAI NI兵NU
XAI兵XY NE巫NI
TY兵ZY ZE弓ZY無
NIA車ZAI無 NA車XU
XO馬MI CI兵MI

[SY]為獣而手三
再行

XY兵XU XI兵XU
或車CI NI巫CI

[SY]為行行而五
終季

春終
[JV]十二 [SY] 二十八
```

## 例6
```
{黒名:kuai1} {赤名:iei2}
{一位色:赤} {季:夏}
"meAxeze3",
"mauAmaimy1",
"zoMnine",
"ziaKxiacau",
"caCcizo4",
"zaiVzo",
"ziVzo"
"caiPcy",
"BVzi",
"myAmima2",
"paOma",
"pauScaucai",
"zoVto",
"miaHxaito4=tymor",
"tiPto3",
"caiScixe1=taxt=FHCV/HT"
```
