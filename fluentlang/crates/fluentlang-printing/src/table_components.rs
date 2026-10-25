use fluentlang_core::error::AppErrors;

pub mod art_component;
pub mod chat_component;
pub mod status_component;
pub mod config_status_component;

pub trait TableComponent {
    /// The function is supposed to create component and returns `Self` for a chain call.
    fn get_component(self) -> Result<Self, AppErrors>
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