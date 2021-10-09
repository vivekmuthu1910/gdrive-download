macro_rules! item_url {
    ($id:expr) => {
        format!("https://drive.google.com/open?id={}", $id)
    };
}

macro_rules! file_url {
    ($id:expr, $confirm:expr) => {
        format!(
            "https://docs.google.com/uc?export=download&id={}&confirm={}",
            $id, $confirm
        )
    };
}

macro_rules! folder_url {
    ($id:expr) => {
        format!(
            "https://drive.google.com/embeddedfolderview?id={}#list",
            $id
        )
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn check_item_url() {
        assert_eq!(&item_url!("123"), "https://drive.google.com/open?id=123");
    }
    #[test]
    fn check_file_url() {
        assert_eq!(
            &file_url!("123", "true"),
            "https://docs.google.com/uc?export=download&id=123&confirm=true"
        );
    }
    #[test]
    fn check_folder_url() {
        assert_eq!(
            &folder_url!("123"),
            "https://drive.google.com/embeddedfolderview?id=123#list"
        );
    }
}
