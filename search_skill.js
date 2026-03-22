const fs = require('fs');

// ダウンロードしたファイルを読み込む
const data = JSON.parse(fs.readFileSync('championFull.json', 'utf8'));
const champions = data.data;

// 探したいキーワード（ここを書き換えて遊べます）
const keyword = "チャーム"; 

console.log(`--- 「${keyword}」を持つチャンピオンを検索中 ---`);

for (let key in champions) {
    const champ = champions[key];
    let hasSkill = false;

    // パッシブと全スキル(Q,W,E,R)の説明文をチェック
    const allSkills = [champ.passive, ...champ.spells];
    
    allSkills.forEach(skill => {
        if (skill.description.includes(keyword)) {
            hasSkill = true;
        }
    });

    if (hasSkill) {
        console.log(`・${champ.name} (${champ.title})`);
    }
}
