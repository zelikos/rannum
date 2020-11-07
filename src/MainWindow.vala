/*  Copyright (C) 2020 Patrick Csikos (https://zelikos.github.io)
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

public class Rollit.MainWindow : Hdy.Window {

    private Rollit.Menu menu_button;
    private Rollit.NumDisplay number_display;
    private Hdy.HeaderBar header;
    private Gtk.Button roll_button;
    private Gtk.Box action_buttons;
    private Gtk.Grid main_view;

    private uint configure_id;

    public MainWindow (Rollit.Application app) {
        Object (
            application: app
        );
    }

    construct {
        Hdy.init ();

        restore_state ();

        header = new Hdy.HeaderBar () {
            title = "Roll-It",
            show_close_button = true
        };

        number_display = new Rollit.NumDisplay () {
            margin_top = 12
        };

        roll_button = new Gtk.Button.with_label (_("Roll"));
        roll_button.tooltip_markup = Granite.markup_accel_tooltip ({"<Ctrl>R"}, roll_button.label);
        roll_button.get_style_context ().add_class (Gtk.STYLE_CLASS_SUGGESTED_ACTION);

        menu_button = new Rollit.Menu ();

        action_buttons = new Gtk.Box (Gtk.Orientation.HORIZONTAL, 0) {
            halign = Gtk.Align.CENTER,
            margin = 12
        };

        action_buttons.add (roll_button);
        action_buttons.add (menu_button);
        action_buttons.get_style_context ().add_class (Gtk.STYLE_CLASS_LINKED);

        main_view = new Gtk.Grid ();
        main_view.attach (header, 0, 0);
        main_view.attach (number_display, 0, 1);
        main_view.attach (action_buttons, 0, 2);

        add (main_view);

        show_all ();

        roll_button.clicked.connect (e => {
            number_display.num_gen (menu_button.max_roll);
        });

        var accel_group = new Gtk.AccelGroup ();

        accel_group.connect (
            Gdk.Key.@1,
            Gdk.ModifierType.CONTROL_MASK,
            Gtk.AccelFlags.VISIBLE | Gtk.AccelFlags.LOCKED,
            () => {
                menu_button.shortcut_pressed(1);
                return true;
            }
        );

        accel_group.connect (
            Gdk.Key.@2,
            Gdk.ModifierType.CONTROL_MASK,
            Gtk.AccelFlags.VISIBLE | Gtk.AccelFlags.LOCKED,
            () => {
                menu_button.shortcut_pressed(2);
                return true;
            }
        );

        accel_group.connect (
            Gdk.Key.@3,
            Gdk.ModifierType.CONTROL_MASK,
            Gtk.AccelFlags.VISIBLE | Gtk.AccelFlags.LOCKED,
            () => {
                menu_button.shortcut_pressed(3);
                return true;
            }
        );

        accel_group.connect (
            Gdk.Key.R,
            Gdk.ModifierType.CONTROL_MASK,
            Gtk.AccelFlags.VISIBLE | Gtk.AccelFlags.LOCKED,
            () => {
                roll_button.clicked();
                return true;
            }
        );

        accel_group.connect (
            Gdk.Key.D,
            Gdk.ModifierType.CONTROL_MASK,
            Gtk.AccelFlags.VISIBLE | Gtk.AccelFlags.LOCKED,
            () => {
                menu_button.clicked();
                return true;
            }
        );

        add_accel_group (accel_group);
    }

    private void restore_state () {
        var rect = Gdk.Rectangle ();
        Application.settings.get ("window-size", "(ii)", out rect.width, out rect.height);

        default_width = rect.width;
        default_height = rect.height;

        int window_x, window_y;
        Application.settings.get ("window-position", "(ii)", out window_x, out window_y);

        if (window_x != -1 || window_y != -1) {
            move (window_x, window_y);
        }

        var window_maximized = Application.settings.get_boolean ("maximized");
        if (window_maximized) {
            maximize ();
        }
    }

    public override bool configure_event (Gdk.EventConfigure event) {
        if (configure_id != 0) {
            GLib.Source.remove (configure_id);
        }

        configure_id = Timeout.add (400, () => {
            configure_id = 0;

            if (is_maximized) {
                Application.settings.set_boolean ("maximized", true);
            } else {
                Application.settings.set_boolean ("maximized", false);

                var rect = Gdk.Rectangle ();
                get_size (out rect.width, out rect.height);
                Application.settings.set ("window-size", "(ii)", rect.width, rect.height);

                int root_x, root_y;
                get_position (out root_x, out root_y);
                Application.settings.set ("window-position", "(ii)", root_x, root_y);
            }
            return false;
        });

        return base.configure_event (event);
    }
}
