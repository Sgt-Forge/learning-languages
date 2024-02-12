fn main() {
    {
        println!("==================================================");
        println!("Chapter 6: enums");
        println!("==================================================");

        #[derive(Debug)]
        enum IpAddrKind {
            V4,
            V6,
        }
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        println!("V4 = {:#?}, V6 = {:#?}", four, six);
        println!();
    }
    {
        println!("==================================================");
        println!("Chapter 6: enums and storing data");
        println!("==================================================");

        #[derive(Debug)]
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }
        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));

        println!("home = {:#?}, loopback = {:#?}", home, loopback);
    }

    {
        println!("==================================================");
        println!("Chapter 6: match control flow");
        println!("==================================================");
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
            //--snip--
        }
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }

        let alabama_quarter = Coin::Quarter(UsState::Alabama);
        match alabama_quarter {
            Coin::Penny => {
                println!("A lucky penny!");
                println!("1")
            },
            Coin::Nickel => println!("5"),
            Coin::Dime => println!("10"),
            Coin::Quarter(state) => println!("Your have a quarter from {:?}", state),
        }
    }

    {
        println!("==================================================");
        println!("Chapter 6: if let");
        println!("==================================================");

        let config_max = Some(3u8);
        match config_max {
            Some(max) => println!("the maximum is configured to be {}", max),
            _ => (),
        }

        let config_max2 = Some(10u8);
        match config_max2 {
            Some(max) => println!("the maximum is configured to be {}", max),
            _ => (),
        }

        let config_max3 = Some(20u8);
        if let Some(max) = config_max3 {
            println!("The maximum is configured to be {}", max)
        }

        let config_max4: Option<u8> = None;
        if let Some(max) = config_max4 {
            println!("The maximum is configured to be {}", max);
        } else {
            println!("literally anything else");
        }
    }
}