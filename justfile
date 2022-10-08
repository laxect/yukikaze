V := "v" + `bat Cargo.toml | taplo get package.version`
app := `bat ./Cargo.toml | taplo get package.name`
repo := env_var("REPO")

all: build

build: echo
    #!/usr/bin/env bash
    set -euxo pipefail
    buildah build --format docker -t {{repo}}/{{app}}:{{V}}

push: echo
    #!/usr/bin/env bash
    set -euxo pipefail
    V=v`bat Cargo.toml | taplo get package.version`
    podman push {{repo}}/{{app}}:{{V}}

deploy: echo
    #!/usr/bin/env bash
    set -euxo pipefail
    V=v`bat Cargo.toml | taplo get package.version`
    gcloud run deploy {{app}} --image {{repo}}/{{app}}:{{V}}

check: echo
    #!/usr/bin/env bash
    cargo check
    cargo clippy
    dprint check

echo:
    #!/usr/bin/env bash
    echo app := {{app}}
    echo ver := {{V}}
    echo repo := {{repo}}
