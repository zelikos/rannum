rollit_sources = [
  'application.vala',
  'main.vala',
  'window.vala',
  # 'Widgets/Menu.vala',
  # 'Widgets/NumDisplay.vala',
  # 'Widgets/RollHistory.vala',
  # 'Widgets/Buttons/MenuItem.vala',
  # 'Widgets/Buttons/PreviousRoll.vala',
]

rollit_deps = [
  dependency('libadwaita-1', version: '>= 1.0'),
]

gnome = import('gnome')

rollit_sources += gnome.compile_resources('rollit-resources',
  'rollit.gresource.xml',
  c_name: 'rollit'
)

executable('com.gitlab.zelikos.rollit', rollit_sources,
  c_args: [
    '-DGETTEXT_PACKAGE="com.gitlab.zelikos.rollit"'
  ],
  vala_args: '--target-glib=2.50', dependencies: rollit_deps,
  install: true,
)
