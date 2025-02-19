use const_format::concatcp;
use dppkf_lib::utils::text_style::{BOLD, RED, RESET};

pub const GET_PLAYER_NAME_STR: &str = "What's your name? ";
pub const FAILED_GET_PLAYER_NAME_STR: &str = concatcp!(
    RED, BOLD, "ERROR:", RESET, "Failed to read your name."
);
pub const GET_SUIT_TYPES_STR: &str = concatcp!(
    BOLD, "Suit Types:\n", RESET,
    " 1. French\n",
    " 2. German\n",
    "Select your suit type (type 1 or 2): "
);
pub const FAILED_GET_SUIT_TYPE_STR: &str = concatcp!(
    RED, BOLD, "ERROR:", RESET, "Failed to read your suit type."
);
pub const INVALID_SUIT_TYPE_STR: &str = concatcp!(
    RED, BOLD, "ERROR:", RESET, "Invalid number. You can only enter '1' or '2'"
);
pub const GET_PACK_SIZE_STR: &str = concatcp!(
    BOLD, "Pack size:\n", RESET,
    " 1. 48 cards\n",
    " 2. 40 cards\n",
    "Select your suit type (type 1 or 2): "
);
pub const FAILED_GET_PACK_SIZE_STR: &str = concatcp!(
    RED, BOLD, "ERROR:", RESET, "Failed to read your pack size."
);
pub const INVALID_PACK_SIZE_STR: &str = concatcp!(
    RED, BOLD, "ERROR:", RESET, "Invalid number. You can only enter '1' or '2'"
);
