#!/bin/bash

source ./.installing_utils/text_utils.sh

###############################################################################
# Logs info message about installing binary
# Globals:
#   None
# Arguments:
#   None
# Returns:
#	  None
###############################################################################
info_installing_binary(){
  info_text "${regular_text}" "Installing files..."
}

###############################################################################
# Logs info message about binaries successfully installed
# Globals:
#   None
# Arguments:
#   None
# Returns:
#	  None
###############################################################################
info_installed_files(){
    info_text "${regular_text}" "Doppelkopf files installed! ✅\n"
    intended_text "${regular_text}" "Verify installation by running: 'dppkf -h"
}