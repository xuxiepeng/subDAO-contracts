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
VOTE_MODULE=vote_manager
TEMPLATE_MODULE=template_manager
REDPACKET_MODULE=redpacket

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
    echo "copy to ../release"
    cp ${m_dir}/target/ink/${m_name}.wasm ../release/${m_name}_v$VERSION.wasm
    cp ${m_dir}/target/ink/${m_name}.contract ../release/${m_name}_v$VERSION.contract
    cp ${m_dir}/target/ink/metadata.json ../release/${m_name}_v$VERSION.json
    cd -
}

echo "clean release"
rm -rf ${WORK_DIR}/release
mkdir -p ${WORK_DIR}/release

build_module ${BASE_MODULE}
build_module ${TEMPLATE_MODULE}
build_module ${ERC20_MODULE}
# build_module ${GITHUB_MODULE}
build_module ${ORG_MODULE}
build_module ${VAULT_MODULE}
build_module ${VOTE_MODULE}
build_module ${DAO_MODULE}
build_module ${MAIN_MODULE}
build_module ${REDPACKET_MODULE}
