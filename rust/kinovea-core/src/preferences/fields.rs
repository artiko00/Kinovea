//! Macros generating the element mapping of a struct from an ordered field list.

/// Generates `write_fields`/`read_fields` over the listed fields, in the given order.
macro_rules! xml_fields {
    ($ty:ty { $($field:ident => $name:literal),* $(,)? }) => {
        impl $ty {
            pub(crate) fn write_fields(&self, e: &mut $crate::preferences::dom::Element) {
                use $crate::preferences::helpers::XmlField;
                $( if let Some(c) = self.$field.write($name) { e.children.push(c); } )*
            }
            pub(crate) fn read_fields(&mut self, e: &$crate::preferences::dom::Element) {
                use $crate::preferences::helpers::XmlField;
                for c in &e.children {
                    match c.name.as_str() {
                        $( $name => self.$field.read(c), )*
                        _ => {}
                    }
                }
            }
        }
    };
}
pub(crate) use xml_fields;

/// Generates a nested element type: `to_element`/`from_element` built on the field list.
macro_rules! xml_struct {
    ($ty:ty { $($field:ident => $name:literal),* $(,)? }) => {
        $crate::preferences::fields::xml_fields!($ty { $($field => $name),* });
        impl $crate::preferences::helpers::XmlField for $ty {
            fn write(&self, name: &str) -> Option<$crate::preferences::dom::Element> {
                let mut e = $crate::preferences::dom::Element::new(name);
                self.write_fields(&mut e);
                Some(e)
            }
            fn read(&mut self, element: &$crate::preferences::dom::Element) {
                self.read_fields(element);
            }
        }
    };
}
pub(crate) use xml_struct;
