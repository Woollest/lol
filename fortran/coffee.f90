program coffee_cooling
    use iso_fortran_env, only: real64
    implicit none

    !変数の定義
    real(real64), parameter :: T_env = 20.0_real64  !室温（°C）
    real(real64), parameter :: T_init = 90.0_real64  !コーヒーの初期温度（°C）
    real(real64), parameter :: K = 0.05_real64  !冷却系数（容器の断熱性能）
    integer, parameter :: steps = 10

    !配列の宣言
    real(real64) :: minutes(0:10)
    real(real64) :: temperature(0:10)
    integer :: i

    minutes = [(real(i, real64), i = 0, steps)]

    temperature = T_env + (T_init - T_env) * exp(-k * minutes)

    print *, "---Coffee Temperature Log---"
    print *, "Minute | Temperature"
    print *, "--------------------"
    do i = 0, steps
        print "(F7.1, A, F10.2)", minutes(i), "min : ", temperature(i)
    end do

end program coffee_cooling