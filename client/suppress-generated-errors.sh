#! /bin/bash

# Sometimes the generated files have small errors that would require `strict: false` to compile
# Instead of changing the tsconfig for the project, just add @ts-nocheck to the start of all generated files
# Generated files are assumed to be correct despite type errors

for file in ./src/grpc-api/*.ts; do
  echo "$file"
  printf "// @ts-nocheck\n%s" "$(cat "$file")" > "$file"
done