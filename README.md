# KIMDE

![logo](pictures/logo.png)

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0) [![written in Rust](https://img.shields.io/badge/written%20in-Rust-orange)](https://www.rust-lang.org/) ![version](https://img.shields.io/badge/version-0.1.3-yellow)

***

## 소개

![screenshot](pictures/screenshot0.png)

- KIMDE는 [한양대학교 블랙보드](https://learn.hanyang.ac.kr/) 출결 확인 프로그램입니다.
- 자동 출석 프로그램이 아닙니다.
- 개인정보를 수집하지 않습니다.
- GPLv3 라이선스로 배포됩니다.
- 사용시 발생하는 모든 문제의 책임은 실행한 사람에게 있습니다.

## 사용법

### 1. KIMDE 실행 파일 다운로드 혹은 빌드
- KIMDE는 크로스 플랫폼 프로그램입니다. Windows, Linux, macOS 등 다양한 환경에서 사용할 수 있습니다.
- 빌드된 실행 파일은 Windows에 한해 제공됩니다. [여기](bin)에서 다운로드 받을 수 있습니다.
- 다른 OS를 이용하거나 빌드된 실행 파일을 신뢰할 수 없는 경우, `cargo build --release`를 사용해 직접 빌드할 수 있습니다.

### 2. 웹드라이버 설치
- KIMDE는 웹드라이버를 사용해 출결을 한 번에 확인합니다.
- 다양한 웹드라이버를 사용할 수 있지만, 여기서는 `ChromeDriver`를 사용하는 예제를 보여줍니다.
- 우선 [Google Chrome](https://www.google.com/chrome/)을 설치합니다. 이미 설치되어 있다면, 최신 버전으로 업데이트해 줍니다. 현재 stable 최신 버전은 85입니다.
- 크롬 버전과 운영체제에 맞는 [Chrome Driver](https://chromedriver.chromium.org/downloads)를 다운받아, 적당한 곳에 압축을 풀어줍니다.

### 3. 설정 파일 작성
- KIMDE는 `config.toml` 파일을 사용해 아이디, 비밀번호 등의 설정을 관리합니다.
- `config.toml`은 한 번만 작성하면 됩니다.
- `config.toml`은 KIMDE 실행 파일과 같은 디렉토리에 있어야 합니다.
- 메모장 혹은 에디터를 열어 다음의 내용을 작성한 후, `config.toml`로 저장합니다.

```
port = 9515
id = "my_id"
password = "my_password"
semester = "2020년 2학기"
```

- `9515`는 `ChromeDriver`가 사용하는 포트 번호입니다. 다른 웹드라이버를 사용하려면 해당 웹드라이버의 포트 번호를 입력합니다.
- `id`와 `password`에는 자신의 블랙보드(한양대학교 포털) 아이디와 비밀번호를 입력합니다.
- `semester`에는 현재 학기를 입력해 줍니다. 블랙보드 코스에서 현재 학기를 확인할 수 있습니다.

### 4. 실행
- 1에서 다운받은 `chromedriver.exe`를 실행합니다.
- KIMDE 실행 파일을 실행합니다. 이 때 `config.toml`이 같은 디렉토리에 있어야 합니다.
- 최초 실행 시 몇 가지 정보를 `config.toml`에 캐싱하느라 시간이 오래 걸릴 수 있습니다. 느긋하게 기다려 주세요.
- 실행이 끝나면, `chromedriver.exe`를 종료합니다.

### 5. 설정 파일 관리(고급)
- 어떤 교수님들은 녹강 대신 실강만 진행합니다.
- `config.toml`을 편집해 해당 과목을 출결 확인에서 제외할 수 있습니다.
- 이 방법은 KIMDE를 1회 이상 실행한 이후에 사용할 수 있습니다.
- 다음은 `config.toml`의 예시입니다.

```
port = 9515
id = "my_id"
password = "my_password"
semester = "2020년 2학기"

[[course]]
id = "_11111_1"
name = "Take it slow"
prof = "아이린"

[[course]]
id = "_22222_2"
name = "Candy"
prof = "슬기"

[[course]]
id = "_33333_3"
name = "Light me up"
prof = "웬디"

[[course]]
id = "_44444_4"
name = "Cool world"
prof = "조이"

[[course]]
id = "_55555_5"
name = "Fool"
prof = "예리"
```

- 이 중에서 `조이` 교수님의 `Cool world` 과목의 출결을 확인하고 싶지 않으면, 해당 블록을 지워주시면 됩니다.
- 이제 `조이` 교수님과 실강으로 수업합니다. 만세!

```
port = 9515
id = "my_id"
password = "my_password"
semester = "2020년 2학기"

[[course]]
id = "_11111_1"
name = "Take it slow"
prof = "아이린"

[[course]]
id = "_22222_2"
name = "Candy"
prof = "슬기"

[[course]]
id = "_33333_3"
name = "Light me up"
prof = "웬디"

[[course]]
id = "_55555_5"
name = "Fool"
prof = "예리"
```

### 6. 버그 제보, 의견
- 버그를 발견하거나 건의할 의견이 있다면, [카카오톡 오픈채팅](https://open.kakao.com/o/sSsjNIwc)으로 연락주세요.
- 다음은 버그의 예시입니다.
1. 프로그램 실행이 5분 이상 소요됨
2. 프로그램이 켜지자마자 닫힘
3. 프로그램을 실행할 수 없음
4. 블랙보드 상 출결과 다르게 나옴
- 혹은 과자를 사주셔도 됩니다.

## 릴리즈 노트

### v0.1.3
- 이제 테이블 모드에서도 올바르게 동작합니다.

### v0.1.2
- 이제 Chrome 창이 표시되지 않습니다.
- ChromeDriver 이외의 웹드라이버를 사용하려면 소스를 수정해야 합니다.

### v0.1.1
- 종료 전에 엔터를 받습니다.

### v0.1.0
- 최초 릴리즈

## 코로나 안전수칙
![코로나 안전수칙](pictures/corona.png)
