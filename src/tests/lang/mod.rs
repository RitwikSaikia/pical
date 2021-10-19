macro_rules! derive_mock_accessor {
    ($name:ident, $type:ident) => {
        impl Test {
            #[allow(non_snake_case)]
            fn $name(&self) -> &str {
                match &self.$name {
                    $type::__Mock(x) => x.as_str(),
                    _ => panic!()
                }
            }
        }
    };

    ($name:ident, $type:ident, $lifetime:lifetime) => {
        impl<$lifetime> Test<$lifetime> {
            #[allow(non_snake_case)]
            fn $name(&self) -> &str {
                match &self.$name {
                    $type::__Mock(x) => x.as_str(),
                    _ => panic!()
                }
            }
        }
    };
}


mod agent_spec;
mod prefix_spec;
mod if_spec;
mod action_spec;
mod name_spec;
mod ch_spec;