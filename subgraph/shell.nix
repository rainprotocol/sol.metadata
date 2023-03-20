let
  pkgs = import
    (builtins.fetchTarball {
      name = "nixos-unstable-2021-10-01";
      url = "https://github.com/nixos/nixpkgs/archive/d3d2c44a26b693293e8c79da0c3e3227fc212882.tar.gz";
      sha256 = "0vi4r7sxzfdaxzlhpmdkvkn3fjg533fcwsy3yrcj5fiyqip2p3kl";
    })
    { };

  prettier-check = pkgs.writeShellScriptBin "prettier-check" ''
    prettier --check .
  '';

  prettier-write = pkgs.writeShellScriptBin "prettier-write" ''
    prettier --write .
  '';

  flush-all = pkgs.writeShellScriptBin "flush-all" ''
    rm -rf cache
    rm -rf node_modules
    rm -rf abis
  '';

  init = pkgs.writeShellScriptBin "init" ''
    rm -rf data
    mkdir -p abis
    cp ../out/MetaBoard.sol/MetaBoard.json abis/
    cargo update
    ethcontract generate abis/MetaBoard.json contract.bin -n MetaBoard --output-dir src
  '';

  codegen = pkgs.writeShellScriptBin "codegen" ''
    graph codegen
  '';

  build = pkgs.writeShellScriptBin "build" ''
    graph build
  '';

  docker-up = pkgs.writeShellScriptBin "docker-up" ''
    docker-compose up -d
  '';

  ci-test = pkgs.writeShellScriptBin "ci-test" ''
    npx mustache config/localhost.json subgraph.template.yaml subgraph.yaml
    codegen
  '';

  local-test = pkgs.writeShellScriptBin "local-test" ''
    npx mustache config/localhost.json subgraph.template.yaml subgraph.yaml
    codegen
  '';

in
pkgs.stdenv.mkDerivation {
  name = "shell";
  buildInputs = [
    pkgs.nixpkgs-fmt
    pkgs.yarn
    pkgs.nodejs-16_x
    pkgs.rustup
    pkgs.cargo
    prettier-check
    prettier-write
    flush-all
    init
    codegen
    build
    ci-test
    local-test
    docker-up
  ];

  shellHook = ''
    export PATH=$( npm bin ):$PATH
    export CARGO_HOME="$HOME/.cargo"
    export PATH="$CARGO_HOME/bin:$PATH"c
    # keep it fresh
    npm i
    init
  '';
}