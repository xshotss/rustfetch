pub mod info;
pub mod modules;


pub const DEFAULT_LUA_CONFIG: &str = r#"
-- This is an automatically generating config file for Rustfetch.
-- Check the Github repo for help:
-- https://github.com/xshotss/rustfetch

-- All ASCII art files should be placed in ~/.config/rustfetch/ascii/
ascii_art = "tux.txt"

-- For configuring your own modules, go to the repository for help.
default_modules = true

module_mode = "fancy"
"#;


/// A built in const containing ASCII art of the Linux mascot, Tux.
pub const TUX_ASCII_ART: &str = r#"
                .88888888:.
               88888888.88888.
             .8888888888888888.
             888888888888888888
             88' _`88'_  `88888
             88 88 88 88  88888
             88_88_::_88_:88888
             88:::,::,:::::8888
             88`:::::::::'`8888
            .88  `::::'    8:88.
           8888            `8:888.
         .8888'             `888888.
        .8888:..  .::.  ...:'8888888:.
       .8888.'     :'     `'::`88:88888
      .8888        '         `.888:8888.
     888:8         .           888:88888
   .888:88        .:           888:88888:
   8888888.       ::           88:888888
   `.::.888.      ::          .88888888
  .::::::.888.    ::         :::`8888'.:.
 ::::::::::.888   '         .::::::::::::
 ::::::::::::.8    '      .:8::::::::::::.
.::::::::::::::.        .:888:::::::::::::
:::::::::::::::88:.__..:88888:::::::::::'
 `'.:::::::::::88888888888.88:::::::::'
       `':::_:' -- '' -'-' `':_::::'`
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
