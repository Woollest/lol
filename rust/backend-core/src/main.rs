// src/main.rs
mod models;

use models::{LeagueList, MatchData};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufWriter;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

// 【関数1】PUUIDから直近10試合のマッチIDリストを取得する関数（前回と同じ）
async fn fetch_match_ids_by_puuid(
    client: &reqwest::Client,
    puuid: &str,
    api_key: &str,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!(
        "https://asia.api.riotgames.com/lol/match/v5/matches/by-puuid/{}/ids?start=0&count=10",
        puuid
    );
    let response = client.get(&url).header("X-Riot-Token", api_key).send().await?;
    if !response.status().is_success() {
        return Err(format!("APIエラー(ID取得): ステータス {}", response.status()).into());
    }
    let match_ids: Vec<String> = response.json().await?;
    Ok(match_ids)
}

// 【関数2】マッチIDから試合詳細データ（MatchData）を取得する関数（前回と同じ）
async fn fetch_match_detail(
    client: &reqwest::Client,
    match_id: &str,
    api_key: &str,
) -> Result<MatchData, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!(
        "https://asia.api.riotgames.com/lol/match/v5/matches/{}",
        match_id
    );
    let response = client.get(&url).header("X-Riot-Token", api_key).send().await?;
    if !response.status().is_success() {
        return Err(format!("APIエラー(詳細取得 {}): ステータス {}", match_id, response.status()).into());
    }
    let match_data: MatchData = response.json().await?;
    Ok(match_data)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 🌐 広域ターゲット・大量データ一括コレクター（手堅い直列版） 🌐 ===");

    let api_key = "RGAPI-a55ff0c3-0f30-49ff-8f12-222d1a70b7d2"; 
    let client = reqwest::Client::new();

    // --------------------------------------------------
    // 1. マスター帯の名簿取得
    // --------------------------------------------------
    let league_url = "https://jp1.api.riotgames.com/lol/league/v4/masterleagues/by-queue/RANKED_SOLO_5x5";
    println!("1. マスター帯のプレイヤー一覧を取得中...");
    let response = client.get(league_url).header("X-Riot-Token", api_key).send().await?;
    let league: LeagueList = serde_json::from_str(&response.text().await?)?;
    println!("👉 マスター名簿確保: {} 人\n", league.entries.len());

    // --------------------------------------------------
    // 2. 100人からマッチIDを「超安全リミッター」で回収する
    // --------------------------------------------------
    let target_entries = league.entries.iter().take(100).cloned().collect::<Vec<_>>();
    let (tx_id, mut rx_id) = mpsc::channel(64);

    println!("2. 100人分のプレイヤーからマッチID（各10試合）を分割取得中...");
    let player_chunks: Vec<&[models::LeagueItem]> = target_entries.chunks(3).collect();

    for chunk in player_chunks.iter() {
        for entry in *chunk {
            let tx_clone = tx_id.clone();
            let client_clone = client.clone();
            let api_key_str = api_key.to_string();
            let puuid_str = entry.puuid.clone();

            tokio::spawn(async move {
                if let Ok(match_ids) = fetch_match_ids_by_puuid(&client_clone, &puuid_str, &api_key_str).await {
                    let _ = tx_clone.send(match_ids).await;
                }
            });
        }
        sleep(Duration::from_millis(600)).await;
    }
    drop(tx_id);

    let mut unique_match_ids = HashSet::new();
    while let Some(match_ids) = rx_id.recv().await {
        for id in match_ids {
            unique_match_ids.insert(id);
        }
    }
    
    let all_match_ids: Vec<String> = unique_match_ids.into_iter().collect();
    println!("👉 100人の履歴から重複を排除したユニーク総マッチID: {} 件\n", all_match_ids.len());

    // クールダウン（一応念のため長めに休む）
    println!("⏳ サーバーを落ち着かせるため、30秒間待機します...");
    sleep(Duration::from_secs(30)).await;

    // --------------------------------------------------
    // 3. 【大修正】1件ずつ確実にダウンロードする（直列ループ）
    // --------------------------------------------------
    // 今回は確実に成功させるため、まずは「20件」をターゲットにしてみましょう！
    let download_limit = 20; 
    let download_targets: Vec<String> = all_match_ids.into_iter().take(download_limit).collect();
    
    let mut final_matches: Vec<MatchData> = vec![];
    let total_targets = download_targets.len();

    println!("3. 厳選した {} 件の試合詳細データを1件ずつ安全に取得開始...", total_targets);

    for (index, match_id) in download_targets.iter().enumerate() {
        print!("   [{}/{}] ダウンロード中: {} ... ", index + 1, total_targets, match_id);
        
        // ここで tokio::spawn を使わず、この場で順番に await します
        match fetch_match_detail(&client, match_id, api_key).await {
            Ok(data) => {
                println!("✅ 成功");
                final_matches.push(data);
            }
            Err(e) => {
                println!("❌ 失敗: {}", e);
            }
        }

        // 1件取得するごとに、強制的に 1.3秒 の休憩を挟む（これで秒間1リクエスト未満になり、429は絶対に起きません）
        sleep(Duration::from_millis(1300)).await;
    }

    println!("\n==============================================");
    println!("🎉 完了！ {}件中 【{} 件】 の詳細データを取得・集約！", total_targets, final_matches.len());
    println!("==============================================");

    // --------------------------------------------------
    // 4. JSONに一括保存
    // --------------------------------------------------
    let output_filename = "collected_matches.json";
    let file = File::create(output_filename)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &final_matches)?;
    println!("データを {} に上書き保存しました。✨", output_filename);

    Ok(())
}