#!/bin/bash

source ./.installing_utils/text_utils.sh

###############################################################################
# Logs error about OS unsupported by installer
# Globals:
#   None
# Arguments:
#   None
# Returns:
#	  None
###############################################################################
unsupported_os(){
  error_text "${bold_color_red}" "Unsupported OS ❌"
  intended_text "${color_red}" "You may ask for support for your OS - https://github.com/Rbatistab/dopplekopf-cdk/issues"
  exit 1
}

info_text "${regular_text}" "Checking installer support for your OS..."
OS=$(uname)
info_text "${regular_text}" "Detected OS: ${OS}"

case "$OS" in
  "Linux"|"Darwin")
    # Linux installer
    info_text "${regular_text}" "${OS} is compatible with this installer! ✅"
    unsupported_os
    exit 0
    ;;
  "MINGW"*|"MSYS"*|"CYGWIN"*)
    # Windows installer
    unsupported_os
    ;;
  *)
    unsupported_os
    ;;
esac
