#[test]
fn sample1() {
    use super::*;
    parse_ckka(
        r#"
    {https://drive.google.com/drive/folders/183ENcTW65lPGVONnnhkJ1nGKDUYeXWHX?usp=sharing}
    {:2020-10-15}
    [SY]二十 [ぶちょー]二十
    CI兵XIXU無撃裁         LAU弓LAILY橋三
    LE弓LILU橋二           MAI兵MY無撃裁
    PE巫MECE橋二           XAU虎CAIXAU橋三
    CE巫CI無撃裁           ZIA王TAUNAU無撃裁
    XI兵XUXY無撃裁         XAU虎ZAIXY無撃裁 手兵
    XU兵XY無撃裁 手虎      NAI兵NY無撃裁
    CI巫KIALIA橋二 手馬 
    [SY]為(同色獣)再行 
                            LY弓LU無撃裁 手弓
    LIA巫NIANAU無撃裁 手王 
    [SY]為(同色獣)(王)終季 手二十
    
    星一周"#,
    )
    .unwrap();
}

#[test]
fn sample2() {
    use super::*;
    parse_ckka(
        r#"
        {始時:2021-11-06T14:04:02.430Z}
        {一位色:赤}
        {終時:2021-11-06T14:30:35.476Z}
        [jekto.vatimeliju] [Rhemestry]
        LE片LILU橋二    KAU片KAIKY無撃裁
        NI片TITU無撃裁    MAU片MAIMY橋二
        KE片NE無撃裁    XAI片XY無撃裁
        KA片KIKY橋三 手巫    KAI片KY無撃裁 手筆
        NE片NI無撃裁    ZO皇[ZY]ZAIZAU
        ZI片ZAI無撃裁 手船    CAI片CAU無撃裁
        ZAU皇ZAI[ZY]TY    TAI片ZAIZAU無撃裁
        ZAI片ZAUZIA橋二 手王
        
        [jekto.vatimeliju]為(王)終季 手五
            春終
        
        MAU片MAIMY橋三    PE片PIPU無撃裁
        NAI片TAITY無撃裁    LE片TEZE橋三
        CAI片CAU無撃裁    ZO皇[XY]CAI
        TIA片ZAU無撃裁    ZI片ZAIZAU橋一 手将
        XIA片ZAU無撃裁 手船    ZA片TENE無撃裁
        CAI皇[MAU]CAUXIA    PU片XU無撃裁
        PAU片CAUCAI橋三    XU片XAICAI橋二 手巫
        XAU片CAI無撃裁 手巫    PA片PIPU橋三
        LAU片NAU無撃裁    PU片PAIPIA橋二 手筆
        
        [jekto.vatimeliju]為(地心)終季 手七
            夏終
        
        LE片LILU橋四    PAU片PAIPY無撃裁
        ZO皇[ZU]ZIZE    ZAI片ZI無撃裁 手船
        TE片ZIXU橋二    ZIA片XAUCAU無撃裁
        XA片ZATE無撃裁    PY片XY無撃裁
        XI片XUXY無撃裁 手巫    CAI片XAIXY無撃裁 手兵
        ZA片TENE無撃裁    ZI片CICA橋無此無
        TA片TEZI水二此無    ZI片CICA橋二 手車
        ME片MACA橋三 手船    MAU片MAIMY橋三
        ZE皇[XI]CIMU    XAI片XYXU無撃裁 手虎
        XE片CIXU橋四 手兵    XY片XU無撃裁 手虎
        黒船CO    MU皇CO[CY]CAI
        赤巫MO    MAI片MYMO無撃裁 手巫
        CAI皇[CY]MOMU    CAU片MAU無撃裁
        CO片MOMY橋四 手弓    MO片MY無撃裁 手船
        CA片CIXU橋四 手兵    黒虎CAI
        MU皇[CO]XY    LAU片LAILY橋五
        XU片MYMAU橋四 手王
        
        [jekto.vatimeliju]為(馬弓兵)(王)(戦集)(助友)終季 手十六
            秋終
        
        
        星一周"#,
    )
    .unwrap();
}

/*
#[test]
fn sample3() {
    use super::*;
    parse_ckka(
        r#"
        {始時:2021-11-06T14:04:02.430Z}
        {一位色:赤}
        {終時:2021-11-06T14:30:35.476Z}
        [jekto.vatimeliju] [Rhemestry]
        LE片LILU橋二    KAU片KAIKY無撃裁
        NI片TITU無撃裁    MAU片MAIMY橋二
        KE片NE無撃裁    XAI片XY無撃裁
        KA片KIKY橋三手黒巫    KAI片KY無撃裁手黒筆
        NE片NI無撃裁    ZO皇[ZY]ZAIZAU
        ZI片ZAI無撃裁手黒船    CAI片CAU無撃裁
        ZAU皇ZAI[ZY]TY    TAI片ZAIZAU無撃裁
        ZAI片ZAUZIA橋二手黒王

        或為王而手五
        終季    春終

        MAU片MAIMY橋三    PE片PIPU無撃裁
        NAI片TAITY無撃裁    LE片TEZE橋三
        CAI片CAU無撃裁    ZO皇[XY]CAI
        TIA片ZAU無撃裁    ZI片ZAIZAU橋一手赤将
        XIA片ZAU無撃裁手赤船    ZA片TENE無撃裁
        CAI皇[MAU]CAUXIA    PU片XU無撃裁
        PAU片CAUCAI橋三    XU片XAICAI橋二手赤巫
        XAU片CAI無撃裁手黒巫    PA片PIPU橋三
        LAU片NAU無撃裁    PU片PAIPIA橋二手黒筆

        或為地心而手七
        終季    夏終

        LE片LILU橋四    PAU片PAIPY無撃裁
        ZO皇[ZU]ZIZE    ZAI片ZI無撃裁手赤船
        TE片ZIXU橋二    ZIA片XAUCAU無撃裁
        XA片ZATE無撃裁    PY片XY無撃裁
        XI片XUXY無撃裁手赤巫    CAI片XAIXY無撃裁手赤兵
        ZA片TENE無撃裁    ZI片CICA橋無此無
        TA片TEZI水二此無    ZI片CICA橋二手赤車
        ME片MACA橋三手黒船    MAU片MAIMY橋三
        ZE皇[XI]CIMU    XAI片XYXU無撃裁手赤虎
        XE片CIXU橋四手赤兵    XY片XU無撃裁手黒虎
        黒船CO    MU皇CO[CY]CAI
        赤巫MO    MAI片MYMO無撃裁手赤巫
        CAI皇[CY]MOMU    CAU片MAU無撃裁
        CO片MOMY橋四手赤弓    MO片MY無撃裁手黒船
        CA片CIXU橋四手黒兵    黒虎CAI
        MU皇[CO]XY    LAU片LAILY橋五
        XU片MYMAU橋四手黒王

        或為馬弓兵加王加戦集加助友而手十六
        終季    秋終


        星一周"#,
    )
    .unwrap();
}

*/
