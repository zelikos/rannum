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
 * Authored by Patrick Csikos <akzeldev@fastmail.com>
 */

public class Rollit.Window : Hdy.Window {

    private uint configure_id;

    public Window (Application app) {
        Object (
            application: app
        );
    }

    construct {
        Hdy.init ();

        default_width = 260;
        default_height = 260;
        //resizable = false;

        int window_x, window_y;
        Application.settings.get ("window-position", "(ii)", out window_x, out window_y);

        
        if (window_x != -1 || window_y != -1) {
            move (window_x, window_y);
        }

        /*
        var header = new Gtk.HeaderBar () {
            title = "Roll-It",
            show_close_button = true,
            decoration_layout = "close:"
        };
        header.get_style_context ().add_class ("default-decoration");
        */

        var header = new Hdy.HeaderBar () {
            title = "Roll-It",
            show_close_button = true
        };

        var style_switch = new Granite.ModeSwitch.from_icon_name (
            "display-brightness-symbolic",
            "weather-clear-night-symbolic"
        ) {
            primary_icon_tooltip_text = _("Light"),
            secondary_icon_tooltip_text = _("Dark"),
            valign = Gtk.Align.CENTER
        };
        
        var gtk_settings = Gtk.Settings.get_default ();
        style_switch.bind_property ("active", gtk_settings, "gtk_application_prefer_dark_theme");
        Application.settings.bind ("dark-style", style_switch, "active", SettingsBindFlags.DEFAULT);

        header.pack_end (style_switch);
        //set_titlebar (header);


        var number_display = new Rollit.NumDisplay ();

        var roll_button = new Gtk.Button.with_label (_("Roll"));
        roll_button.get_style_context ().add_class (Gtk.STYLE_CLASS_SUGGESTED_ACTION);

        var menu_button = new Gtk.MenuButton () {
            image = new Gtk.Image.from_icon_name ("open-menu-symbolic", Gtk.IconSize.MENU),
            tooltip_text = _("Dice Settings")
        };
        menu_button.get_style_context ().add_class (Gtk.STYLE_CLASS_SUGGESTED_ACTION);

        var menu_popover = new Gtk.Popover (menu_button);
        menu_button.popover = menu_popover;
        var menu_grid = new Rollit.Menu ();
        menu_popover.add (menu_grid);

        var action_buttons = new Gtk.Grid ();
        action_buttons.add (roll_button);
        action_buttons.add (menu_button);
        action_buttons.get_style_context ().add_class (Gtk.STYLE_CLASS_LINKED);
        
        var btn_box = new Gtk.ButtonBox (Gtk.Orientation.HORIZONTAL);
        btn_box.spacing = 6;
        
        btn_box.add (action_buttons);

        var main_view = new Gtk.Grid ();
        //main_view.margin = 12;
        main_view.attach (header, 0, 0);
        main_view.attach (number_display, 0, 1);
        main_view.attach (btn_box, 0, 2);

        add (main_view);

        roll_button.clicked.connect (e => {
            int max_roll = Application.settings.get_int ("max-roll");
            number_display.num_gen (max_roll);
        });

        show_all ();
    }

    public override bool configure_event (Gdk.EventConfigure event) {
        if (configure_id != 0) {
            GLib.Source.remove (configure_id);
        }

        configure_id = Timeout.add (100, () => {
            configure_id = 0;

            int root_x, root_y;
            get_position (out root_x, out root_y);
            Application.settings.set ("window-position", "(ii)", root_x, root_y);

            return false;
            }
        );

        return base.configure_event (event);
    }
}
