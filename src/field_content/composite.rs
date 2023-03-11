use frame_metadata::{PalletMetadata, RuntimeMetadata, RuntimeMetadataV14};
use scale_info::{
    form::PortableForm, Type, TypeDef, TypeDefArray, TypeDefBitSequence, TypeDefCompact,
    TypeDefComposite, TypeDefPrimitive, TypeDefSequence, TypeDefTuple, TypeDefVariant, Variant,
};

use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlSelectElement, WebSocket};
use yew::prelude::*;
use yew::{
    events,
    platform::{
        pinned::mpsc::UnboundedSender,
        spawn_local,
        time::{interval, sleep},
    },
    AttrValue, Callback,
};

use crate::field_content::{Field, FieldContent};
use crate::messaging::{CallConstructorEvent, CCEI};

#[derive(Debug)]
pub struct Composite {
    name: Option<String>,
    fields: Vec<FieldContent>,
}

impl Composite {
    pub fn resolve(
        input: &TypeDefComposite<PortableForm>,
        name: Option<&str>,
        metadata: &RuntimeMetadataV14,
    ) -> Self {
        let name = match name {
            Some(a) => Some(a.to_string()),
            None => None,
        };
        Composite {
            name: name,
            fields: input
                .fields()
                .iter()
                .map(|a| {
                    FieldContent::new(
                        metadata.types.resolve(a.ty().id()),
                        a.name().map(|x| &**x),
                        metadata,
                    )
                })
                .collect(),
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|x| &**x)
    }
}

impl Field for Composite {
    fn handle_event(&mut self, _ccei: CCEI, _metadata: &RuntimeMetadataV14) -> bool {
        false
    }

    fn get_children(&self) -> &[FieldContent] {
        &self.fields
    }


    fn render_self(
        &self,
        parent: Vec<usize>,
        callback_original: Callback<CallConstructorEvent>,
        metadata: &RuntimeMetadataV14,
    ) -> Html {
        match self.name() {
                Some(name) => html! {<p>{format!("{}", name)}</p>},
                None => html! {},
            }
    }
}