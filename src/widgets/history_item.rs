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
use crate::window::RollitWindow;

use adw::subclass::prelude::*;
use adw::prelude::PreferencesRowExt;
use adw::prelude::ActionRowExt;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;

mod imp {

    use super::*;

    #[derive(Debug, Default, CompositeTemplate)]
    #[template(resource = "/com/gitlab/zelikos/rollit/gtk/history-item.ui")]
    pub struct RollitHistoryItem {
        #[template_child]
        pub(super) item_row: TemplateChild<adw::ActionRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RollitHistoryItem {
        const NAME: &'static str = "RollitHistoryItem";
        type Type = super::RollitHistoryItem;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for RollitHistoryItem {
        fn constructed (&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }

    impl WidgetImpl for RollitHistoryItem {}
    impl BinImpl for RollitHistoryItem {}
}

glib::wrapper! {
    pub struct RollitHistoryItem(ObjectSubclass<imp::RollitHistoryItem>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Buildable;
}

impl RollitHistoryItem {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create RollitHistoryItem")
    }

    pub fn add_result (&self, result: u16, max: u16) {
        let imp = self.imp();
        imp.item_row.set_title(&result.to_string());
        imp.item_row.set_subtitle(&(i18n("Out of ") + &max.to_string()));
    }

    pub fn connect_to (win: RollitWindow) {
        // TODO: Copy result to clipboard, add toast
    }
}

