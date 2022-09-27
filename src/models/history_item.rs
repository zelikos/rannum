/*  Copyright (C) 2022 Patrick Csikos (https://zelikos.github.io)
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
use std::cell::RefCell;
use std::rc::Rc;

use glib::{Object, ParamSpec, ParamSpecUInt, Value};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct RollitHistoryItem {
        pub data: Rc<RefCell<RollitHistoryData>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RollitHistoryItem {
        const NAME: &'static str = "RollitHistoryItem";
        type Type = super::RollitHistoryItem;
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
                    self.data.borrow_mut().result = input_num;
                }
                "max-val" => {
                    let input_num =
                        value.get().expect("Value must be type 'u32'.");
                    self.data.borrow_mut().max_val = input_num;
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "result" => self.data.borrow().result.to_value(),
                "max-val" => self.data.borrow().max_val.to_value(),
                _ => unimplemented!(),
            }
        }

        fn constructed (&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }
}

glib::wrapper! {
    pub struct RollitHistoryItem(ObjectSubclass<imp::RollitHistoryItem>);
}

impl RollitHistoryItem {
    #[allow(clippy::new_without_default)]
    pub fn new(roll_result: u32, max: u32) -> Self {
        glib::Object::new(&[
            ("result", &roll_result),
            ("max-val", &max)])
            .expect("Failed to create RollitHistoryItem")
    }
}

#[derive(Debug, Default)]
pub struct RollitHistoryData {
    pub result: u32,
    pub max_val: u32,
}
