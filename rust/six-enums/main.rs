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
}