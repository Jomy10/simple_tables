#!/usr/bin/env zsh
cd tables || (echo "The tables directory is no more" && exit)
echo "Building docs.."
cargo doc

echo "Copying docs..."
cd ..
rm -r docs
cp -r tables/target/doc docs

echo "Copying index.html..."
cp res/index.html docs/index.html

echo "Building docs complete."