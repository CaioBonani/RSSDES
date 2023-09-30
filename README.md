# RSSDES

## Rust Simple Simplified Data Encryption Standard!

This repo contains a simple (as the name suggests!) implementation of the SDES algorithm, made in Rust.

### Usage

To build/run/test the code, follow these steps:

  1. Open the terminal
  2. Change to the project directory and make sure to be in the root of the project folder
  3. Now, type the following: ``` cargo run -- [option] [key] [text] ```
     
     * **[option]:** **E** - Encrypt, **D** - Decrypt;
     * **[key]:** a 10-bit key. e.g., __1010011101__;
     * **[text]:** a 8-bit text to be **{crypted, encrypted}**. e.g., __01010010__;
       
If You followed the instructions and provided valids arguments, the code gonna show the **{crypted, encrypted}** text as the output.
