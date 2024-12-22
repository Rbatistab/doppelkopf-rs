#!/bin/bash

source ./.installing_utils/text_utils.sh

info_text "${regular_text}" "Checking for dependencies..."

# Check if rust is installed
if ! command -v cargo >/dev/null 2>&1; then
  error_text "${bold_color_red}" "Cargo is not installed ❌"
  intended_text "${color_red}" "Visit https://www.rust-lang.org/tools/install to install it"
  exit 1
fi

info_text "${regular_text}" "Dependencies checked! ✅\n"

exit 0