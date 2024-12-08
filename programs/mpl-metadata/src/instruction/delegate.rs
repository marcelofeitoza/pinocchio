use core::fmt;

use alloc::string::ToString;

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum HolderDelegateRole {
    PrintDelegate,
}

impl fmt::Display for HolderDelegateRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Self::PrintDelegate => "print_delegate".to_string(),
        };

        write!(f, "{message}")
    }
}

#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum MetadataDelegateRole {
    AuthorityItem,
    Collection,
    Use,
    Data,
    ProgrammableConfig,
    DataItem,
    CollectionItem,
    ProgrammableConfigItem,
}

impl fmt::Display for MetadataDelegateRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Self::AuthorityItem => "authority_item_delegate".to_string(),
            Self::Collection => "collection_delegate".to_string(),
            Self::Use => "use_delegate".to_string(),
            Self::Data => "data_delegate".to_string(),
            Self::ProgrammableConfig => "programmable_config_delegate".to_string(),
            Self::DataItem => "data_item_delegate".to_string(),
            Self::CollectionItem => "collection_item_delegate".to_string(),
            Self::ProgrammableConfigItem => "prog_config_item_delegate".to_string(),
        };

        write!(f, "{message}")
    }
}
