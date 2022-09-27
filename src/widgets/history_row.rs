/*  Copyright (C) 2020-2022 Patrick Csikos (https://zelikos.github.io)
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 * Authored by Patrick Csikos <zelikos@pm.me>
 */

use crate::deps::*;
use crate::i18n::*;

use std::cell::Cell;

use adw::subclass::prelude::*;
use adw::prelude::PreferencesRowExt;
use adw::prelude::ActionRowExt;
use glib::{BindingFlags, ParamSpec, ParamSpecUInt, Value};
use gtk::prelude::*;
use gtk::CompositeTemplate;
use once_cell::sync::Lazy;

mod imp {

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/gitlab/zelikos/rollit/gtk/history-row.ui")]
    pub struct RollitHistoryItem {
        pub result: Cell<u32>,
        pub max_val: Cell<u32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RollitHistoryItem {
        const NAME: &'static str = "RollitHistoryItem";
        type Type = super::RollitHistoryItem;
        type ParentType = adw::ActionRow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);

            klass.install_action("history.copy-result", None, move |history, _, _| {
                history.copy_result();
            });
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RollitHistoryItem {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecUInt::builder("result").build(),
                    ParamSpecUInt::builder("max-val").build(),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(
            &self,
            _obj: &Self::Type,
            _id: usize,
            value: &Value,
            pspec: &ParamSpec,
        ) {
            match pspec.name() {
                "result" => {
                    let input_num = 
                        value.get().expect("Value must be type 'u32'.");
                    self.result.replace(input_num);
                }
                "max-val" => {
                    let input_num = 
                        value.get().expect("Value must be type 'u32'.");
                    self.max_val.replace(input_num);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "result" => self.result.get().to_value(),
                "max-val" => self.max_val.get().to_value(),
                _ => unimplemented!(),
            }
        }

        fn constructed (&self, obj: &Self::Type) {
            self.parent_constructed(obj);
            obj.bind_property("result", obj, "title")
                .flags(BindingFlags::SYNC_CREATE)
                .build();
        }
    }

    impl WidgetImpl for RollitHistoryItem {}
    impl ListBoxRowImpl for RollitHistoryItem {}
    impl PreferencesRowImpl for RollitHistoryItem {}
    impl ActionRowImpl for RollitHistoryItem {}
}

glib::wrapper! {
    pub struct RollitHistoryItem(ObjectSubclass<imp::RollitHistoryItem>)
        @extends gtk::Widget, gtk::ListBoxRow, adw::PreferencesRow, adw::ActionRow,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl RollitHistoryItem {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create RollitHistoryItem")
    }

    pub fn from_result(result: u32, max: u32) -> Self {
        let obj: RollitHistoryItem = glib::Object::new(&[
            ("result", &result.to_value()),
            ("max-val", &max.to_value())])
            .expect("Failed to create RollitHistoryItem");
        
        obj.set_subtitle_label(max);
        obj
    }

    // pub fn add_result (&self, result: u32, max: u32) {
    //     self.set_title(&result.to_string());
    //     self.set_subtitle(&(i18n("Out of ") + &max.to_string()));
    // }

    fn get_result(&self) -> u32 {
        self.imp().result.get()
    }

    fn get_max_val(&self) -> u32 {
        self.imp().max_val.get()
    }

    fn set_subtitle_label(&self, max: u32) {
        // let max_num = self.imp().max_val.get();

        log::debug!("Setting subtitle to {}", max);
        self.set_subtitle(&(i18n(&format!("Out of {max}"))));
    }

    fn copy_result (&self) {
        let clipboard = self.clipboard();
        clipboard.set_text(&self.title());

        self.activate_action("win.show-toast", Some(&(i18n("Result copied"), 0).to_variant())).unwrap();
    }
}

