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
    private Rollit.RollHistory roll_history;
    private Hdy.HeaderBar header;
    private Gtk.Button roll_button;
    private Gtk.Button history_button;
    private Gtk.Box action_buttons;
    private Gtk.Grid main_view;
    private Gtk.Paned hp;

    private bool history_visible;
    private uint configure_id;

    public MainWindow (Rollit.Application app) {
        Object (
            application: app,
            title: "Roll-It"
        );
    }

    construct {
        Hdy.init ();

        restore_state ();

        header = new Hdy.HeaderBar () {
            title = "Roll-It",
            show_close_button = true
        };

        history_button = new Gtk.Button.from_icon_name ("document-open-recent-symbolic", Gtk.IconSize.MENU) {
            tooltip_markup = Granite.markup_accel_tooltip ({"<Ctrl>H"}, _("Roll History"))
        };

        header.pack_end (history_button);

        number_display = new Rollit.NumDisplay ();

        roll_button = new Gtk.Button.with_label (_("Roll"));
        roll_button.tooltip_markup = Granite.markup_accel_tooltip ({"<Ctrl>R"}, roll_button.label);
        roll_button.get_style_context ().add_class (Gtk.STYLE_CLASS_SUGGESTED_ACTION);

        menu_button = new Rollit.Menu ();

        action_buttons = new Gtk.ButtonBox (Gtk.Orientation.HORIZONTAL) {
            layout_style = Gtk.ButtonBoxStyle.CENTER
        };

        action_buttons.add (roll_button);
        action_buttons.add (menu_button);

        main_view = new Gtk.Grid () {
            row_spacing = 12,
            margin = 12
        };
        main_view.attach (number_display, 0, 0);
        main_view.attach (action_buttons, 0, 1);

        roll_history = new Rollit.RollHistory ();

        hp = new Gtk.Paned (HORIZONTAL);
        hp.pack1 (main_view, true, false);
        hp.pack2 (roll_history, false, false);

        var grid = new Gtk.Grid ();
        grid.attach (header, 0, 0);
        grid.attach (hp, 0, 1);

        add (grid);

        show_all ();

        roll_history.visible = history_visible;

        roll_button.clicked.connect (e => {
            roll_history.add_roll (number_display.num_gen (menu_button.max_roll));
        });

        history_button.clicked.connect (e => {
            roll_history.visible = !roll_history.visible;
            Application.settings.set_boolean ("show-history", roll_history.visible);
        });

        var accel_group = new Gtk.AccelGroup ();

        accel_group.connect (
            Gdk.Key.@1,
            Gdk.ModifierType.CONTROL_MASK,
            Gtk.AccelFlags.VISIBLE | Gtk.AccelFlags.LOCKED,
            () => {
                menu_button.shortcut_pressed (1);
                return true;
            }
        );

        accel_group.connect (
            Gdk.Key.@2,
            Gdk.ModifierType.CONTROL_MASK,
            Gtk.AccelFlags.VISIBLE | Gtk.AccelFlags.LOCKED,
            () => {
                menu_button.shortcut_pressed (2);
                return true;
            }
        );

        accel_group.connect (
            Gdk.Key.@3,
            Gdk.ModifierType.CONTROL_MASK,
            Gtk.AccelFlags.VISIBLE | Gtk.AccelFlags.LOCKED,
            () => {
                menu_button.shortcut_pressed (3);
                return true;
            }
        );

        accel_group.connect (
            Gdk.Key.@4,
            Gdk.ModifierType.CONTROL_MASK,
            Gtk.AccelFlags.VISIBLE | Gtk.AccelFlags.LOCKED,
            () => {
                menu_button.shortcut_pressed (4);
                return true;
            }
        );

        accel_group.connect (
            Gdk.Key.R,
            Gdk.ModifierType.CONTROL_MASK,
            Gtk.AccelFlags.VISIBLE | Gtk.AccelFlags.LOCKED,
            () => {
                roll_button.clicked ();
                menu_button.close_menu ();
                return true;
            }
        );

        accel_group.connect (
            Gdk.Key.D,
            Gdk.ModifierType.CONTROL_MASK,
            Gtk.AccelFlags.VISIBLE | Gtk.AccelFlags.LOCKED,
            () => {
                menu_button.clicked ();
                return true;
            }
        );

        accel_group.connect (
            Gdk.Key.H,
            Gdk.ModifierType.CONTROL_MASK,
            Gtk.AccelFlags.VISIBLE | Gtk.AccelFlags.LOCKED,
            () => {
                history_button.clicked ();
                return true;
            }
        );

        accel_group.connect (
            Gdk.Key.C,
            Gdk.ModifierType.MOD1_MASK, // TODO: Replace with ALT_MASK for GTK4
            Gtk.AccelFlags.VISIBLE | Gtk.AccelFlags.LOCKED,
            () => {
                roll_history.clear_button.clicked ();
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

        history_visible = Application.settings.get_boolean ("show-history");
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
