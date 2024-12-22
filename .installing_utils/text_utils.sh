#!/bin/bash

###############################################################################
#                           Colors and styles                                 #
###############################################################################

# Text reset
regular_text='\033[0m'

# Regular Colors
color_red='\033[0;31m'
color_green='\033[0;32m'

# Bold Colors
bold_color_red='\033[1;31m'
bold_color_green='\033[1;32m'

###############################################################################
#                               Text format                                   #
###############################################################################

###############################################################################
# Formats a text according to a desired color or style. Then resets the color
# back to normal
# Globals:
#   None
# Arguments:
#   [style] [text]
# Returns:
#	  None
###############################################################################
f_text(){
  echo -e "${1}${2}${regular_text}\0"
  return 0
}

###############################################################################
# Prints an info message
# Globals:
#   None
# Arguments:
#   [style] [text]
# Returns:
#	  None
###############################################################################
info_text(){
  f_text "${1}" "[INFO] ${2}"
}

###############################################################################
# Prints an error message
# Globals:
#   None
# Arguments:
#   [style] [text]
# Returns:
#	  None
###############################################################################
error_text(){
  f_text "${1}" "[ERROR] ${2}"
}

###############################################################################
# Prints a message that is intended. Should be used after info or
# error messages
# Globals:
#   None
# Arguments:
#   [style] [text]
# Returns:
#	  None
###############################################################################
intended_text(){
  f_text "${1}" "  - ${2}"
}