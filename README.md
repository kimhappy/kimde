# KIMDE

![logo](pictures/logo.png)

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0) [![written in Rust](https://img.shields.io/badge/written%20in-Rust-orange)](https://www.rust-lang.org/) ![version](https://img.shields.io/badge/version-0.2.0-yellow) ![Hanyang University](https://img.shields.io/badge/Hanyang-University-1d2475) ![CSE20](https://img.shields.io/badge/CSE-20-red)

***

## 소개

![screenshot](pictures/screenshot0.png)

- KIMDE는 [한양대학교 블랙보드](https://learn.hanyang.ac.kr/) 출결 확인 프로그램입니다.
- 자동 출석 프로그램이 아닙니다.
- 개인정보를 수집하지 않습니다.
- GPLv3 라이선스로 배포됩니다.
- 사용시 발생하는 모든 문제의 책임은 실행한 사람에게 있습니다.

## 사용법

### 1. KIMDE 실행 파일 다운로드

- Windows에 한해 빌드된 실행 파일을 제공합니다. [여기](bin)에서 다운로드 받을 수 있습니다.
- 다른 OS를 이용하거나 빌드된 실행 파일을 신뢰할 수 없는 경우 [빌드 매뉴얼](how_to_build.md)을 참고해 직접 빌드해 주세요(선택).

### 2. 웹드라이버 설치

- [Google Chrome](https://www.google.com/chrome/)을 설치합니다. 이미 설치되어 있다면, 최신 버전으로 업데이트해 줍니다. 현재 stable 최신 버전은 85입니다.
- 운영체제에 맞는 [Chrome Driver](https://chromedriver.chromium.org/downloads) 버전 85를 다운받아, 적당한 곳에 압축을 풀어줍니다.

### 3. 설정 파일 작성

- `config.toml`은 아이디, 비밀번호 등의 설정을 관리하는 파일입니다.
- `config.toml`은 한 번만 작성하면 됩니다.
- `config.toml`은 KIMDE 실행 파일과 같은 디렉토리에 있어야 합니다.
- 메모장 혹은 에디터를 열어 다음의 내용을 작성한 후, `config.toml`로 저장합니다.

```toml
id = "my_id"
password = "my_password"
```

- `my_id`와 `my_password`에는 자신의 블랙보드(한양대학교 포털) 아이디와 비밀번호를 입력합니다.
- [커스터마이징 매뉴얼](how_to_customize.md)을 참고해 다양한 옵션을 추가할 수 있습니다(선택).

### 4. 실행

- 1에서 다운받은 `chromedriver.exe`를 실행합니다.
- KIMDE 실행 파일을 실행합니다. 이 때 `config.toml`이 같은 디렉토리에 있어야 합니다.
- 실행이 끝나면, `chromedriver.exe`를 종료합니다.

## 자주 묻는 질문

Q1. 검은 창이 반짝 하고 닫혀요ㅠㅠ

A1. 검은 창이 반짝 하고 닫히는 것은 여러 원인이 있습니다. 다음의 사항들을 체크해 주세요.

1. `config.toml`을 잘 작성했나 확인해 주세요.
2. [확장자를 보이게 한 뒤](https://mainia.tistory.com/5104), `config.toml.txt`을 `config.toml`로 바꿔주세요.
3. 사용자가 직접 Chrome을 통해 로그인할 때, 비밀번호를 변경하라는 창이 뜨지 않는지 확인해 주세요. 의도된 동작은 아니지만, 수정할 계획은 없습니다. 비밀번호를 주기적으로 변경하게 하기 위함입니다.
4. 종료 후 다시 실행해 주세요. 크롬과 크롬 브라우저 버전이 85인지 확인해 주세요.
5. 대학원생이면 로그인에 실패하는 문제가 있습니다. 빠르게 수정하도록 하겠습니다.

***

Q2. 5분 이상 실행했는데 반응이 없어요ㅠㅠ

A2. 종료 후 다시 실행해 주세요. 재차 반복해도 반응이 없으면 `headless`를 `false`로 설정하고 어떤 창에서 멈추는지 캡쳐해서 제보해 주세요.

***

Q3. 글자가 깨져요ㅠㅠ

A3. 일부 터미널(예: cmd)에서 글자가 깨져보일 수 있습니다. 사용에는 문제가 없습니다.

## 버그 제보, 의견

- 버그를 발견하거나 건의할 의견이 있다면, [카카오톡 오픈채팅](https://open.kakao.com/o/sSsjNIwc)으로 연락주세요. 다음은 버그의 예시입니다.

1. 프로그램 실행이 5분 이상 소요됨
2. 프로그램이 켜지자마자 닫힘
3. 프로그램을 실행할 수 없음
4. 블랙보드 상 출결과 다르게 나옴

- 혹은 과자를 사주셔도 됩니다.

## 도움 주신 분들

- `넘멋져용`님 도네 감사합니다.
- `대학원생`님 버그 제보 감사합니다.
- `하진`님 도네 감사합니다.
- `최은국`님 도네 감사합니다.
- `펭`님 도네 감사합니다.
- `ㅠㅠ`님 도네 감사합니다.
- `감사합니당`님 버그 제보, 도네 감사합니다.
- `보드`님 버그 제보, 도네 감사합니다.
- `안냐세요`님 버그 제보 감사합니다.
- `박소희`님 도네 감사합니다.
- `감사합니당`님 도네 감사합니다.
- `20CSE`님 버그 제보 감사합니다.
- `카톡`님 도네 감사합니다.
- `C2H5OH`님 의견 제시, 도네 감사합니다.
- `카멜`님 버그 제보, 도네 감사합니다.
- [1cdkrim](https://github.com/1cekrim)님이 블랙보드 API 분석을 도와주셨습니다. 감사합니다.
- 그 이외에 응원해 주신 모든 분들께 감사합니다.

## 릴리즈 노트

### v0.2.0

- 속도를 대폭 개선했습니다.
- `config.toml`에서 `port` 관련 항목을 삭제했습니다.
- `config.toml`에서 `headless`가 다시 필수가 아니게 됐습니다.
- `config.toml`에 `no_empty`, `no_ok`, `no_past`, `no_future` 항목을 설정할 수 있습니다.

### v0.1.6

- 최초 실행시 속도가 빨라졌습니다.
- 이제 `headless`가 반드시 필요합니다.
- '온라인 출석 조회' 탭의 순서에 상관없이 실행이 가능해졌습니다.

### v0.1.5

- `config.toml` 관련 오류 메시지가 상세해졌습니다.

### v0.1.4

- 이제 공지가 떠도 공지를 닫고 올바르게 동작합니다.
- `config.toml`의 `headless` 옵션을 통해 Chrome 창이 보여지게 할 수 있습니다.

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
