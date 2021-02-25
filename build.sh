#!/usr/bin/env bash

VERSION=0.1
WORK_DIR=$(cd $(dirname $0); pwd)
BASE_MODULE=base
DAO_MODULE=dao_manager
ERC20_MODULE=erc20
GITHUB_MODULE=github
MAIN_MODULE=main
ORG_MODULE=org
VAULT_MODULE=vault
VOTE_MODULE=vote

function build_module() {
    m_name=$1
    m_dir=${WORK_DIR}/${m_name}
    echo "build module ${m_dir}"
    cd ${m_dir}
    cargo +nightly contract build
    if [ $? -ne 0 ];then
      echo "build module failed"
      exit 1
    fi
    echo "copy to ../target"
    cp ${m_dir}/target/${m_name}.wasm ../target/${m_name}_v$VERSION.wasm
    cp ${m_dir}/target/${m_name}.contract ../target/${m_name}_v$VERSION.contract
    cp ${m_dir}/target/metadata.json ../target/${m_name}_v$VERSION.json
    cd -
}

echo "clean target"
rm -rf ${WORK_DIR}/target
mkdir -p ${WORK_DIR}/target

build_module ${BASE_MODULE}
build_module ${DAO_MODULE}
build_module ${ERC20_MODULE}
build_module ${GITHUB_MODULE}
build_module ${MAIN_MODULE}
build_module ${ORG_MODULE}
build_module ${VAULT_MODULE}
build_module ${VOTE_MODULE}