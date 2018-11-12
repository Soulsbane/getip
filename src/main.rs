#[macro_use]
extern crate clap;
use clap::{App, Arg};

extern crate dns_lookup;
use dns_lookup::lookup_host;

fn main() {
	let matches = App::new(crate_name!())
		.version(crate_version!())
		.author(crate_authors!("\n"))
		.about(crate_description!())
		.arg(
			Arg::with_name("URL")
				.required(true)
				.takes_value(true)
				.index(1)
				.help("URL of the website to get the IP address for."),
		)
		.get_matches();

	let url = matches.value_of("URL").unwrap();
	let ips: Vec<std::net::IpAddr> = lookup_host(url).unwrap();

	if !ips.is_empty() {
		for x in ips.iter() {
			println!("{}", x);
		}
	} else {
		println!("{}{}", "Unable to retrieve the IP Address for ", url);
	}
}
