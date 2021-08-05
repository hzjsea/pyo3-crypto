#!/bin/bash

echo "init......"

# set python version 
# INSTALL_PYTHON_VERSION=python3.6


find_python() {
        set +e
        unset BEST_VERSION
        for V in 37 3.7 38 3.8 39 3.9 3; do
                if which python$V >/dev/null; then
                        if [ "$BEST_VERSION" = "" ]; then
                                BEST_VERSION=$V
                        fi
                fi
        done
        echo $BEST_VERSION
        set -e
}

if [ "$INSTALL_PYTHON_VERSION" = "" ]; then
        INSTALL_PYTHON_VERSION=$(find_python)
fi

# This fancy syntax sets INSTALL_PYTHON_PATH to "python3.7", unless
# INSTALL_PYTHON_VERSION is defined.
# If INSTALL_PYTHON_VERSION equals 3.8, then INSTALL_PYTHON_PATH becomes python3.8
# 找不到就python3.7
INSTALL_PYTHON_PATH=python${INSTALL_PYTHON_VERSION:-3.7}
echo $INSTALL_PYTHON_PATH

echo "Python version is $INSTALL_PYTHON_VERSION"
$INSTALL_PYTHON_PATH -m venv venv
if [ ! -f "activate" ]; then
        ln -s venv/bin/activate .
fi

. ./activate

python -m pip install --upgrade pip
python -m pip install wheel
python -m pip install -r ./requirements.txt
maturin build
maturin develop

current_shell=$(echo $SHELL)
if current_shell=/bin/bash; then
	echo  "PASS: source /venv/bin/activate >> ~/.bashrc"
elif current_shell=/bin/zsh;then
	echo "PASS: source /venv/bin/activate >> ~/.zshrc"
fi



