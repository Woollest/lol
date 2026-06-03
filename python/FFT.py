import numpy as np
import matplotlib.pyplot as plt

def visualize_fft_multiply(a_val, b_val):
    # 1. 数値をリスト（多項式の係数）に変換
    # 例: 123 -> [3, 2, 1] (下の桁から順に)
    a = [int(d) for d in str(a_val)[::-1]]
    b = [int(d) for d in str(b_val)[::-1]]
    
    # 畳み込み後のサイズ（2つの長さの和より大きく、2の累乗が望ましい）
    n = 1
    while n < len(a) + len(b): n *= 2
    
    # 2. FFT（時間領域から周波数領域へ）
    a_fft = np.fft.fft(a, n)
    b_fft = np.fft.fft(b, n)
    
    # 3. 点別積（周波数領域での掛け算）
    res_fft = a_fft * b_fft
    
    # 4. 逆FFT（周波数領域から時間領域へ）
    res_ifft = np.fft.ifft(res_fft)
    coeffs = np.real(res_ifft).round().astype(int)

    # --- 視覚化セクション ---
    fig, axes = plt.subplots(3, 1, figsize=(8, 10))
    plt.subplots_adjust(hspace=0.5)

    # グラフ1: 元の数値の「形」
    axes[0].stem(a, linefmt='b-', label=f'Number A ({a_val})')
    axes[0].stem(b, linefmt='r-', label=f'Number B ({b_val})')
    axes[0].set_title("1. Input Digits (Time Domain)")
    axes[0].legend()

    # グラフ2: FFT後の「波の成分」
    axes[1].plot(np.abs(a_fft), 'b-o', label='FFT(A)')
    axes[1].plot(np.abs(b_fft), 'r-o', label='FFT(B)')
    axes[1].set_title("2. Frequency Components (After FFT)")
    axes[1].legend()

    # グラフ3: 逆FFT後の結果（各桁の積の和）
    axes[2].stem(coeffs, linefmt='g-')
    axes[2].set_title("3. Result Coefficients (After IFFT)")
    
    plt.show()

# 実行（例：123 * 456）
visualize_fft_multiply(123456789, 123456789)