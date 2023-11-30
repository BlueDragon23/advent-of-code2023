#!/usr/bin/env bash

set -euxo pipefail

DAY="day$1"
SOURCE_FILE="src/bin/$DAY.rs"
cp src/template.rs "$SOURCE_FILE"
sed -e "s#../input/example.txt#../../input/$DAY.txt#" -i '' "$SOURCE_FILE"
sed -e "s#../input/example.test.txt#../../input/$DAY.test.txt#" -i '' "$SOURCE_FILE"