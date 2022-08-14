mod gh_repo_status;

fn main() {

    // body of program here

    gh_repo_status::check_version()
        .expect("check_version error");

}
