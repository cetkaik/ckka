# CKKAファイルの仕様（書きかけ）

2020年10月15日に書きはじめた jekto.vatimeliju

## 概要

この仕様書は、「机戦」「セーケ」「パイグ将棋」「セッカイク」などの名で知られている遊戯の棋譜を記述するための標準形式として、CKKA (cet2 kaik kia1 ak1) ファイルの仕様を定めるものである。この仕様は、人間が自然に書くことができ、かつ現状存在する複数の棋譜機構・棋譜データとの互換性をなるべく保つことのできるように定めたものである。

以下、適宜構文をISO/IEC 14977の[EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form)で表記する（ISO/IEC 14977は古いし[やめろ](https://dwheeler.com/essays/dont-use-iso-14977-ebnf.html)と主張する記事が出てきたので[W3CのEBNF](https://www.w3.org/TR/xml/#sec-notation)を用いようかとも思ったが、まあISO/IEC 14977でええやろ）。

## 文字とエンコード
CKKAファイルはUTF-8でエンコードされていなければならない。バイトオーダーマークは許容しない。

### 改行
改行は、CR+LFとLFの両方を許容する。

### スペース
スペースは、少なくともU+0009（タブ文字）、U+0020（スペース）、U+00A0（ノーブレークスペース）、U+3000（全角スペース）が許容される。

### 句読点
U+002C（カンマ）、U+002E（ピリオド）、U+3001（読点）、U+3002（句点）、U+FF0C（全角カンマ）、U+FF0E（全角ピリオド）、U+FF61（半角句点）、U+FF64（半角読点）が許容される。

## 数の表現方法

数の表現方法は、算用数字表記と漢字転写表記の二種類を許容する。-2,147,483,648 以上 2,147,483,647 以下のみを想定してよい。

### 算用数字表記
`30` とか `-27` とか `-0` とか。正規表現でいうなら `-?(0|[1-9][0-9]*)` にマッチするやつ。

### 漢字転写表記

パイグ語での表現を、便宜上漢字に転写して用いる。用いる漢字は以下の通り。

| 漢字転写 | パイグ音  |
|----------|----------|
| `無` | mun1 |
| `下` | ut2 |
| `一` | et2 |
| `二` | ik2 |
| `三` | om2 または om1 |
| `四` | ap1 |
| `五` | un1 |
| `六` | net2 |
| `七` | nik2 |
| `八` | nom2 または nom1 |
| `九` | nap1 |
| `十` | nun1 |
| `百` | kit1 |
| `万` | ue1 |
| `億` | |

0 は `無` と表記。負数は対応する正数の前に `下` をつけて表記。

1 から 120 までは、 `一`, `二`, `三`, `四`, `五`, `六`, `七`, `八`, `九`, `十`, `十一`, `十二`, `十三`, `十四`, `十五`, `十六`, `十七`, `十八`, `十九`, `二十`, `二十一`, `二十二`, `二十三`, `二十四`, `二十五`, `二十六`, `二十七`, `二十八`, `二十九`, `三十`, ..., `九十九`, `百`, `百一`, `百二`, `百三`, `百四`, `百五`, `百六`, `百七`, `百八`, `百九`, `百十`, `百十一`, `百十二`, `百十三`, `百十四`, `百十五`, `百十六`, `百十七`, `百十八`, `百十九`, `百二十` のように表記する。

121 から 129, 131 から 139, ..., 191 から 199 のように、端数が3文字になるものについては、たとえば 125 であれば `百二十五` だけではなく `百二五` も許容される。

200 から 999 については、`二百十`, `五百二五` などとする。端数が3文字になるものについては必ず `十` を落とす。

1,000 から 9,999 については、例えば 1,000 は `十百`、1,200 は `十二百`、2,000 は `二十百`、2,200 は `二二百` であり、5,678 は `五六百七八` となる。つまり、100で割った商と余りそれぞれについて、三文字になるなら真ん中の `十` を落とす。

10,000 から 19,999 については、`万` の後に残りを言う。 12,345 は `万二三百四五` である。

20,000 から 99,999,999 については、例えば 876,543 は 87万6543 であるので `八十七万六五百四三`。 12,345,678 は 1234万5678 であるので、 `一二百三四万五六百七八` である。

100,000,000 から 199,999,999 は `億` の後に残りである。

それ以上は、例えば 2,147,483,647 は `二十一億四七百四八万三六百四七` である。

[EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form)で表記するなら、以下の `pekzep-integer` 規則にマッチするものが認められる。

```ebnf
less-than-ten = '一' | '二' | '三' | '四' | '五' | '六' | '七' | '八' | '九';
less-than-hundred = [ less-than-ten ], '十', [ less-than-ten ] 
    | less-than-ten;
elided = '十', [ less-than-ten ] 
    | less-than-ten, '十' 
    | less-than-ten, less-than-ten 
    | less-than-ten;
less-than-ten-thousand = [ elided ], '百', [ elided ]
    | '百', less-than-hundred
    | less-than-hundred;
less-than-hundred-million = [ less-than-ten-thousand ], '万', [ less-than-ten-thousand ] | less-than-ten-thousand;
positive = [ less-than-hundred-million ], '億', [ less-than-hundred-million ] | less-than-hundred-million;
pekzep-integer = '無' | [ '下' ], positive;
```


## ヘッダ部

ヘッダ部は、対局情報の列である。対局情報は、`{` と `}` で囲まれた任意の文字列である。文字列自身に `}` の文字を用いたいときには、 `#{` と `}#`　で挟む。文字列自身に `}#` を用いたいときには、 `##{` と `}##` で挟む。0以上の任意の個数の `#` を使ってこれができる。

`{キー:値}` の形で書かれた対局情報は、処理系により特殊な扱いを受けるかもしれない。キーは `:` を含むことがない。 キーが空文字列ときには、キーがないのと同じ扱いになる。例えば、 `18:00` という値のみを書きたい場合は、 `{:18:00}` と書けばよい。

キー及び値の中には改行を許さない。

|     構文         |         ありそうな意味                               |
|--------------|---------------------------------------------------|
| `{律:硬皇力}`| ルールが硬皇力であることを処理系に伝えるかもしれない |
| `{始時:2020-10-14T19:39:05+09:00}`| 対戦開始時刻が日本標準時2020年10月14日19時39分5秒であることを処理系に伝えるかもしれない |
| `{黒名:kuai1}`| 黒を指す人の名前がkuai1であるということを処理系に伝えるかもしれない |
| `{一位色:赤}` | 先手の色が赤であることを処理系に伝えるかもしれない |
| `{季:夏}` | 試合が夏であることを伝えるかもしれない |
| `{https://drive.google.com/drive/folders/183ENcTW65lPGVONnnhkJ1nGKDUYeXWHX?usp=sharing}` | このURL上に関連情報があることを処理系に伝えるかもしれない |

ヘッダ部の最後に、例えば`[筆]十二 [翰]二十八` と記載することで、「先手の名前が『筆』であり12点を所持していること」「後手の名前が『翰』であり28点を所持していること」を表すことができる。点数を両方省略した場合は、どちらも20点を持っているものとする。点数を片方省略した場合は、合計が40点になるようにもう片方の点数が推論される。名前自身に `]` の文字を用いたいときには、 `#[` と `]#`　で挟む。名前自身に `]#` を用いたいときには、 `##[` と `]##` で挟む。0以上の任意の個数の `#` を使ってこれができる。

## ボディ部
`K`, `L`, `N`, `T`, `Z`, `X`, `C`, `M`, `P`, `"` のどれかが行頭（ただしスペースを無視して考える）にある行に遭遇した段階でヘッダ部は終了し、残りはボディ部となる。

ボディ部は「移動要素」と「ゲーム進行要素」の2種類から構成される列である。

環境側は、以下の「表記法⓪」をサポートしなくてはならない。「表記法①」以降の記法をサポートしてもよいが、これはoptionalである。

### 表記法⓪

各々の移動要素とゲーム進行要素はスペースまたは改行によって分断するのが普通であるが、句読点での分断も許される。

移動要素は、以下の `no-step-and-no-stick`, `no-step-and-water-stick`, `step-and-no-stick`, `step-and-water-stick`, `step-and-bridge-stick`, `step-and-bridge-stick-and-water-stick`, `parachute`, `tam-no-step`, `tam-step-unspecified`, `tam-step-during-former`, `tam-step-during-latter` のどれか、およびその後に（それぞれの移動に対するコメントとして）`{` と `}` で囲まれた任意の文字列を追加したものである（前述と同様、 `#{` と `}#` などで囲まれていてもよい。）

持ち駒を打つのでなければ、`兵` などの職業名の代わりに `片` と書くことを許容する。

```ebnf
column = "K" | "L" | "N" | "T" | "Z" | "X" | "C" | "M" | "P";
row = "A" | "E" | "I" | "U" | "O" | "Y" | "AI" | "AU" | "IA";
square = column, row;
water-square = "NO" | "TO" | "ZO" | "XO" | "CO" | "ZI" | "ZU" | "ZY" | "ZAI";
non-water-square = square - water-square;
non-vessel = "兵" | "弓" | "車" | "虎" | "馬" | "筆" | "巫" | "将" | "王";
piece = "船" | non-vessel;
piece-or-wildcard = piece | "片";
non-vessel-or-wildcard = non-vessel | "片";
water-stick = "水", ("或" | "或此無" | "無此無" | "一此無" | "二此無" | "三" | "四" | "五");
bridge-stick-size = "橋", ("或" | "無" | "一" | "二" | "三" | "四" | "五");
tam-sqbracket = "[", (square | "或"), "]";
color = "黒" | "赤";
```

#### 移動―踏越えなし判定なし

```ebnf
no-step-and-no-stick = square, piece-or-wildcard, square, "無撃裁";
```

|     構文     |  意味     |
|--------------|-----------|
| `XU兵XY無撃裁` | XUの兵がXYに移動した。裁は必要なく、したがって判定はしていない。 |

#### 移動―踏越えなし入水判定あり

```ebnf
no-step-and-water-stick = non-water-square, non-vessel-or-wildcard, water-square, water-stick;
```

|     構文     |  意味     |
|--------------|-----------|
| `LY弓ZY水或此無`   | LYの弓がZYに入水しようとしたが、入水判定に失敗した。|
| `LY弓ZY水一此無`   | LYの弓がZYに入水しようとしたが、一が出たので入水判定に失敗した。|
| `LY弓ZY水無此無`   | LYの弓がZYに入水しようとしたが、ゼロが出たので入水判定に失敗した。|
| `LY弓ZY水五`   | LYの弓がZYに入水しようとした。五が出たので入水判定に成功した。|
| `LY弓ZY水或`   | LYの弓がZYに入水しようとした。裁は不明だが入水判定に成功した。|

#### 移動―踏越えあり判定なし

```ebnf
step-and-no-stick = square, piece-or-wildcard, square, square, "無撃裁"; 
```

|     構文     |  意味     |
|--------------|-----------|
| `XU兵XYXAU無撃裁` | XUの兵がXYを踏んでXAUに移動した。裁は必要なく、したがって判定はしていない。 |

#### 移動―踏越えあり無限移動判定なし入水判定あり

```ebnf
step-and-water-stick = non-water-square, non-vessel-or-wildcard, square, water-square, water-stick;
```

|     構文     |  意味     |
|--------------|-----------|
| `NY巫CYCO水五`   | NYの巫がCYを踏んで、COに入水しようとし、五を出し入水判定に成功した。|

#### 移動―踏越えあり無限移動判定あり入水判定なし及び未到達

```ebnf
step-and-bridge-stick = square, piece-or-wildcard, square, square, bridge-stick-size, [ "此無" ];
```

|     構文     |  意味     |
|--------------|-----------|
| `ME弓MIMU橋四` | MEの弓がMIを踏んでMUに進んだ。踏越えの裁は四。 |
| `ME弓MIMY橋或` | MEの弓がMIを踏んでMYに進んだ。踏越えの裁は不明だが判定に成功はしている。 |
| `ME弓MIMY橋或此無` | MEの弓がMIを踏んでMYに進もうとした。裁は不明だが判定に失敗し、MEに戻った。 |
| `ME弓MIMY橋一此無` | MEの弓がMIを踏んでMYに進もうとした。一が出たので判定に失敗している。 |
| `ME弓MIMY橋無此無` | MEの弓がMIを踏んでMYに進もうとした。ゼロが出たので判定に失敗している。 |

#### 移動―踏越えあり無限移動判定あり入水判定あり

```ebnf
step-and-bridge-stick-and-water-stick = non-water-square, no-vessel-or-wildcard, square, water-square, bridge-stick-size, water-stick;
```

|     構文     |  意味     |
|--------------|-----------|
| `LO弓NOCO橋四水五`   | LOの弓がNOを踏んで、四を出してCOに入水しようとし、五を出し入水判定に成功した。|
| `LO弓NOCO橋四水一此無`   | LOの弓がNOを踏んで、四を出してCOに入水しようとし、一を出し入水判定に失敗した。|

#### 移動―手駒からの打ち込み

```ebnf
parachute = color, piece, square;
```

|     構文     |  意味     |
|--------------|-----------|
| `黒弓MY`     | 黒の弓を手元からMYに打った。 |
| `赤車CI` | 赤色の車をCIに打った。|

#### 移動―皇が駒を踏まずに移動し移動

```ebnf
tam-no-step = square, "皇", [ tam-sqbracket ], square;
```

皇は決して裁を要求しないので、`無撃裁`を書かない。

|     構文     |  意味     |
|--------------|-----------|
| `KE皇KI` | KEの皇がどこかに行き、そのあとKIに移動した。|
| `KE皇[或]KI` | KEの皇がどこかに行き、そのあとKIに移動した（`KE皇KI`と同じ）。|
| `KE皇[LE]KI` | KEの皇がLEに行き、そのあとKIに移動した。|

#### 移動―皇が駒を踏んで移動

```ebnf
tam-step-unspecified = square, "皇", square, square;
tam-step-during-former = square, "皇", square, tam-sqbracket, square;
tam-step-during-latter = square, "皇", tam-sqbracket, square, square;
```

|     構文     |  意味     |
|--------------|-----------|
| `PAU皇CAIMAU` | PAUの皇がMAUに移動する途中でCAIを踏んだ。 |
| `PAU皇[MAU]CAIMAU` | PAUの皇がMAUに行き、そのあとCAIを踏んでMAUに移動した。 |
| `PAU皇[或]CAIMAU` | PAUの皇がどこかに行き、そのあとCAIを踏んでMAUに移動した。 |
| `KE皇LI[KE]KA` | KEの皇がLIを踏んでKEに行き、その後KAに移動した。 |
| `KE皇LI[或]KA` | KEの皇がLIを踏んでどこかに行き、その後KAに移動した。 |

#### 役と進行

ゲーム進行要素としては、「駒取得コメント」「再行」「終季」の3種を定義する。

駒取得コメントは読みやすさのために書くことができる。書かなくてもよい。

```ebnf
capture-comment = "手", [ color ], piece;
```

|   構文   |  意味     |
|----------|-----------|
| `手馬`   | 直前の手により馬を手に入れたことを補足する。 |
| `手黒馬` | 直前の手により黒馬を手に入れたことを補足する。 |

再行は `[SY]為(獣)(同色馬弓兵)再行` のように書く。終季は `[SY]為(行行)終季 手五` のように書く。こいつらは義務である。

TODO: 対局者名がかぶったらどうする？直前の手により一応判別が可能なのでなんとかなる説はあるが

### 表記法①

表記法⓪の簡略化であり、「無撃裁」「橋」「水」などをいちいち書かず、普段人間が書く棋譜になるべく合わせた記法である。

|     構文     |  意味     | 対応する表記法⓪ |
|--------------|-----------|----------------|
| `XU兵XYXAU` | XUの兵がXYを踏んでXAUに移動した。裁は必要なく、したがって判定はしていない。 | `XU兵XYXAU無撃裁`
| `ME弓MIMU四` | MEの弓がMIを踏んでMUに進んだ。裁は四。 | `ME弓MIMU橋四` |
| `ME弓MIMU` | MEの弓がMIを踏んでMUに進んだ。裁は不明だが判定に成功はしている。 | `ME弓MIMU橋或` |
| `ME弓MIMY或` | MEの弓がMIを踏んでMYに進んだ。裁は不明だが判定に成功はしている。 | `ME弓MIMY橋或` |
| `ME弓MIMY或ME` | MEの弓がMIを踏んでMYに進もうとした。裁は不明だが判定に失敗し、MEに戻った。 | `ME弓MIMY橋或此無` |
| `ME弓MIMY無` | MEの弓がMIを踏んでMYに進もうとした。裁は不明だが判定に失敗し、MEに戻った。 | `ME弓MIMY橋或此無` |
| `ME弓MIMY一此無` | MEの弓がMIを踏んでMYに進もうとした。一が出たので判定に失敗している。 | `ME弓MIMY橋一此無`
| `ME弓MIMY無此無` | MEの弓がMIを踏んでMYに進もうとした。ゼロが出たので判定に失敗している。 | `ME弓MIMY橋無此無` |
| `LY弓ZY無`   | LYの弓がZYに入水しようとしたが、入水判定に失敗した。| `LY弓ZY水或此無`
| `LY弓ZY此無`   | LYの弓がZYに入水しようとしたが、入水判定に失敗した。| `LY弓ZY水或此無`
| `LY弓ZY一`   | LYの弓がZYに入水しようとしたが、一が出たので入水判定に失敗した。| `LY弓ZY水一此無`
| `LY弓ZY一此無`   | LYの弓がZYに入水しようとしたが、一が出たので入水判定に失敗した。| `LY弓ZY水一此無`
| `LY弓ZY一LY`   | LYの弓がZYに入水しようとしたが、一が出たので入水判定に失敗した。| `LY弓ZY水一此無`
| `LY弓ZY無此無`   | LYの弓がZYに入水しようとしたが、ゼロが出たので入水判定に失敗した。| `LY弓ZY水無此無`
| `LO弓NOCO四五`   | LOの弓がNOを踏んで、四を出してCOに入水しようとし、五を出し入水判定に成功した。| `LO弓NOCO橋四水五`
| `黒弓MY`     | 黒の弓を手元からMYに打った。 | `黒弓MY`
| `或車CI` | なんらかの色の車をCIに打った（状況から色が判断できない場合はエラー）。| `黒車CI` または `赤車CI`
| `PAU皇CAIMAU` | PAUの皇がMAUに移動する途中でCAIを踏んだ。 | `PAU皇CAIMAU`
| `PAU皇[MAU]CAIMAU` | PAUの皇がMAUに行き、そのあとCAIを踏んでMAUに移動した。 | `PAU皇[MAU]CAIMAU`

### 表記法②
漢字転写表記である表記法①に対応する、パイグ音ベースでの表記法。例えば `TU dau2 XY` のように表記する。各々はピリオドによって区切られるのが普通であるが、句読点または改行によっても分断される。

### 表記法③
例えば `"mauAmaimy1"` のように表記される。各々はJSONの仕様により規定される文字列リテラルから構成され、スペース、改行、または句読点（U+3002）によって分断される。[cerke_noterのフォーマット](https://github.com/schwert398/cerke_noter/wiki/Specification-of-note-in-machine-readable-format-%28MRF%29)との互換性を重視している。

## 例

### 例1（表記法①）

```
LAU弓LAILY無{定弓失敗} LE弓LILU{定弓}
LAU弓LAILY無{もう一度失敗} ME弓XEZE{王の守り}
NAI兵LAILY{抗弓①} NI兵NE{皇処之巫狙い}
LAU弓NAU{抗弓②} ZO皇NU{此善}
```

### 例2（表記法①）

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

### 例3（表記法①）

```
{律:硬皇力}
{:2018年4月8日 18:00頃}
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

### 例4（表記法②）
```
{律:硬皇力}
{:2018年4月8日 17:30頃}
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

### 例5（表記法①）
```
{:2018年4月8日 01:30頃}
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

### 例6（表記法③）
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
