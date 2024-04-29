### Steps to Reproduce | 재현

rust wasm으로 js가 아닌 wasm에서의 setInterval 구현
비동기 라이브러리 tokio를 사용

```toml
[dependencies.tokio]
version = "1.37.0"
features = [
    "time",
    "rt"
]
```
```rust
use tokio::time;

time::Interval();
```

### Expected Behavior | 기대 동작
make set_interval like js

### Error Messages | 에러 메세지
`panicked at library`
`time not implemented on this platform`

### Why | 이유
[tokio docs](https://docs.rs/tokio/latest/tokio/#wasm-support)

>The time module will only work on WASM platforms that have support for timers (e.g. wasm32-wasi). The timing functions will panic if used on a WASM platform that does not support timers.

wasm에서 time feature을 지원하긴 하지만
wasm-unknown-unknown과 같은 일반적인 브라우저 환경에서 지원하지 않음
wasm32-wasi에선 지원함.
