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
use crate::utils;

use core::ops::Deref;

use adw::subclass::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::CompositeTemplate;

use random_number::random;
use std::time::Duration;

mod imp {
    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/gitlab/zelikos/rollit/gtk/main-view.ui")]
    pub struct RollitMainView {
        #[template_child]
        pub(super) max_roll: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub(super) result_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) result_revealer: TemplateChild<gtk::Revealer>,
        #[template_child]
        pub(super) result_stack: TemplateChild<gtk::Stack>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RollitMainView {
        const NAME: &'static str = "RollitMainView";
        type Type = super::RollitMainView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RollitMainView {
        fn constructed (&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            let settings = utils::settings_manager();
            settings.bind("max-roll", self.max_roll.deref(), "value").build();

        }
    }

    impl WidgetImpl for RollitMainView {}
    impl BinImpl for RollitMainView {}
}

glib::wrapper! {
    pub struct RollitMainView(ObjectSubclass<imp::RollitMainView>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Buildable;
}

impl RollitMainView {
    pub fn get_roll_result (&self) -> u16 {
        const MIN_NUM: u16 = 1;
        let max_num: u16 = self.get_max_roll();
        let rnd_num: u16 = random!(MIN_NUM, max_num);

        self.set_result_label(rnd_num.clone());

        rnd_num
    }

    pub fn hide_label (&self) {
        self.imp().result_stack.set_visible_child(&self.imp().result_stack.child_by_name("empty").unwrap());
    }

    pub fn show_label (&self) {
        self.imp().result_stack.set_visible_child(&self.imp().result_stack.child_by_name("result").unwrap());
    }

    fn get_max_roll (&self) -> u16 {
        self.imp().max_roll.value_as_int() as u16
    }

    fn set_result_label (&self, result: u16) {
        let imp = self.imp();
        let transition_dur = Duration::from_millis(imp.result_revealer.transition_duration().into());

        if imp.result_stack.visible_child_name().unwrap() != "result" {
            imp.result_stack.set_visible_child(&imp.result_stack.child_by_name("result").unwrap());
            imp.result_label.set_label(&result.to_string());
            imp.result_revealer.set_reveal_child(true);
        } else {
            imp.result_revealer.set_reveal_child(false);
            glib::timeout_add_local (transition_dur,
                clone!(@weak self as this => @default-return Continue(false), move || {
                    this.imp().result_label.set_label(&result.to_string());
                    this.imp().result_revealer.set_reveal_child(true);
                    Continue(false)
                })
            );
        }
    }
}

