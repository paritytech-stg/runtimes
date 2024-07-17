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
    runtime_pallets_map = {}

    profile = "release"

    # TODO: uncomment
    os.system(f"cargo build -p chain-spec-generator --profile {profile} --features runtime-benchmarks")

    # filter out only the specified runtime from runtimes
    if args.runtime:
        print(f'Provided runtimes: {args.runtime}')
        runtimesMatrix = list(filter(lambda x: x['name'] in args.runtime, runtimesMatrix))
        # convert to mapped dict
        runtimesMatrix = {x['name']: x for x in runtimesMatrix}

        print(f'Filtered out runtimes: {runtimesMatrix}')

    # loop over remaining runtimes to collect available pallets
    for runtime in runtimesMatrix.values():
        print(f'-- listing pallets for benchmark for {runtime["name"]}')
        wasm_file = f"target/{profile}/wbuild/{runtime['package']}/{runtime['package'].replace('-', '_')}.wasm"
        output = os.popen(f"frame-omni-bencher v1 benchmark pallet --all --list --runtime={wasm_file}").read()
        raw_pallets = output.split('\n')[1:]  # skip the first line with header

        all_pallets = set()
        for pallet in raw_pallets:
            if pallet:
                all_pallets.add(pallet.split(',')[0])

        pallets = list(all_pallets)
        print(f'Pallets in {runtime}: {pallets}')
        runtime_pallets_map[runtime['name']] = pallets

    # filter out only the specified pallets from collected runtimes/pallets
    if args.pallet:
        print(f'Pallet: {args.pallet}')
        new_pallets_map = {}
        # keep only specified pallets if they exist in the runtime
        for runtime in runtime_pallets_map:
            if set(args.pallet).issubset(set(runtime_pallets_map[runtime])):
                new_pallets_map[runtime] = args.pallet

        runtime_pallets_map = new_pallets_map

    print(f'Filtered out runtimes & pallets: {runtime_pallets_map}')

    if not runtime_pallets_map:
        if args.pallet and not args.runtime:
            print(f"No pallets [{args.pallet}] found in any runtime")
        elif args.runtime and not args.pallet:
            print(f"{args.runtime} runtime does not have any pallets")
        elif args.runtime and args.pallet:
            print(f"No pallets [{args.pallet}] found in {args.runtime}")
        else:
            print('No runtimes found')
        sys.exit(0)

    header_path = os.path.abspath('./.github/scripts/cmd/file_header.txt')

    for runtime in runtime_pallets_map:
        for pallet in runtime_pallets_map[runtime]:
            config = runtimesMatrix[runtime]
            print(f'-- config: {config}')
            output_path = f"./{config['path']}/src/weights/{pallet.replace('::', '_')}.rs";
            print(f'-- benchmarking {pallet} in {runtime} into {output_path}')

            os.system(f"frame-omni-bencher v1 benchmark pallet "
                      f"--extrinsic=* "
                      f"--runtime=target/{profile}/wbuild/{config['package']}/{config['package'].replace('-', '_')}.wasm "
                      f"--pallet={pallet} "
                      f"--header={header_path} "
                      f"--output={output_path} "
                      f"--wasm-execution=compiled  "
                      f"--steps=50 "
                      f"--repeat=20 "
                      f"--heap-pages=4096 "
                      )

    tempdir.cleanup()

elif args.command == 'fmt':
    os.system('cargo +nightly fmt')
    os.system('taplo format --config .config/taplo.toml')

print('Done')
