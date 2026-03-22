// 最新のパッチバージョンを取得する簡単なプログラム
fetch('https://ddragon.leagueoflegends.com/api/versions.json')
  .then(response => response.json())
  .then(data => {
    const latestVersion = data[0];
    console.log('-----------------------------------------');
    console.log('現在のLoL最新パッチは: ' + latestVersion);
    console.log('Woollestさんの開発環境から接続成功！');
    console.log('-----------------------------------------');
  })
  .catch(error => console.error('エラーが発生しました:', error));
