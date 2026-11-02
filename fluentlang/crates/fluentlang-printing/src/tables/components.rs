use fluentlang_utils::error::AppErrors;

pub mod art;
pub mod chat;
pub mod status;
pub mod config_status;

pub trait TableComponent {
    /// The function is supposed to create component and returns `Self` for a chain call.
    fn into_component(self) -> Result<Self, AppErrors>
    where
        Self: Sized;

    /// The function is supposed to change component so it becomes a top component of a table.
    ///
    /// # Example
    /// ```rust,ignore
    /// let art = ArtComponent::new(...).get_component();
    /// let table_top = art.to_start_of_table();
    /// ```
    ///
    /// # Output
    /// ```text
    /// ╭───────────────────────────╮
    /// │        ArtComponent       │
    /// ├───────────────────────────┤
    /// │        OtherComponent     │
    /// ├───────────────────────────┤
    /// │        OtherComponent     │
    /// ╰───────────────────────────╯
    /// ```
    fn to_start_of_table(&self) -> Vec<String>;

    /// The function is supposed to change component so it becomes a middle component of a table.
    ///
    /// # Example
    /// ```rust,ignore
    /// let art = ArtComponent::new(...).get_component();
    /// let table_middle = art.to_middle_of_table();
    /// ```
    ///
    /// # Output
    /// ```text
    /// ╭───────────────────────────╮
    /// │        OtherComponent     │
    /// ├───────────────────────────┤
    /// │        ArtComponent       │
    /// ├───────────────────────────┤
    /// │        OtherComponent     │
    /// ╰───────────────────────────╯
    /// ```
    fn to_middle_of_table(&self) -> Vec<String>;

    /// The function is supposed to change component so it becomes a final component of a table.
    ///
    /// # Example
    /// ```rust,ignore
    /// let art = ArtComponent::new(...).get_component();
    /// let table_end = art.to_end_of_table();
    /// ```
    ///
    /// # Output
    /// ```text
    /// ╭───────────────────────────╮
    /// │        OtherComponent     │
    /// ├───────────────────────────┤
    /// │        OtherComponent     │
    /// ├───────────────────────────┤
    /// │        ArtComponent       │
    /// ╰───────────────────────────╯
    /// ```
    fn to_end_of_table(&self) -> Vec<String>;

    /// The function is supposed to change component so it becomes a single component of a table.
    ///
    /// # Example
    /// ```rust,ignore
    /// let art = ArtComponent::new(...).get_component();
    /// let table_end = art.to_solo_table();
    /// ```
    ///
    /// # Output
    /// ```text
    /// ╭───────────────────────────╮
    /// │        ArtComponent       │
    /// ╰───────────────────────────╯
    /// ```
    fn to_solo_table(&self) -> Vec<String>;
}

pub struct NecessaryDataTable {
    component: Vec<String>,
    terminal_inner_width: usize,
    padding: Paddings
}

#[derive(Clone)]
pub struct Paddings {
    pub left: usize,
    pub right: usize,
    pub top: usize,
    pub bottom: usize
}

impl Default for Paddings {
    fn default() -> Self {
        Self {
            left: 5,
            right: 5,
            top: 1,
            bottom: 1
        }
    }
}