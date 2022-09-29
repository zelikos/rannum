project('rollit', 'rust',
          version: '3.0.0',
    meson_version: '>= 0.59.0',
  default_options: [ 'warning_level=2',
                   ],
)

i18n = import('i18n')
gnome = import('gnome')

dependency('gtk4', version: '>=4.0.0')
dependency('libadwaita-1', version: '>= 1.2')

cargo = find_program('cargo', required: true)

iv_prefix = get_option('prefix')
iv_bindir = iv_prefix / get_option('bindir')
iv_localedir = iv_prefix / get_option('localedir')
iv_datadir = iv_prefix / get_option('datadir')
iv_pkgdatadir = iv_datadir / meson.project_name()

profile = ''
version_suffix = ''

if get_option('profile') == 'development'
  profile = '.Devel'
  vcs_tag = run_command('git', 'rev-parse', '--short', 'HEAD').stdout().strip()
  if vcs_tag == ''
    version_suffix = '-devel'
  else
    version_suffix = '-@0@'.format(vcs_tag)
  endif
endif

app_id = 'dev.zelikos.rollit@0@'.format(profile)
iv_version = meson.project_version() + version_suffix

cargo_sources = files(
  'Cargo.toml',
  'Cargo.lock',
)

conf = configuration_data()
conf.set('APP_ID', app_id)
conf.set('PROFILE', profile)
conf.set('VERSION', iv_version)
conf.set('bindir', iv_bindir)
conf.set('localedir', iv_localedir)
conf.set('pkgdatadir', iv_pkgdatadir)

subdir('data')
subdir('src')
subdir('po')

gnome.post_install(
  glib_compile_schemas: true,
  gtk_update_icon_cache: true,
  update_desktop_database: true,
)

