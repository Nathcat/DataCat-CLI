use std::process::exit;

pub fn no_auth_client_setup(host_url: String) {
    eprintln!(
        "No auth client setup for {}!\nConsider setting one up with 'clicat auth-client -u {}'",
        host_url, host_url
    );
    exit(-1);
}

pub fn no_auth_grant_setup(host_url: String) {
    eprintln!(
        "Not logged in on {}!\nConsider logging in with 'clicat login -u {}'",
        host_url, host_url
    );
    exit(-1);
}

pub fn unauthorized(host_url: String) {
    eprintln!(
        "{} returned code 401 Unauthorized, you may need to login again.",
        host_url
    );
    exit(-1);
}
