GIT_KEY := env_var('GIT_KEY_SEC')

clippy:
    cargo clippy --features testing

bench:
    cargo bench

fmt:
    cargo fmt

test:
    cargo test --features testing

new-release TAG:
    cargo clippy
    git add -u
    git commit -m {{ TAG }}
    git tag {{ TAG }}
    git signify sign -k {{ GIT_KEY }} {{ TAG }}
    git push
    git push origin {{ TAG }}
    git signify push
