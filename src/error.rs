pub struct Error {
    pub cause: String
}

impl std::fmt::Display for Error {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        write!(f, "{}", self.cause)

    }

}

impl From<reqwest::Error> for Error {

    fn from(err: reqwest::Error) -> Self {

        Self {
            cause: format!("{}", err)
        }

    }

}

impl From<std::io::Error> for Error {

    fn from(err: std::io::Error) -> Self {

        Self {
            cause: format!("{}", err)
        }

    }

}
