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

namespace Rollit {
    [GtkTemplate (ui = "/com/gitlab/zelikos/rollit/window.ui")]
    public class Window : Gtk.ApplicationWindow {
        [GtkChild] private unowned Gtk.Label label;

        public Window (Adw.Application app) {
            Object (application: app);
        }
    }
}


    // private Rollit.Menu menu_button;
    // private Rollit.NumDisplay number_display;
    // private Rollit.RollHistory roll_history;
    // private Adw.HeaderBar header;
    // private Gtk.Button roll_button;
    // private Gtk.Button history_button;
    // private Gtk.Box action_buttons;
    // private Gtk.Grid main_view;
    // private Gtk.Paned hp;
    // private bool history_visible = true;


    // construct {
    //     Adw.init ();

        // restore_state ();

    //     header = new Adw.HeaderBar (); //{
        //     title = "Roll-It",
            // show_title_buttons = true
        // };

    //     history_button = new Gtk.Button.from_icon_name ("document-open-recent-symbolic") { //, Gtk.IconSize.MENU) {
    //         tooltip_text = (_("Roll history")),
            // tooltip_markup = Granite.markup_accel_tooltip ({"<Ctrl>H"}, _("Roll history"))
    //         tooltip_markup = ("Ctrl+H")
    //     };

    //     header.pack_end (history_button);

    //     number_display = new Rollit.NumDisplay ();

    //     roll_button = new Gtk.Button.with_label (_("Roll"));
        // roll_button.tooltip_markup = Granite.markup_accel_tooltip ({"<Ctrl>R"}, roll_button.label);
    //     roll_button.tooltip_text = roll_button.label;
    //     roll_button.tooltip_markup = ("Ctrl+R");
        // roll_button.get_style_context ().add_class (Gtk.STYLE_CLASS_SUGGESTED_ACTION);

    //     menu_button = new Rollit.Menu ();

    //     action_buttons = new Gtk.Box (Gtk.Orientation.HORIZONTAL, 6) {
            // layout_style = Gtk.ButtonBoxStyle.CENTER
    //         homogeneous = true
    //     };

    //     action_buttons.append (roll_button);
    //     action_buttons.append (menu_button);

    //     main_view = new Gtk.Grid () {
    //         row_spacing = 12,
    //         margin_top = margin_bottom = margin_start = margin_end = 12,
    //         hexpand = true,
    //         vexpand = true
    //     };
    //     main_view.attach (number_display, 0, 0);
    //     main_view.attach (action_buttons, 0, 1);

    //     roll_history = new Rollit.RollHistory ();

    //     hp = new Gtk.Paned (HORIZONTAL) {
    //         shrink_start_child = false,
    //         shrink_end_child = false
    //     };
    //     hp.set_start_child (main_view); //, true, false);
    //     hp.set_end_child (roll_history); //, false, false);

    //     var grid = new Gtk.Grid ();
    //     grid.attach (header, 0, 0, 2);
    //     grid.attach (hp, 0, 1);

    //     set_child (grid);

    //     roll_history.visible = history_visible;

    //     roll_button.clicked.connect (e => {
    //         roll_history.add_roll (number_display.num_gen (menu_button.max_roll));
    //     });

    //     history_button.clicked.connect (e => {
    //         roll_history.visible = !roll_history.visible;
    //         Application.settings.set_boolean ("show-history", roll_history.visible);
    //     });

    // }
