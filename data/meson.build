desktop_file = i18n.merge_file(
  input: configure_file(
    input: 'com.gitlab.zelikos.rollit.desktop.in.in',
    output: 'com.gitlab.zelikos.rollit.desktop.in',
    configuration: conf,
  ),
  output: '@0@.desktop'.format(app_id),
  type: 'desktop',
  po_dir: '../po',
  install: true,
  install_dir: iv_datadir / 'applications'
)

desktop_utils = find_program('desktop-file-validate', required: false)
if desktop_utils.found()
  test('Validate desktop file', desktop_utils,
    args: [desktop_file]
  )
endif

appstream_file = i18n.merge_file(
  input: configure_file(
    input: 'com.gitlab.zelikos.rollit.appdata.xml.in.in',
    output: 'com.gitlab.zelikos.rollit.appdata.xml.in',
    configuration: conf,
  ),
  output: '@0@.appdata.xml'.format(app_id),
  po_dir: '../po',
  install: true,
  install_dir: iv_datadir / 'appdata'
)

# appstream_util = find_program('appstream-util', required: false)
# if appstream_util.found()
#   test('Validate appstream file', appstream_util,
#     args: ['validate', appstream_file]
#   )
# endif

install_data('com.gitlab.zelikos.rollit.gschema.xml.in',
  install_dir: iv_datadir / 'glib-2.0' / 'schemas'
)

compile_schemas = find_program('glib-compile-schemas', required: false)
if compile_schemas.found()
  test('Validate schema file', compile_schemas,
    args: ['--strict', '--dry-run', meson.current_source_dir()]
  )
endif

gnome.compile_resources(
  'rollit',
  'rollit.gresource.xml',
  gresource_bundle: true,
  install: true,
  install_dir: iv_pkgdatadir,
)

subdir('icons')
