use crate::{passfile::PassFile, passgen::PassGenerator, passline::Passline, utils};
use std::{io, process};

pub struct AddOptions {
    pub name: String,
    pub len: usize,
    pub no_upper: bool,
    pub no_lower: bool,
    pub no_numbers: bool,
    pub no_special_chars: bool,
}

pub struct AddHandler;

impl AddHandler {
    pub fn handle(options: AddOptions) -> io::Result<()> {
        let pass_file = PassFile::new();
        let update_pass_line = Self::is_update(&pass_file, &options.name)?;
        let gen = Self::init_pass_generator(&options);

        let mut password;
        loop {
            password = gen.gen_password();

            println!("");
            println!("Pass: {}", password);

            if !utils::want_to_re_generate() {
                break;
            }
        }

        let pass_line = Passline::new(options.name, password);

        if !update_pass_line {
            pass_file.write_pass(&pass_line)?;
        } else {
            pass_file.update_pass(&pass_line)?;
        }

        pass_line.print();

        Ok(())
    }

    fn init_pass_generator(options: &AddOptions) -> PassGenerator {
        let mut gen = PassGenerator::new(options.len);
        gen.use_lower(!options.no_lower)
            .use_upper(!options.no_upper)
            .use_numbers(!options.no_numbers)
            .use_special_chars(!options.no_special_chars);

        gen
    }

    fn is_update(pass_file: &PassFile, name: &String) -> io::Result<bool> {
        let mut update_pass_line = false;

        if pass_file.pass_line_exist(name)? {
            if !utils::want_to_update(name) {
                process::exit(0);
            }

            update_pass_line = true;
        }

        Ok(update_pass_line)
    }
}
