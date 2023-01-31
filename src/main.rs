mod repo;

fn main() {

    // body of program here

    repo::check_version()
        .expect("check_version error");

}
