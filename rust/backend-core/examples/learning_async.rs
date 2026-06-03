use serde::Deserialize;
use tokio::sync::mpsc;

#[derive(Deserialize, Debug, Clone)]
struct Post {
    id: u32,
    title: String,
}

// 【改善の主役】1件のダウンロードとパースを行う「専用の関数」を新しく作ります。
// 関数に切り出すことで、この中では便利な `?` が使い放題になります！
async fn fetch_single_post(url: &str) -> Result<Post, Box<dyn std::error::Error + Send + Sync>> {
    // 1. 通信開始（失敗したら ? で即座にエラーを返して終了）
    let response = reqwest::get(url).await?;
    
    // 2. テキスト取得（失敗したら ? で終了）
    let body = response.text().await?;
    
    // 3. 構造体へ変換（失敗したら ? で終了）
    let post = serde_json::from_str::<Post>(&body)?;
    
    // すべて成功したら、Ok で構造体を包んで返します
    Ok(post)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("第3段階（改）：スッキリした並行処理テストを開始します...");

    let urls = vec![
        "https://jsonplaceholder.typicode.com/posts/1",
        "https://jsonplaceholder.typicode.com/posts/2",
        "https://jsonplaceholder.typicode.com/posts/3",
    ];

    let (tx, mut rx) = mpsc::channel(32);

    for url in urls {
        let tx_clone = tx.clone();

        // 【劇的ビフォーアフター】
        // 入れ子が消えて、一目で何をしているか分かる美しいコードになりました！
        tokio::spawn(async move {
            // 切り出した関数を呼び出すだけ
            if let Ok(post) = fetch_single_post(url).await {
                // 成功した時だけチャンネルに送る
                let _ = tx_clone.send(post).await;
            } else {
                // 失敗した時の処理（ログ出力など）もここにスッキリ書けます
                println!("URL: {} の取得に失敗しました（スキップします）", url);
            }
        });
    }

    drop(tx); // 大元の送信機を破棄

    // 結果の回収（ここは前と同じです）
    let mut collected_posts = vec![];
    while let Some(post) = rx.recv().await {
        collected_posts.push(post);
    }

    println!("\n--- 集約完了したデータ一覧 ---");
    for post in collected_posts {
        println!("投稿ID: {}, タイトル: {}", post.id, post.title);
    }

    Ok(())
}