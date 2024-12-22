#!/bin/bash

source ./.installing_utils/text_utils.sh

###############################################################################
# Logs info message about removing binary
# Globals:
#   None
# Arguments:
#   None
# Returns:
#	  None
###############################################################################
info_uninstalling_binary(){
  info_text "${regular_text}" "Removing files..."
}

###############################################################################
# Logs info message about binaries successfully removed
# Globals:
#   None
# Arguments:
#   None
# Returns:
#	  None
###############################################################################
info_deleted_files(){
    info_text "${regular_text}" "Doppelkopf has been removed! ✅\n"
}
