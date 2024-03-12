#!/bin/bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

WORKDIR=$(mktemp -d)

pushd "${WORKDIR}" || exit 1

# System V ABI
wget -q -r -np https://www.sco.com/developers/gabi/latest/contents.html && \
    rm -rf gabi && \
    mv www.sco.com/developers/gabi/latest gabi && \
    rm -rf www.sco.com && \
    pushd gabi && \
    ls -tr1 *.html | xargs -i bash -c 'pandoc {} -t html -o {}.pdf || echo "Failed to convert {}"' && \
    pdfunite $(ls -tr1 *.pdf | tr '\n' ' ') "${SCRIPT_DIR}/gabi.pdf" && \
    popd && \
    rm -rf gabi

# ARM
gh release download -D "${SCRIPT_DIR}" --repo ARM-software/abi-aa --pattern '*elf*.pdf' --clobber

# M68K
curl -L -o "${SCRIPT_DIR}/m68k-abi.pdf" https://uclibc.org/docs/psABI-m68k.pdf

# MIPS
curl -L -o "${SCRIPT_DIR}/mips.pdf" https://uclibc.org/docs/psABI-mips.pdf

# PA-RISC
curl -L -o "${SCRIPT_DIR}/pa-risc-abi.pdf" https://uclibc.org/docs/psABI-pa-risc.pdf

# PowerPC
curl -L -o "${SCRIPT_DIR}/ppc-abi.pdf" https://uclibc.org/docs/psABI-ppc.pdf

# PowerPC-TLS
curl -L -o "${SCRIPT_DIR}/ppc-tls.pdf" https://uclibc.org/docs/tls-ppc.pdf

# PowerPC64
curl -L -o "${SCRIPT_DIR}/ppc64-abi.pdf" https://uclibc.org/docs/psABI-ppc64.pdf

# PowerPC64-TLS
curl -L -o "${SCRIPT_DIR}/ppc64-tls.pdf" https://uclibc.org/docs/tls-ppc64.pdf

# S390
curl -L -o "${SCRIPT_DIR}/s390-abi.pdf" https://uclibc.org/docs/psABI-s390.pdf

# S390X
curl -L -o "${SCRIPT_DIR}/s390x-abi.pdf" https://uclibc.org/docs/psABI-s390x.pdf

# SH
curl -L -o "${SCRIPT_DIR}/sh-abi.pdf" https://uclibc.org/docs/psABI-sh.txt

# SPARC
curl -L -o "${SCRIPT_DIR}/sparc-abi.pdf" https://uclibc.org/docs/psABI-sparc.pdf

# RISC-V
gh release download -D "${SCRIPT_DIR}" --repo riscv-non-isa/riscv-elf-psabi-doc --pattern '*.pdf' --clobber && \

# X86-64
curl -L -o "${SCRIPT_DIR}/x86-64-abi.pdf" https://gitlab.com/x86-psABIs/x86-64-ABI/-/jobs/artifacts/master/raw/x86-64-ABI/abi.pdf?job=build

# X86
if [ ! -d "${WORKDIR}/i386-ABI" ]; then
    git clone https://gitlab.com/x86-psABIs/i386-ABI "${WORKDIR}/i386-ABI"
else
    git -C "${WORKDIR}/i386-ABI" git pull
fi

pushd "${WORKDIR}/i386-ABI" && \
    make pdf && \
    cp abi.pdf "${SCRIPT_DIR}/i386-abi" && \
    popd
