#!/usr/bin/env python

import os
import sys
import json
import argparse
import tempfile

f = open('.github/workflows/runtimes-matrix.json', 'r')
runtimesMatrix = json.load(f)

runtimeNames = list(map(lambda x: x['name'], runtimesMatrix))

print(f"Available runtimes: {runtimeNames}")

parser = argparse.ArgumentParser(description='A command runner for polkadot runtimes repo')
parser.add_argument('command', help='Command to run', choices=['bench', 'fmt'])
parser.add_argument('--runtime', help='Runtime(s) space separated', choices=runtimeNames, nargs='*')
parser.add_argument('--pallet', help='Pallet(s) space separated', nargs='*')

args = parser.parse_args()

if args.command == 'bench':
    tempdir = tempfile.TemporaryDirectory()
    print(f'Created temp dir: {tempdir.name}')

    # TODO: uncomment
    os.system('cargo build -p chain-spec-generator --profile release --features runtime-benchmarks')

    if args.runtime:
        # filter out only the specified runtime from runtimes
        print(f'Provided runtimes: {args.runtime}')
        runtimesMatrix = list(filter(lambda x: x['name'] in args.runtime, runtimesMatrix))
        print(f'Filtered out runtimes: {runtimesMatrix}')

    # loop over left runtimes and print names
    for runtime in runtimesMatrix:
        print(f'-- building chain specs for {runtime["name"]}')
        print(f'-- listing pallets for benchmark for {runtime["name"]}')
        wasm_file = f"target/production/wbuild/{runtime['package']}/{runtime['package'].replace('-', '_')}.wasm"
        output = os.popen(f"frame-omni-bencher v1 benchmark pallet --all --list --runtime={wasm_file}").read()
        print(f"Output of list: {output}")
        raw_pallets = output.split('\n')

        pallets = []
        for pallet in raw_pallets:
            if pallet:
                pallets.append(pallet.split(',')[0])

        print(f'Pallets: {pallets}')

    # if args.pallet:
    #     print(f'Pallets: {args.pallet}')

    tempdir.cleanup()

elif args.command == 'fmt':
    os.system('cargo +nightly fmt')
    os.system('taplo format --config .config/taplo.toml')

print('Done')
