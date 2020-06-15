mod args;
mod file;
mod req;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::args();
    let image = file::load_image(args.file)?;
    let client = reqwest::blocking::Client::new();
    let resp = req::send_request(&client, image, args.lang)?;
    report(resp);

    Ok(())
}

fn report(resp: req::Response) {
    if resp.error {
        print(resp.error_message);
        print(resp.error_details)
    } else {
        for res in resp.parsed_results {
            print(res.text);
        }
    }
}

fn print(s: Option<String>) {
    if let Some(s) = s {
        println!("{}", s)
    }
}