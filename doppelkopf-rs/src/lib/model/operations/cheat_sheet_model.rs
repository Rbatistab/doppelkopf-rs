/// Model operations for `cheat_sheet_logic`

use crate::model::types::cheat_sheet::CheatSheetOption;

/// Model input for `cheat_sheet_logic` from the dppkf_lib
///
/// # Fields
/// * `cheat_sheet` - Option of the desired cheat sheet
pub struct CheatSheetInput {
    pub cheat_sheet: CheatSheetOption
}

impl CheatSheetInput {
    /// Contains the arguments to ask for a cheat sheet
    ///
    /// # Fields
    /// * `cheat_sheet` - Option of the desired cheat sheet
    ///
    /// # Returns CheatSheetInput instance
    ///
    /// # Example
    /// ```
    /// use dppkf_lib::model::operations::cheat_sheet_model::CheatSheetInput;
    /// use dppkf_lib::model::types::cheat_sheet::CheatSheetOption;
    ///
    /// let cheat_sheet_input = CheatSheetInput::from(CheatSheetOption::Rules);
    /// assert_eq!(cheat_sheet_input.cheat_sheet, CheatSheetOption::Rules);
    /// ```
    pub fn from(cheat_sheet: CheatSheetOption) -> CheatSheetInput {
        CheatSheetInput { cheat_sheet }
    }
}

impl Default for CheatSheetInput {
    /// Contains the arguments to ask for a cheat sheet that defaults to GameOverview
    ///
    /// # Returns CheatSheetInput instance with GameOverview value for a cheat
    ///
    /// # Example
    /// ```
    /// use dppkf_lib::model::operations::cheat_sheet_model::CheatSheetInput;
    /// use dppkf_lib::model::types::cheat_sheet::CheatSheetOption;
    ///
    /// let cheat_sheet_input = CheatSheetInput::default();
    /// assert_eq!(cheat_sheet_input.cheat_sheet, CheatSheetOption::GameOverview);
    /// ```
    fn default() -> Self {
        CheatSheetInput { cheat_sheet: CheatSheetOption::GameOverview }
    }
}

/// Model output for `cheat_sheet_logic` from the dppkf_lib
///
/// # Fields
/// * `cheat` - Desired cheat sheet
pub struct CheatSheetOutput {
    pub cheat: String
}

impl CheatSheetOutput {
    /// CheatSheet cheat value
    ///
    /// # Returns CheatSheetInput instance with GameOverview value for a cheat
    ///
    /// # Example
    /// ```
    /// use dppkf_lib::model::operations::cheat_sheet_model::CheatSheetOutput;
    ///
    /// let cheat = "mock_cheat".to_string();
    /// let cheat_sheet_output = CheatSheetOutput::from(cheat.clone());
    /// assert_eq!(cheat_sheet_output.cheat, cheat);
    /// ```
    pub fn from(cheat: String) -> CheatSheetOutput {
        CheatSheetOutput { cheat }
    }
}