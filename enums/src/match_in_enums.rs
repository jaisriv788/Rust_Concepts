#[derive(Debug)]
enum OS {
    Windows,
    MacOS,
    Linux,
    Ubuntu,
    Kali,
}

#[derive(Debug)]
enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String, bool),
}


//methods in enums
impl LaundryCycle {
    fn wash(&self) {
        match self {
            LaundryCycle::Cold => {
                println!("Cold");
            }
            LaundryCycle::Hot { temperature } => {
                println!("Temperature right now {} degree celcius.", temperature);
            }
            LaundryCycle::Delicate(fabric, handle_with_care) => {
                println!(
                    "It is {} that the cloth is delicate because its made of {}",
                    handle_with_care, fabric
                );
            }
        }
    }
}

fn main() {
    //type 1 and 2
    let my_os_age = years_since_release(OS::Linux);
    println!("{:?}", my_os_age);

    wash(LaundryCycle::Hot { temperature: 42 });

    wash(LaundryCycle::Delicate(String::from("Silk"), true));

    //implementing methods of enum
    LaundryCycle::Hot{temperature: 33}.wash();
}

//match type 1 and type 2 is the windows option we can write like that also
fn years_since_release(os: OS) -> u32 {
    match os {
        OS::Windows => {
            println!("Windows");
            22
        }
        OS::MacOS => 23,
        OS::Linux => 24,
        OS::Ubuntu => 25,
        OS::Kali => 26,
    }
}

//Type 3 where parameters are being passed on to the cases as struct or tuple
fn wash(wash_type: LaundryCycle) {
    match wash_type {
        LaundryCycle::Cold => {
            println!("Cold");
        }
        LaundryCycle::Hot { temperature } => {
            println!("Running on hot {}", temperature);
        }
        LaundryCycle::Delicate(fabric, handle_with_care) => {
            println!(
                "It is {} that the cloth is delicate because its made of {}",
                handle_with_care, fabric
            );
        }
    }
}
