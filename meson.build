project('com.github.zelikos.rannum', 'vala', 'c')

subdir('src')
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