use std::process::{Command, Stdio}

// cerner_2tothe5th_2022
// running kubectl in a rust binary to get deployments for namespace defined in context (it is possible to add more flags like the namespace but it causes it to go over 32 lines if you want more processing)

fn main() {
    let directory = std::env::current_dir().unwrap();
    let kubectl_output_child = Command::new('kubectl)
        .arg("get")
        .arg("deployments")
        .stdout(Stdio::piped())
        .spawn().expect("kubectl failed");
        // This can be done on one line, but it is more readable this way

        let kubectl_output = kubectl_output_child.wait_with_output().unwrap();

        println!(
            "K8s Deployments:\n{}",
            String::from_utf8(kubectl_output::stdout).unwrap()
        )
}
