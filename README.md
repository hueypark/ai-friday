# AI Friday

Rust + Bevy 0.18 + avian2d 0.5 기반 2D 플랫포머 게임

**플레이**: https://hueypark.github.io/ai-friday/

## 기술 스택

- **Rust** (Edition 2024)
- **Bevy 0.18** — ECS 게임 엔진
- **avian2d 0.5** — 2D 물리 엔진

## 실행 방법

Rust toolchain이 설치되어 있어야 합니다.

```bash
cargo run
```

## 빌드 / 린트

```bash
cargo check   # 컴파일 검증
cargo clippy   # 린트
```

## 조작법

| 키 | 동작 |
|----|------|
| A / D 또는 ← / → | 좌우 이동 |
| Space | 점프 (짧게 누르면 낮은 점프) |
| 적 위에서 착지 | 적 처치 |

## 게임 소개

- 4단계 수직 진행 레벨
- 코인을 수집하여 점수 획득 (tier별 색상 구분)
- 적을 회피하거나 밟아서 처치
- 목숨 3개, 모두 소진 시 게임 오버

## 프로젝트 구조

```
src/
├── main.rs        # 앱 진입점
├── common.rs      # 공유 타입, 상수, 이벤트
├── player/        # 이동, 입력, 점프, 착지, 사망 처리
├── level/         # 맵 레이아웃, 코인, 카메라
├── enemy/         # 적 AI, 순찰, 충돌
└── ui/            # 메뉴, HUD, 게임오버 화면
```
