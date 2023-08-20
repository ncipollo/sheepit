pub fn branch_ref_name(branch_name: &str) -> String {
    format!("refs/heads/{branch_name}")
}

pub fn tag_ref_name(tag: &str) -> String {
    format!("refs/tags/{tag}")
}

#[cfg(test)]
mod test {
    use crate::repo::reference;

    #[test]
    fn branch_ref_name() {
        let expected = "refs/heads/my_branch";
        let reference = reference::branch_ref_name("my_branch");
        assert_eq!(expected, reference)
    }

    #[test]
    fn tag_ref_name() {
        let expected = "refs/tags/1.0.0";
        let reference = reference::tag_ref_name("1.0.0");
        assert_eq!(expected, reference)
    }
}