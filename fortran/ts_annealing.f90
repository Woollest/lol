program tsp_quantum_annealing
    use iso_fortran_env, only: real64
    implicit none

    ! 1. 全ての定数を最初に定義 (parameterを確実に付与)
    integer, parameter :: N = 3
    integer, parameter :: NN = 9
    integer, parameter :: N2 = 512
    integer, parameter :: TIME_STEPS = 1000
    complex(real64), parameter :: eye = (0.0_real64, 1.0_real64)

    ! 2. 配列と変数の宣言
    integer :: id(N2, NN)
    real(real64) :: d_dist(N, N)
    real(real64) :: HLL(N2)
    integer :: H01(N2, N2)
    complex(real64) :: f0(N2), f1(N2), T_mat(N2, N2)
    
    real(real64) :: x_pos(N), y_pos(N)
    real(real64) :: a_val, b_val, B0, dt, time_val, At, Bt, D_norm, pena1, pena2, P1, P2
    integer :: i, j, iA, iB, LA, LB, L, L1, step, NS, idx

    ! 3. 値の初期化
    x_pos = [0.2_real64, 0.5_real64, 0.7_real64]
    y_pos = [0.2_real64, 0.5_real64, 0.7_real64]
    a_val = 100.0_real64
    b_val = 100.0_real64
    B0 = 15.0_real64
    dt = 1.0_real64 / real(TIME_STEPS, real64)

    f0 = sqrt(1.0_real64 / N2)
    f1 = 0.0_real64

    ! 4. id配列（スピン状態）の初期化
    id = 0
    do L = 0, N2 - 1
        L1 = L
        do i = 1, NN
            if (L1 > 0) then
                id(L+1, NN - i + 1) = mod(L1, 2)
                L1 = L1 / 2
            end if
        end do
    end do
    id = 1 - 2 * id

    ! 5. 都市間の距離計算
    do i = 1, N
        do j = 1, N
            d_dist(i, j) = sqrt((x_pos(i)-x_pos(j))**2 + (y_pos(i)-y_pos(j))**2)
        end do
    end do

    ! 6. ハミルトニアン対角成分 HLL の計算
    HLL = 0.0_real64
    do L = 1, N2
        ! 都市巡回コスト
        do i = 0, N - 1
            do iA = 1, N
                do iB = 1, N
                    LA = i * N + iA
                    LB = mod((i + 1) * N + iB - 1, NN) + 1
                    HLL(L) = HLL(L) + d_dist(iA, iB) * real(id(L, LA) * id(L, LB), real64)
                end do
            end do
        end do

        ! ペナルティ1
        pena1 = 0.0_real64
        do iA = 1, N
            P1 = 0.0_real64
            do i = 0, N - 1
                P1 = P1 + real(id(L, i*N + iA), real64)
            end do
            pena1 = pena1 + (P1 - 1.0_real64)**2
        end do
        
        ! ペナルティ2
        pena2 = 0.0_real64
        do i = 0, N - 1
            P2 = 0.0_real64
            do iA = 1, N
                P2 = P2 + real(id(L, i*N + iA), real64)
            end do
            pena2 = pena2 + (P2 - 1.0_real64)**2
        end do

        HLL(L) = HLL(L) + a_val * pena1 + b_val * pena2
    end do

    ! 7. 非対角成分 H01 の計算
    H01 = 0
    do i = 1, N2
        do j = 1, N2
            NS = 0
            do idx = 1, NN
                NS = NS + id(i, idx) * id(j, idx)
            end do
            if (NS == NN - 2) H01(i, j) = 1
        end do
    end do

    ! 8. タイムループ (断熱遷移)
    do step = 0, TIME_STEPS - 1
        time_val = step * dt
        At = time_val
        Bt = B0 * (1.0_real64 - time_val)

        do i = 1, N2
            do j = 1, N2
                T_mat(i, j) = -0.5_real64 * dt * (-Bt * real(H01(i, j), real64)) * eye
            end do
            T_mat(i, i) = (1.0_real64, 0.0_real64) + T_mat(i, i) - (0.5_real64 * dt * (HLL(i) * At) * eye)
        end do

        f1 = matmul(T_mat, f0)
        D_norm = sum(abs(f1)**2)
        f0 = f1 / sqrt(D_norm)
    end do

    ! 9. 結果の出力
    print *, "--- Annealing Results (Prob > 0.01) ---"
    do i = 1, N2
        D_norm = abs(f0(i))**2
        if (D_norm > 0.01_real64) then
            print "(A, I3, A, F10.6)", "State(", i-1, ") Prob:", D_norm
        end if
    end do

end program tsp_quantum_annealing