use crate::types::process;
use inquire::{validator::StringValidator, Confirm, Select, Text};

pub fn numeric(msg: &str) -> i32 {
    let number_validator: StringValidator = &|input| {
        if input.len() == 0 {
            return Err(String::from("Introduce un valor"));
        }
        match input.parse::<i32>() {
            Ok(val) => {
                if val > 0 {
                    Ok(())
                } else {
                    Err(String::from("Debe ser mayor a 0"))
                }
            }
            Err(_) => Err(String::from("Introduce un número válido")),
        }
    };
    let number = Text::new(msg)
        .with_validator(number_validator)
        .prompt()
        .unwrap_or(String::from("0"));
    number.parse::<i32>().unwrap_or(0)
}

pub fn float(msg: &str) -> f64 {
    let number_validator: StringValidator = &|input| {
        if input.len() == 0 {
            return Err(String::from("Introduce un valor"));
        }
        match input.parse::<f64>() {
            Ok(_) => Ok(()),
            Err(_) => Err(String::from("Introduce un número entero válido")),
        }
    };
    let number = Text::new(msg)
        .with_validator(number_validator)
        .prompt()
        .unwrap_or(String::from("0"));
    number.parse::<f64>().unwrap_or(0.0)
}

pub fn text(msg: &str) -> String {
    let text_validator: StringValidator = &|input| {
        if input.len() == 0 {
            return Err(String::from("Introduce un valor"));
        }
        Ok(())
    };
    Text::new(msg)
        .with_validator(text_validator)
        .prompt()
        .unwrap_or(String::from(""))
}

pub fn unique(msg: &str, list: &[&str]) -> String {
    let text_validator: StringValidator = &|input| {
        if input.len() == 0 {
            return Err(String::from("Introduce un valor"));
        }
        match input.chars().find(|&c| !c.is_numeric()) {
            Some(_) => return Err(String::from("Solo puede contener núemros")),
            None => (),
        };
        match list.iter().find(|&&x| x.eq(input)) {
            Some(_) => Err(String::from("El identificados ya está en uso")),
            None => Ok(()),
        }
    };
    Text::new(msg)
        .with_validator(text_validator)
        .prompt()
        .unwrap_or(String::from(""))
}

pub fn operator() -> process::Operators {
    let options: Vec<&str> = vec!["+ ", "-", "*", "/", "%", "^"];
    let ans = Select::new("Operación:", options).prompt().unwrap_or("+");
    if ans.eq("-") {
        return process::Operators::Resta;
    }
    if ans.eq("*") {
        return process::Operators::Mult;
    }
    if ans.eq("/") {
        return process::Operators::Div;
    }
    if ans.eq("%") {
        return process::Operators::Mod;
    }
    if ans.eq("^") {
        return process::Operators::Pow;
    }
    process::Operators::Suma
}

pub fn confirm() -> bool {
    let ans = Confirm::new("¿Comenzar a ejecutar los procesos? ")
        .with_default(false)
        .prompt();
    match ans {
        Ok(val) => val,
        Err(_) => false,
    }
}
