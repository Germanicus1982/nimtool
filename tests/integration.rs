#[cfg(test)]
mod integration {
    use std::process::Command;

    #[test]
    fn no_args() {
        let noargs: &'static str = "";
        let output = Command::new("./target/debug/nimtool")
            .output()
            .expect("");

        assert_eq!(noargs, String::from_utf8_lossy(&output.stdout));
    }

    //
    // Flags
    //

    #[test]
    fn help_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("--help")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn version_short_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("-V")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn version_long_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("--version")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn network_stats_short_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("-n")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn network_stats_long_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("--network-stats")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn supply_short_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("-s")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn supply_long_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("--supply")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    //
    // Options
    //

    #[test]
    fn addressbook_short_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("-a")
            .arg(r#""NQ63 RHCJ XVXE 3TX5 V318 SGUD 75NQ SP4G E9YQ""#)
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn addressbook_long_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("--addressbook")
            .arg(r#""NQ63 RHCJ XVXE 3TX5 V318 SGUD 75NQ SP4G E9YQ""#)
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn blockinfo_short_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("-b")
            .arg("65593")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn blockinfo_long_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("--blockinfo")
            .arg("65593")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn currency_short_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("-c")
            .arg("usd")
            .output()
            .expect("Test failed");

        assert!(!output.status.success());
    }

    #[test]
    fn currency_long_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("--currency")
            .arg("usd")
            .output()
            .expect("Test failed");

        assert!(!output.status.success());
    }

    #[test]
    fn hashrate_current_short_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("-h")
            .arg("current")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn hashrate_current_long_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("--hashrate")
            .arg("current")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn hashrate_hour_short_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("-h")
            .arg("hour")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn hashrate_hour_long_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("--hashrate")
            .arg("hour")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn hashrate_day_short_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("-h")
            .arg("day")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn hashrate_day_long_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("--hashrate")
            .arg("day")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn hashrate_week_short_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("-h")
            .arg("week")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn hashrate_week_long_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("--hashrate")
            .arg("week")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn hashrate_month_short_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("-h")
            .arg("month")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn hashrate_month_long_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("--hashrate")
            .arg("month")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn hashrate_year_short_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("-h")
            .arg("year")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn hashrate_year_long_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("--hashrate")
            .arg("year")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn label_short_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("-l")
            .arg("nimtool.com")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn label_long_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("--label")
            .arg("nimtool.com")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn transaction_short_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("-t")
            .arg("71d100206d2c4a0e202c08b84e0f4d3f4a61142e5368bbf29c8475218fb1b4c4")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn transaction_long_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("--transaction")
            .arg("71d100206d2c4a0e202c08b84e0f4d3f4a61142e5368bbf29c8475218fb1b4c4")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn price_current_short_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("-p")
            .arg("current")
            .arg("-c")
            .arg("usd")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn price_current_long_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("--price")
            .arg("current")
            .arg("--currency")
            .arg("usd")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn price_day_short_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("-p")
            .arg("day")
            .arg("-c")
            .arg("usd")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn price_day_long_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("--price")
            .arg("day")
            .arg("--currency")
            .arg("usd")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn price_week_short_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("-p")
            .arg("week")
            .arg("-c")
            .arg("usd")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn price_week_long_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("--price")
            .arg("week")
            .arg("--currency")
            .arg("usd")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn price_month_short_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("-p")
            .arg("month")
            .arg("-c")
            .arg("usd")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn price_month_long_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("--price")
            .arg("month")
            .arg("--currency")
            .arg("usd")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn price_year_short_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("-p")
            .arg("year")
            .arg("-c")
            .arg("usd")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }

    #[test]
    fn price_year_long_arg() {
        let output = Command::new("./target/debug/nimtool")
            .arg("--price")
            .arg("year")
            .arg("--currency")
            .arg("usd")
            .output()
            .expect("Test failed");

        assert!(output.status.success());
    }
}