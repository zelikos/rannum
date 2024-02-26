/*  Copyright (C) 2022-2023 Patrick Csikos (https://zelikos.dev)
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
 * Authored by Patrick Csikos <pcsikos@zelikos.dev>
 */

use std::cell::RefCell;
use std::rc::Rc;

use glib::{ParamSpec, ParamSpecUInt, Value};
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct RollitTrayItem {
        pub data: Rc<RefCell<RollitTrayData>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RollitTrayItem {
        const NAME: &'static str = "RollitTrayItem";
        type Type = super::RollitTrayItem;
    }

    impl ObjectImpl for RollitTrayItem {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> =
                Lazy::new(|| vec![ParamSpecUInt::builder("dice-value").build()]);
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "dice-value" => {
                    let input_num = value.get().expect("Value must be type 'u32'.");
                    self.data.borrow_mut().dice_value = input_num;
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "dice-value" => self.data.borrow().dice_value.to_value(),
                _ => unimplemented!(),
            }
        }

        fn constructed(&self) {
            self.parent_constructed();
        }
    }
}

glib::wrapper! {
    pub struct RollitTrayItem(ObjectSubclass<imp::RollitTrayItem>);
}

impl RollitTrayItem {
    #[allow(clippy::new_without_default)]
    pub fn new(dice_value: u32) -> Self {
        glib::Object::builder()
            .property("dice-value", dice_value)
            .build()
    }
}

#[derive(Debug, Default)]
pub struct RollitTrayData {
    pub dice_value: u32,
}
