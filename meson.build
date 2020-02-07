project('com.github.zelikos.rannum', 'vala', 'c')

gnome = import('gnome')

asresources = gnome.compile_resources (
    'as-resources', join_paths ('data', 'gresource.xml'),
    source_dir: 'data',
    c_name: 'as'
)

executable(
    meson.project_name(),
    'src/Main.vala',
    'src/Application.vala',
    'src/Widgets/Window.vala',
    'src/Widgets/NumDisplay.vala',
    asresources,
    dependencies: [
        dependency('gtk+-3.0')
    ],

    install: true
)

subdir('data')

install_data(
    join_paths('data', meson.project_name() + '.desktop'),
    install_dir: join_paths(get_option('datadir'), 'applications')
)

install_data(
    join_paths('data', meson.project_name() + '.appdata.xml'),
    install_dir: join_paths(get_option('datadir'), 'metainfo')
)

meson.add_install_script('meson/post_install.py')