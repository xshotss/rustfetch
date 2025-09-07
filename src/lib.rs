pub mod info;
pub mod modules;

pub const DEFAULT_LUA_CONFIG: &str = r#"
-- This is an automatically generated config file for Rustfetch.
-- Check the Github repo for help:
-- https://github.com/xshotss/rustfetch

-- All ASCII art files should be placed in ~/.config/rustfetch/ascii/
ascii = "tux.txt"

mode = "fancy"

modules = {
  cpu = {
    type = "builtin"
  },
  gpu = {
    type = "builtin"
  },
  hostname = {
    type = "builtin"
  }
  memory = {
    type = "builtin"
  }
}
"#;

/// A built in const containing ASCII art of the Linux mascot, Tux.
pub const TUX_ASCII_ART: &str = r#"
         _nnnn_
        dGGGGMMb
       @p~qp~~qMb
       M|@||@) M|
       @,----.JM|
      JS^\__/  qKL
     dZP        qKRb
    dZP          qKKb
   fZP            SMMb
   HZM            MMMM
   FqM            MMMM
 __| ".        |\dS"qML
 |    `.       | `' \Zq
_)      \.___.,|     .'
\____   )MMMMMP|   .'
     `-'       `--'
"#;

#[cfg(test)]
mod librs_tests {
    use colored::Colorize;

    use crate::TUX_ASCII_ART;

    #[test]
    fn tux_ascii_art_out() {
        std::fs::write("tests/tux.txt", TUX_ASCII_ART).unwrap();
        println!("{}", TUX_ASCII_ART.bright_blue());
    }
}
