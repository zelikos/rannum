project('rollit', ['c', 'vala'],
          version: '3.0.0',
    meson_version: '>= 0.59.0',
  default_options: [ 'warning_level=2',
                     'werror=false',
                   ],
)

i18n = import('i18n')
gnome = import('gnome')

rollit_deps = [
  dependency('gtk4', version: '>=4.0.0'),
  dependency('libadwaita-1', version: '>= 1.2'),
]

iv_prefix = get_option('prefix')
iv_bindir = iv_prefix / get_option('bindir')
iv_localedir = iv_prefix / get_option('localedir')
iv_datadir = iv_prefix / get_option('datadir')
iv_pkgdatadir = iv_datadir / meson.project_name()

app_id = 'com.gitlab.zelikos.rollit'
iv_version = meson.project_version()

# conf = configuration_data()
# conf.set('APP_ID', app_id)
# conf.set('PROFILE', profile)
# conf.set('VERSION', iv_version)
# conf.set('bindir', iv_bindir)
# conf.set('localedir', iv_localedir)
# conf.set('pkgdatadir', iv_pkgdatadir)

subdir('data')
subdir('src')
subdir('po')

executable(
  app_id,
  rollit_sources,
  rollit_resources,
  c_args: [
    '-DGETTEXT_PACKAGE="@0@"'.format(app_id),
  ],
  vala_args: '--target-glib=2.50',
  dependencies: rollit_deps,
  install: true,
)

gnome.post_install(
  glib_compile_schemas: true,
  gtk_update_icon_cache: true,
  update_desktop_database: true,
)
