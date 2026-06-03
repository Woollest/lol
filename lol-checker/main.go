package main

import (
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"time"
)

// --- 設定 ---
const (
	APIKey     = "RGAPI-05d0e1b3-252b-4bc1-8138-3135927dbfce"
	Region     = "asia"
	Platform   = "jp1"
	MaxMatches = 500  // 取得したい合計試合数
	WaitTime   = 1200 // API制限回避のための待機時間(ms)
)

// --- データ構造 ---

type LeagueList struct {
	Entries []struct {
		Puuid string `json:"puuid"`
	} `json:"entries"`
}

type MatchDetail struct {
	Metadata struct {
		MatchId string `json:"matchId"`
	} `json:"metadata"`
	Info interface{} `json:"info"`
}

func main() {
	// 1. マスター帯プレイヤーリストの取得
	fmt.Printf("🔍 Fetching Master Tier players on %s...\n", Platform)
	leagueURL := fmt.Sprintf("https://%s.api.riotgames.com/lol/league/v4/masterleagues/by-queue/RANKED_SOLO_5x5?api_key=%s", Platform, APIKey)
	
	body, status := apiRequest(leagueURL)
	if status != http.StatusOK {
		fmt.Printf("❌ Failed to fetch league list (Status: %d)\n", status)
		return
	}

	var league LeagueList
	if err := json.Unmarshal(body, &league); err != nil {
		fmt.Println("❌ JSON Unmarshal Error:", err)
		return
	}

	fmt.Printf("✅ Found %d players.\n", len(league.Entries))

	// 2. 試合データの収集
	var allMatches []MatchDetail
	processedMatches := make(map[string]bool) // 重複排除用マップ

	fmt.Printf("🚀 Starting data collection (Target: %d matches)...\n", MaxMatches)

	for _, player := range league.Entries {
		if len(allMatches) >= MaxMatches {
			break
		}

		// プレイヤーごとの最新試合IDを取得
		matchIDsURL := fmt.Sprintf("https://%s.api.riotgames.com/lol/match/v5/matches/by-puuid/%s/ids?start=0&count=5&api_key=%s", Region, player.Puuid, APIKey)
		mBody, mStatus := apiRequest(matchIDsURL)
		if mStatus != http.StatusOK {
			continue
		}

		var ids []string
		json.Unmarshal(mBody, &ids)

		for _, id := range ids {
			if len(allMatches) >= MaxMatches {
				break
			}

			// すでに取得済みの試合ならスキップ
			if processedMatches[id] {
				continue
			}

			// 試合詳細の取得
			fmt.Printf("[%d/%d] Fetching Match: %s\n", len(allMatches)+1, MaxMatches, id)
			detailURL := fmt.Sprintf("https://%s.api.riotgames.com/lol/match/v5/matches/%s?api_key=%s", Region, id, APIKey)
			dBody, dStatus := apiRequest(detailURL)
			
			if dStatus == http.StatusOK {
				var detail MatchDetail
				if err := json.Unmarshal(dBody, &detail); err == nil {
					allMatches = append(allMatches, detail)
					processedMatches[id] = true // 取得済みリストに登録
				}
			}

			// API Rate Limitへの配慮
			time.Sleep(time.Duration(WaitTime) * time.Millisecond)
		}
	}

	// 3. ファイルへの書き出し
	saveToFile("all_matches.json", allMatches)
}

// HTTPリクエストの共通処理
func apiRequest(url string) ([]byte, int) {
	resp, err := http.Get(url)
	if err != nil {
		return nil, http.StatusInternalServerError
	}
	defer resp.Body.Close()
	
	body, _ := io.ReadAll(resp.Body)
	return body, resp.StatusCode
}

// JSONファイル保存処理
func saveToFile(filename string, data interface{}) {
	file, err := json.MarshalIndent(data, "", "  ")
	if err != nil {
		fmt.Println("❌ Failed to marshal JSON:", err)
		return
	}

	if err := os.WriteFile(filename, file, 0644); err != nil {
		fmt.Println("❌ Failed to write file:", err)
		return
	}
	fmt.Printf("\n✨ Successfully saved %d matches to %s\n", len(data.([]MatchDetail)), filename)
}