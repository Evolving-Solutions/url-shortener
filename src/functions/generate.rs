// Copyright (c) 2022 Evolving Software Corporation
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT
use random_string::generate;

////////////////////////////////////////////////////////////////////////////////
/// # Name: generate_url_code                                                ///
/// # Description: Generates a random url code.                              ///
/// # Arguments:                                                             ///
/// # Returns:                                                               ///
/// # Privacy: Public                                                        ///
////////////////////////////////////////////////////////////////////////////////
// Gernerate a random series of 6 alphanumeric characters.
pub fn generate_url_code() -> String {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    generate(6, charset)
}
