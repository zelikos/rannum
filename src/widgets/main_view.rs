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

use crate::utils;

use core::ops::Deref;

use adw::subclass::prelude::*;
use glib::clone;
use gtk::glib;
use gtk::prelude::*;

use random_number::random;
use std::time::Duration;

mod imp {
    use super::*;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/dev/zelikos/rollit/gtk/main-view.ui")]
    pub struct RollitMainView {
        #[template_child]
        pub(super) max_roll: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub(super) result_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) result_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub(super) result_revealer: TemplateChild<gtk::Revealer>,
        #[template_child]
        pub(super) result_stack: TemplateChild<gtk::Stack>,
    }

    impl Default for RollitMainView {
        fn default() -> Self {
            Self {
                max_roll: TemplateChild::default(),
                result_label: TemplateChild::default(),
                result_button: TemplateChild::default(),
                result_revealer: TemplateChild::default(),
                result_stack: TemplateChild::default(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RollitMainView {
        const NAME: &'static str = "RollitMainView";
        type Type = super::RollitMainView;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RollitMainView {
        fn constructed(&self) {
            self.parent_constructed();

            // TODO: Move to separate method
            let settings = utils::settings_manager();
            settings
                .bind("max-roll", self.max_roll.deref(), "value")
                .build();
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
    pub fn get_roll_result(&self) -> u32 {
        const MIN_NUM: u32 = 1;
        let max_num: u32 = self.get_max_roll();
        let rnd_num: u32 = random!(MIN_NUM, max_num);

        self.set_result_label(rnd_num);

        rnd_num
    }

    pub fn hide_label(&self) {
        self.imp()
            .result_stack
            .set_visible_child(&self.imp().result_stack.child_by_name("empty").unwrap());
    }

    pub fn show_label(&self) {
        self.imp()
            .result_stack
            .set_visible_child(&self.imp().result_stack.child_by_name("result").unwrap());
    }

    pub fn get_last_result(&self) -> String {
        self.imp().result_label.label().to_string()
    }

    fn get_max_roll(&self) -> u32 {
        self.imp().max_roll.value_as_int() as u32
    }

    fn set_result_label(&self, result: u32) {
        let imp = self.imp();
        let transition_dur =
            Duration::from_millis(imp.result_revealer.transition_duration().into());

        if imp.result_stack.visible_child_name().unwrap() != "result" {
            imp.result_stack
                .set_visible_child(&imp.result_stack.child_by_name("result").unwrap());
            imp.result_label.set_label(&result.to_string());
            imp.result_revealer.set_reveal_child(true);
        } else {
            imp.result_revealer.set_reveal_child(false);
            glib::timeout_add_local(
                transition_dur,
                clone!(@weak self as this => @default-return Continue(false), move || {
                    this.imp().result_label.set_label(&result.to_string());
                    this.imp().result_revealer.set_reveal_child(true);
                    Continue(false)
                }),
            );
        }
    }
}
