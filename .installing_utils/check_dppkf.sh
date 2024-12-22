#!/bin/bash

source ./.installing_utils/text_utils.sh

info_text "${regular_text}" "Checking installation...."
intended_text "${regular_text}" "Running 'dppkf -h'"

if dppkpf -h; then
  echo ""
  info_text "${regular_text}" "Doppelkopf is installed! ✅\n"
  exit 0
else
  error_text "${bold_color_red}" "Failed verify installation ❌"
  intended_text "${regular_text}" "Add '/usr/local/bin' to your PATH"
  exit 3
fi


