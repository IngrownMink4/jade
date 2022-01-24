use crate::internal::exec::*;
use crate::internal::*;

pub fn new_user(username: &str, hasroot: bool, password: &str) {
    exec_eval(
        exec_chroot(
            "useradd",
            vec![
                String::from("-m"),
                String::from("-s"),
                String::from("/bin/bash"),
                String::from(username),
            ],
        ),
        format!("Create user {}", username).as_str(),
    );
    if hasroot {
        exec_eval(
            exec_chroot(
                "usermod",
                vec![
                    String::from("-a"),
                    String::from("-G"),
                    String::from("wheel"),
                    String::from(username),
                ],
            ),
            format!("Add user {} to wheel group", username).as_str(),
        );
        files_eval(
            files::append_file("/mnt/etc/sudoers", "%wheel ALL=(ALL) ALL"),
            "Add wheel group to sudoers",
        );
        files_eval(
            files::append_file("/mnt/etc/sudoers", "Defaults pwfeedback"),
            "Add pwfeedback to sudoers",
        );
    }
    exec_eval(
        exec_chroot(
            "bash",
            vec![
                String::from("-c"),
                format!(
                    r#"'usermod --password $(echo {} | openssl passwd -1 -stdin) {}'"#,
                    password, username
                ),
            ],
        ),
        format!("Set password for user {}", username).as_str(),
    );
}

pub fn root_pass(root_pass: &str) {
    exec_eval(
        exec_chroot(
            "bash",
            vec![
                String::from("-c"),
                format!(
                    r#"'usermod --password $(echo {} | openssl passwd -1 -stdin) root'"#,
                    root_pass
                ),
            ],
        ),
        "set root password",
    );
}
