use crate::translate;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let text = String::from("Привет, мир!");
        let result = translate::from(text, scheme::core::new());
        assert_eq!(result, "Privėt, mir!");
    }
}
