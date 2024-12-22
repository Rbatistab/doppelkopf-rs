#!/bin/bash

source ./.installing_utils/text_utils.sh

info_text "${regular_text}" "Checking for dependencies..."

if RUSTFLAGS=-Awarnings cargo build --release; then
  info_text "${regular_text}" "Project built! ✅"
  exit 0
else
  error_text "${bold_color_red}" "Failed to build the project ❌"
  intended_text "${color_red}" "Report an issue - https://github.com/Rbatistab/dopplekopf-cdk/issues"
  exit 2
fi


