import streamlit as st
import pandas as pd
import json

# ページ設定：ワイドモード
st.set_page_config(page_title="LoL Strategic Picker", page_icon="⚔️", layout="wide")

# カスタムCSSで少し見た目を豪華に
st.markdown("""
    <style>
    .main { background-color: #f5f7f9; }
    .stMetric { background-color: #ffffff; padding: 15px; border-radius: 10px; box-shadow: 0 2px 4px rgba(0,0,0,0.05); }
    </style>
    """, unsafe_allow_html=True)

@st.cache_data
def load_data():
    with open('all_matches.json', 'r') as f:
        matches = json.load(f)
    rows = []
    for m in matches:
        for p in m['info']['participants']:
            rows.append({
                'matchId': m['metadata']['matchId'],
                'teamId': p['teamId'],
                'champion': p['championName'],
                'win': 1 if p['win'] else 0,
                'role': p.get('individualPosition', 'UNKNOWN') # ロール情報を取得
            })
    return pd.DataFrame(rows)

df = load_data()
all_champs = sorted(df['champion'].unique())

# --- ヘッダー ---
st.title("🛡️ LoL Synergy Analyzer")
st.info("味方のピックを選択してください。蓄積されたマスター帯のデータから、最適なシナジーを算出します。")

# --- メインレイアウト ---
col_input, col_result = st.columns([1, 2], gap="large")

with col_input:
    st.subheader("👥 Ally Picks")
    # マルチセレクトにすることで、1人〜4人まで自由に選べるように改善
    selected_allies = st.multiselect(
        "味方チャンピオン（最大4体）", 
        options=all_champs,
        max_selections=4,
        help="名前を入力して検索できます"
    )
    
    st.divider()
    
    min_games = st.slider("信頼度のしきい値（最低試合数）", 1, 10, 2)
    target_role = st.selectbox("自分の担当ロールで絞り込む", ["ALL", "TOP", "JUNGLE", "MIDDLE", "BOTTOM", "UTILITY"])

with col_result:
    if selected_allies:
        # シナジー計算
        match_ids = set(df['matchId'])
        for ally in selected_allies:
            match_ids &= set(df[df['champion'] == ally]['matchId'])
        
        if not match_ids:
            st.warning("⚠️ 指定された組み合わせのデータがまだ不足しています。")
        else:
            # 同じチームの他メンバーを抽出
            synergy_df = df[(df['matchId'].isin(match_ids)) & (~df['champion'].isin(selected_allies))]
            
            # ロールフィルタリング
            if target_role != "ALL":
                synergy_df = synergy_df[synergy_df['role'] == target_role]

            if synergy_df.empty:
                st.write("該当するロールのデータがありません。")
            else:
                stats = synergy_df.groupby('champion')['win'].agg(['count', 'mean']).sort_values(by='mean', ascending=False)
                stats = stats[stats['count'] >= min_games]
                
                if not stats.empty:
                    # 最もおすすめのキャラを強調表示
                    best_champ = stats.index[0]
                    st.metric(label="🏆 Best Synergy Pick", value=best_champ, delta=f"Win Rate: {stats.iloc[0]['mean']:.1%}")
                    
                    st.write("### 📊 Synergy Rankings")
                    # プログレスバー風の勝率表示
                    display_df = stats.copy()
                    display_df.columns = ['Games', 'Win Rate']
                    st.data_editor(
                        display_df,
                        column_config={
                            "Win Rate": st.column_config.ProgressColumn(
                                "Win Rate", format="%.2f", min_value=0, max_value=1
                            ),
                            "Games": st.column_config.NumberColumn("Matches")
                        },
                        use_container_width=True,
                        disabled=True
                    )
                else:
                    st.write("条件に合うデータがありません。")
    else:
        st.write("👈 左側のメニューから味方のピックを選んでください。")