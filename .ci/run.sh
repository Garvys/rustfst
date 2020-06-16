#!/bin/sh

set -ex

if [ `uname` = "Linux" ]
then
  sudo apt-get update
  sudo apt-get install software-properties-common
  sudo add-apt-repository -y ppa:deadsnakes/ppa
  sudo apt-get update
  sudo apt-get install python3.6
  wget https://github.com/sharkdp/hyperfine/releases/download/v1.6.0/hyperfine_1.6.0_amd64.deb
  sudo dpkg -i hyperfine_1.6.0_amd64.deb
fi

./build_openfst.sh
./run_openfst.sh

cargo clean -p rustfst
cargo clean -p rustfst-cli

cargo build --all
cargo test --all
cargo check --benches --all # running benches on travis is useless
cargo doc --all --no-deps

./build_bench.sh
python3 --version

if which python3.6 
then
    virtualenv venv3 -p python3.6
elif which python3.7
then
    virtualenv venv3 -p python3.7
else
    echo "No suitable python version found."
    exit 2
fi
. venv3/bin/activate

pip install -e rustfst-python-bench
# Run benches on a small FST to check that the script is working fine.
python rustfst-python-bench/rustfst_python_bench/bench_all.py rustfst-tests-data/fst_003/raw_vector.fst report.md
python rustfst-python-bench/rustfst_python_bench/bench_all_detailed.py rustfst-tests-data/fst_003/raw_vector.fst report2.md

