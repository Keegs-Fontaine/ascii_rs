#!/bin/bash

cd ascii_converter &&
wasm-pack build --target web &&
cd ../ &&
mkdir ./dist &&
mv ./ascii_converter/pkg ./dist &&
mv ./app/* ./dist