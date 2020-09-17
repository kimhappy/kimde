# 커스터마이징 매뉴얼

- `config.toml`에 `id`와 `password` 이외에도 다양한 옵션을 추가해 kimde를 더욱 효율적으로 사용할 수 있습니다. 다음은 그 목록입니다.

| 키 | 설명 | 기본값 |
| --- | --- | --- |
| `headless`  | 크롬 창을 감춥니다. | `true` |
| `no_future` | 날짜가 안돼서 들을 수 없는 강의를 세지 않습니다. | `false`
| `no_past`   | 날짜가 지나서 들을 수 없는 강의를 세지 않습니다. | `false`
| `no_empty`  | 강의가 없는 과목을 표시하지 않습니다. | `false`
| `no_ok`     | 모든 강의를 다 들은 과목을 표시하지 않습니다. | `false`

***

예를 들어, 현재 들을 수 없는 과목은 세지 않기 위해서는 `config.toml`을 다음과 같이 작성합니다.

```toml
id = "my_id"
password = "my_password"
no_future = true
no_past = true
```