set dotenv-load

db-prepare:
    DATABASE_URL=$SB_DATABASE_URL cargo sqlx prepare

compose *args:
    podman compose -f compose.yml {{args}}

build *args:
    just compose --parallel 1 build {{args}}

up *args:
    just compose up {{args}} --force-recreate

run *args:
    just build {{args}}
    just up {{args}}

down *args:
    just compose down {{args}}

services *args:
    just {{args}} postgres

app *args:
    just {{args}} starboard

fmt:
    cargo +nightly fmt
    prek run tombi-format
clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings
test:
    cargo test
deny:
    cargo deny -L error --all-features check --config .config/deny.toml
[default]
check:
    prek run --all-files --stage manual
    @just clippy
    @just test
cicd:
    @just check
