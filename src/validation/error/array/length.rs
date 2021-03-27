use crate::validation::error::ToDefaultMessage;

#[derive(Debug)]
pub struct ItemsErrorParams {
    items: Vec<String>,
    items_length: usize,
    min_items: Option<usize>,
    max_items: Option<usize>,
}

impl ItemsErrorParams {
    pub fn new<T>(items: &[T], min_items: Option<usize>, max_items: Option<usize>) -> Self
    where
        T: std::fmt::Debug,
    {
        Self {
            items: items.iter().map(|i| format!("{:?}", i)).collect(),
            items_length: items.len(),
            min_items,
            max_items,
        }
    }

    #[allow(dead_code)]
    pub fn items(&self) -> &Vec<String> {
        &self.items
    }

    #[allow(dead_code)]
    pub fn items_length(&self) -> usize {
        self.items_length
    }

    #[allow(dead_code)]
    pub fn min_items(&self) -> Option<usize> {
        self.min_items
    }

    #[allow(dead_code)]
    pub fn max_items(&self) -> Option<usize> {
        self.max_items
    }
}

impl ToDefaultMessage for ItemsErrorParams {
    fn to_default_message(&self) -> String {
        let min_items = match &self.min_items {
            Some(items) => format!("{} <= ", items),
            None => String::new(),
        };
        let max_items = match &self.max_items {
            Some(items) => format!(" <= {}", items),
            None => String::new(),
        };
        format!(
            "items length of [{}] must be in `{}length{}`, but `{}`.",
            &self.items.join(", "),
            min_items,
            max_items,
            self.items_length
        )
    }
}