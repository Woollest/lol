// src/models.rs
use serde::{Deserialize, Serialize};

// =========================================================================
// 1. リーグ名簿（League-V4）関連の構造体
// =========================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeagueList {
    pub tier: String,             // "MASTER" など
    pub queue: String,            // "RANKED_SOLO_5x5"
    pub name: String,
    pub entries: Vec<LeagueItem>, // プレイヤー一覧
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LeagueItem {
    pub puuid: String,            // マッチID取得に使用する固有ID
    pub leaguePoints: i32,
    pub rank: String,             // "I"
    pub wins: i32,
    pub losses: i32,
    // ⚠️ entriesの内部には tier は存在しないため、ここでは定義しません
}

// =========================================================================
// 2. 試合詳細（Match-V5）関連の構造体
// =========================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchData {
    pub metadata: MatchMetadata,
    pub info: MatchInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MatchMetadata {
    pub match_id: String,         // "JP1_582576428" などのマッチID
    pub participants: Vec<String>, // 参加者のPUUIDリスト
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MatchInfo {
    pub game_creation: i64,
    pub game_duration: i64,       // 試合時間（秒）
    pub game_id: i64,
    pub game_mode: String,        // "CLASSIC"（通常のサモナーズリフト）など
    pub game_type: String,
    pub game_version: String,     // パッチバージョンの特定に使用
    pub participants: Vec<Participant>, // 10人のプレイヤーの個人戦績
    pub teams: Vec<Team>,         // 2つのチームの全体情報
}

// 試合に参加した各プレイヤーの詳細データ
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    pub puuid: String,
    pub summoner_name: Option<String>, // Riot ID移行により現在は空の場合あり
    pub riot_id_game_name: Option<String>, // 現在のゲーム内表示名
    pub riot_id_tagline: Option<String>,
    
    pub champion_id: i32,         // チャンピオンの数値ID
    pub champion_name: String,    // "Aali" や "Rakan" などの英語名（分析の主軸）
    
    pub team_id: i32,             // 100 = ブルーチーム, 200 = レッドチーム
    pub win: bool,                // 勝利したか（true / false）
    
    // シナジー分析をさらに深くしたい場合用の基本ステータス
    pub team_position: String,    // "TOP", "JUNGLE", "MIDDLE", "BOTTOM", "UTILITY"
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
}

// チーム全体のデータ
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub team_id: i32,             // 100 or 200
    pub win: bool,
}