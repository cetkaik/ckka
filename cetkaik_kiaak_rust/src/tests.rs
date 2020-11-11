#[test]
fn sample1() {
    use super::*;
    parse_ckka(r#"
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
    
    星一周"#).unwrap();
}
