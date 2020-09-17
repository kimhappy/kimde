# 빌드 매뉴얼

- Windows 이외의 OS를 이용하거나 빌드된 실행 파일을 신뢰할 수 없는 경우 직접 빌드해야 합니다.

## 윈도우에서 빌드하기

1. [rustup](https://win.rustup.rs)을 설치합니다.
2. [git](https://git-scm.com/download/win)을 설치합니다.
3. 원하는 폴더에서 터미널을 열고, `git clone https://github.com/POMMI3R/kimde && cd kimde && cargo build --release` 를 통해 저장소를 다운로드 및 빌드합니다.
4. `kimde/target/release`에 실행 파일이 생긴걸 볼 수 있습니다.

## Linux, macOS에서 빌드하기

- 현재 macOS에서 `config.toml`을 찾지 못하는 문제가 있습니다. 나중에 나중에 돈을 많이 벌어서 맥을 사면 수정하겠습니다ㅠㅠ

1. `curl https://sh.rustup.rs -sSf | sh`를 통해 rustup을 설치합니다.
2. 각자 OS에 맞는 방법으로 git을 설치합니다.
3. 원하는 폴더에서 터미널을 열고, `git clone https://github.com/POMMI3R/kimde && cd kimde && cargo build --release` 를 통해 저장소를 다운로드 및 빌드합니다.
4. `kimde/target/release`에 실행 파일이 생긴걸 볼 수 있습니다.